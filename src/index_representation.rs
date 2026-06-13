use std::collections::HashMap;
use std::fs::read_to_string;
use crate::similarity::dot_product;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntyType {
    User = 0,
    Item = 1,
}

pub fn compute_similarity(vec_user:&[f32] , vec_item:&[f32] ) -> Result<f32, String> {
    let is_user_empty = vec_user.is_empty();
    let is_item_empty = vec_item.is_empty();
    let mut message = String::new();
    if is_user_empty {
        message +=  "User's embedding is empty";
    }
    if is_item_empty {
        message +=  "Item's embedding is empty";
    }
    if is_user_empty || is_item_empty {
        return Err(message);
    }

    if vec_user.len()!=vec_item.len(){
        return Err("Embeddings dont have the same size.".to_string());
    }
    Ok(dot_product(vec_user,vec_item))
} 

pub struct IndexRepresentation {
    pub user_mapping: HashMap<String, [f32; 128]>,
    pub item_mapping: HashMap<String, [f32; 128]>,
}    

impl IndexRepresentation {
    pub fn new(user_mapping: HashMap<String, [f32; 128]>, item_mapping: HashMap<String, [f32; 128]>,) -> Self{
        Self {user_mapping, item_mapping}
    }

    pub fn from_csv(users_path: &str, items_path: &str) -> Result<Self, String> {
        let raw_user: String = read_to_string(users_path).map_err(|e| e.to_string())?;
        let user = Self::to_hashmap(raw_user);

        let raw_item: String = read_to_string(items_path).map_err(|e| e.to_string())?;
        let item = Self::to_hashmap(raw_item);
        return Ok(Self::new(user,item))
    }   

    fn to_hashmap(raw: String) -> HashMap<String, [f32; 128]>{
        let mut map: HashMap<String, [f32; 128]> = HashMap::new();
        for line in raw.lines().skip(1) {
            let parts: Vec<&str> = line.split(',').collect();
            let parsed: Result<Vec<f32>, _> = parts[1..].iter().map(|p| p.parse::<f32>()).collect();
            if let Ok(vec) = parsed {
                if let Ok(arr) = <[f32; 128]>::try_from(vec) {
                    map.insert(parts[0].to_string(), arr);
                }
            }
      }
      map
    }
    
    pub fn get_user_embeddings(&self, user_id: String ) -> Result<[f32; 128], String>{
        match self.user_mapping.get(&user_id) {
            Some(embedding) => Ok(*embedding),
            None => Err(format!("User ID {} not found", user_id)),
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, id: String, embedding: [f32; 128], entity_type: EntyType) {
        if entity_type== EntyType::User{
            self.user_mapping.insert(id.clone(), embedding);
        }
        if entity_type== EntyType::Item{
            self.item_mapping.insert(id.clone(), embedding);
        }

    }

    pub fn get_item_embeddings(&self) -> &HashMap<String, [f32; 128]> {
        &self.item_mapping 
    }

}



#[cfg(test)]
pub fn create_index() -> IndexRepresentation{
    let user_hashmap = HashMap::from([
        ("1".to_string(), [0.0_f32; 128]),
        ("2".to_string(), [1.0_f32; 128]),
        ("3".to_string(), [2.0_f32; 128]),
    ]);

    let item_hashmap = HashMap::from([
        ("101".to_string(), [100.0_f32; 128]),
        ("102".to_string(), [101.0_f32; 128]),
        ("103".to_string(), [102.0_f32; 128]),
        ("104".to_string(), [103.0_f32; 128]),
    ]);
    let index = IndexRepresentation::new(user_hashmap, item_hashmap);
    return index
}

#[test]
fn test_no_id_in_index(){
    let index = create_index();
    let id = index.get_user_embeddings("non existent id".to_string());
    assert_eq!(id.is_ok(), false)
}
#[test]
fn test_id_in_index(){
    let index = create_index();
    let id = index.get_user_embeddings("1".to_string());
    assert_eq!(id.is_ok(), true)
}
#[test]
fn test_insert_new_embeddings(){
    let mut  index = create_index();
    index.insert("8".to_string(), [7.0_f32; 128], EntyType::User );
    let inserted = index.get_user_embeddings("8".to_string(),);
    assert_eq!(inserted.is_ok(), true)
}
#[test]
fn test_empty_vector(){
    let empty_vec: Vec<f32> = vec![];
    let vec_b = [0.0, 1.0];
    let sim = compute_similarity(&empty_vec, &vec_b);
    assert_eq!(sim.is_ok(), false)
}

#[test]
fn test_dimension_mismatch(){
    let vec_a = [0.0, 1.0, 0.0];
    let vec_b = [0.0, 1.0];
    let sim = compute_similarity(&vec_a, &vec_b);
    assert_eq!(sim.is_ok(), false)
}

#[test]
fn test_csv_path_not_exist(){
    let index = IndexRepresentation::from_csv("", "");
    assert_eq!(index.is_ok(), false)
}

