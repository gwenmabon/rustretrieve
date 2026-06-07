use std::collections::HashSet;
use csv::Reader;
use std::collections::HashMap;
use clap::Parser;
use rustretrieve::index_representation::IndexRepresentation;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap( long)]
    seed: String,

    #[clap(short, long, default_value_t = 1)]
    k: u8,
}

struct SpotifyTrack {
    track_id: String,
    track_name: String,
    artist_name: String,
    features: [f32; 8],
}

fn main() {
    let args = Args::parse();

    let (user_embedding, seed_ids) = load_user_embedding(&args.seed).unwrap();
    println!("User embedding built from {} seed tracks.", seed_ids.len());

    let catalog = load_catalog("data/spotify_features.csv", &seed_ids).unwrap();
    println!("Catalog loaded: {} tracks.", catalog.len());

    let metadata: HashMap<String, (String, String)> = catalog
        .iter()
        .map(|t| (t.track_id.clone(), (t.track_name.clone(), t.artist_name.clone())))
        .collect();

    let index = build_index(user_embedding, &catalog);

    let scores = rustretrieve::top_k_algo::top_k_retrieval(args.k.into(), "me".to_string(), &index).unwrap();

    println!("\nTop {} recommendations:", args.k);
    for (rank, (track_id, score)) in scores.iter().enumerate() {
        if let Some((name, artist)) = metadata.get(track_id) {
            println!("  {}. \"{}\" — {}  (score: {:.4})", rank + 1, name, artist, score);
        } else {
            println!("  {}. {}  (score: {:.4})", rank + 1, track_id, score);
        }
    }
}

fn load_user_embedding(path: &str) -> Result<([f32; 8], HashSet<String>), String>{
    let mut rdr = Reader::from_path(path).map_err(|e| e.to_string())?;
    let mut sums = [0.0_f32; 8];
    let mut track_id = HashSet::<String>::new();
    let mut count = 0;
    for result in rdr.records() {
        let record = result.map_err(|e| e.to_string())?;
        count +=1;
        track_id.insert(record[0].to_string());
        for i in 3..11{
            let val: f32 = record[i].parse::<f32>().map_err(|e| e.to_string())?;
            sums[i-3] += val
        }
    }
    sums[6]/=200.0;
    Ok((sums.map(|y| {y/count as f32}), track_id))
}


fn load_catalog(path: &str, exclude: &HashSet<String>) -> Result<Vec<SpotifyTrack>, String>{
    let mut rdr = Reader::from_path(path).map_err(|e| e.to_string())?;
    let mut catalog: Vec::<SpotifyTrack>  = Vec::<SpotifyTrack>::new();
    let mut seen = HashSet::<String>::new();
    let cols = [5, 6, 8, 9, 11, 14, 15, 17];

    for result in rdr.records() {
        let record = result.map_err(|e| e.to_string())?;
        let track_id: String = record[3].to_string();
        if exclude.contains(&track_id) || seen.contains(&track_id){
            continue;
        }
        
        let mut features = [0.0_f32; 8];
        for (j, &col) in cols.iter().enumerate() {
            features[j] = record[col].parse::<f32>().map_err(|e| e.to_string())?;
        }
        features[6] /= 200.0;
        let track = SpotifyTrack {
        track_id: track_id.clone(),
        track_name: record[2].to_string(),
        artist_name: record[1].to_string(),
        features: features,
        };
        catalog.push(track);
        seen.insert(track_id);
    }
    Ok(catalog)
}

fn pad_to_128(features: &[f32; 8]) -> [f32; 128]{
    let mut arr = [0.0_f32; 128];
    arr[..8].copy_from_slice(features);
    return arr
}

fn build_index(user_embedding: [f32; 8], catalog: &[SpotifyTrack]) -> IndexRepresentation{
    let user_mapping = HashMap::from([
        ("me".to_string(), pad_to_128(&user_embedding)),
    ]);

    let mut items_mapping = HashMap::new();
    for track in catalog{
        items_mapping.insert(track.track_id.clone(), pad_to_128(&track.features));
    }
    IndexRepresentation::new(user_mapping, items_mapping)
}