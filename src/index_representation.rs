use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntyType {
    User = 0,
    Item = 1,
}

pub struct IndexRepresentation {
    pub user_mapping: HashMap<String, [f32; 128]>,
    pub item_mapping: HashMap<String, [f32; 128]>,
}    

impl IndexRepresentation {
    pub fn new(user_mapping: HashMap<String, [f32; 128]>, item_mapping: HashMap<String, [f32; 128]>,) -> Self{
        Self {user_mapping, item_mapping}
    }

    pub fn get_user_embeddings(&self, user_id: String ) -> Result<[f32; 128], String>{
        match self.user_mapping.get(&user_id) {
            Some(embedding) => Ok(*embedding),
            None => Err(format!("User ID {} not found", user_id)),
        }
    }

    pub fn insert(&mut self, id: String, embedding: [f32; 128], entity_type: EntyType) {
        if entity_type== EntyType::User{
            self.user_mapping.insert(id.clone(), embedding);
        }
        if entity_type== EntyType::Item{
            self.item_mapping.insert(id.clone(), embedding);
        }

    }
}

#[cfg(test)]

fn create_index() -> IndexRepresentation{
    let user_hashmap = HashMap::from([
        ("1".to_string(), [0.0_f32; 128]),
        ("2".to_string(), [1.0_f32; 128]),
        ("3".to_string(), [2.0_f32; 128]),
    ]);

    let item_hashmap = HashMap::from([
        ("101".to_string(), [100.0_f32; 128]),
        ("102".to_string(), [101.0_f32; 128]),
        ("103".to_string(), [102.0_f32; 128]),
        ("104".to_string(), [102.0_f32; 128]),
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