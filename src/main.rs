use clap::Parser;
use rustretrieve::top_k_algo::top_k_retrieval;
use rustretrieve::index_representation::IndexRepresentation;


#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap( long)]
    user_id: String,

    #[clap(short, long, default_value_t = 1)]
    k: u8,

    #[clap(long)]
    users_path: String,

    #[clap(short, long)]
    items_path: String,
}


fn main() {
    let args = Args::parse();
    //Load the index
    let index = IndexRepresentation::from_csv(&args.users_path, &args.items_path).unwrap();
    //Run top-k
    let scores = top_k_retrieval(args.k.into(), args.user_id.clone(), &index).unwrap();
    //Print results
    println!("Top {} similar items for user {}:", args.k, &args.user_id);
    for (rank, (item_id, score)) in scores.iter().enumerate() {
        println!("  {}. {}  (score: {:.4})", rank + 1, item_id, score);
    }
}
