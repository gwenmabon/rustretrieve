use std::collections::HashMap;
use std::time::SystemTime;
use std::fmt;
use crate::index_representation::EntyType;

#[derive(Clone, PartialEq)]
pub enum FeatureValue {
    Numeric(f32),
    Categorical(String),
    Timestamp(SystemTime),
}

impl fmt::Display for FeatureValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeatureValue::Numeric(val) => write!(f, "{}", val),
            FeatureValue::Categorical(val) => write!(f, "{}", val),
            FeatureValue::Timestamp(val) => write!(f, "{:?}", val), //SystemTime doesn't implement Display, only Debug
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct FeatureStore {
    pub user_fs: HashMap<String, HashMap<String, FeatureValue>>,
    pub item_fs: HashMap<String, HashMap<String, FeatureValue>>,
}    

impl FeatureStore {

    pub fn new(user_fs: HashMap<String, HashMap<String, FeatureValue>>, item_fs: HashMap<String, HashMap<String, FeatureValue>>,) -> Self{
        Self {user_fs, item_fs}
    }

    pub fn get(&self, entity_id: String, feature_names: Vec<String>, entity_type: EntyType) -> Result<HashMap<String, FeatureValue>, String> {
        let mut res = HashMap::<String, FeatureValue>::new();
        let map = match entity_type {
            EntyType::User => &self.user_fs,
            EntyType::Item => &self.item_fs,
        };
        match map.get(&entity_id) {
            Some(values) => {for name in feature_names{
                let fv = values.get(&name).ok_or(format!("Feature values {} not found", &name).to_string())?;
                res.insert(name.clone(), fv.clone());
            }
            Ok(res)},
            None => Err(format!("Entity ID {} not found", &entity_id)),
        }
        
    }
 
    pub fn upsert(&mut self,  batch: HashMap<String, HashMap<String, FeatureValue>>, entity_type: EntyType) {
        let map = match entity_type {
            EntyType::User => &mut self.user_fs,
            EntyType::Item => &mut self.item_fs,
        };
        for (id, mapping) in batch{
            let inner = map.entry(id).or_insert_with(HashMap::new);
            for (k, v) in mapping {
                inner.insert(k, v);
            }
        }
        
    } 
}

#[cfg(test)]
pub fn create_feature_store() -> FeatureStore {
    let user_fs = HashMap::from([
        ("user_1".to_string(), HashMap::from([
            ("ctr".to_string(), FeatureValue::Numeric(0.05)),
            ("last_visit".to_string(), FeatureValue::Timestamp(SystemTime::now())),
            ("segment".to_string(), FeatureValue::Categorical("premium".to_string())),
        ])),
        ("user_2".to_string(), HashMap::from([
            ("ctr".to_string(), FeatureValue::Numeric(0.12)),
            ("segment".to_string(), FeatureValue::Categorical("free".to_string())),
        ])),
    ]);

    let item_fs = HashMap::from([
        ("item_101".to_string(), HashMap::from([
            ("price".to_string(), FeatureValue::Numeric(29.99)),
            ("category".to_string(), FeatureValue::Categorical("electronics".to_string())),
            ("last_update".to_string(), FeatureValue::Timestamp(SystemTime::now())),
        ])),
        ("item_102".to_string(), HashMap::from([
            ("price".to_string(), FeatureValue::Numeric(9.99)),
            ("category".to_string(), FeatureValue::Categorical("books".to_string())),
        ])),
    ]);

    FeatureStore::new(user_fs, item_fs)
}

#[test]
fn test_get_features_for_existing_entity(){
    let fs = create_feature_store();
    let res = fs.get("user_1".to_string(), vec!["last_visit".to_string()], EntyType::User);
    assert_eq!(res.is_ok(), true)
}
#[test]
fn test_get_features_for_non_existing_entity(){
    let fs = create_feature_store();
    let res = fs.get("user_x".to_string(), vec!["last_visit".to_string()], EntyType::User);
    assert_eq!(res.is_ok(), false)
}
#[test]
fn test_get_features_for_existing_entity_no_feature(){
    let fs = create_feature_store();
    let res = fs.get("user_1".to_string(), vec!["first_visit".to_string()], EntyType::User);
    assert_eq!(res.is_ok(), false)
}
#[test]
fn test_upsert_new_features_on_existing_entity(){
    let mut fs = create_feature_store();
    let b = HashMap::from([("user_1".to_string(), HashMap::from([("spend".to_string(), FeatureValue::Numeric(234.23))]))]);
    fs.upsert(b, EntyType::User);
    let res = fs.get("user_1".to_string(), vec!["spend".to_string()], EntyType::User);
    assert_eq!(res.is_ok(), true)
}

#[test]
fn test_upsert_features_on_new_entity(){
    let mut fs = create_feature_store();
    let b = HashMap::from([("item_1".to_string(), HashMap::from([("price".to_string(), FeatureValue::Numeric(1.0))]))]);
    fs.upsert(b, EntyType::Item);
    let res = fs.get("item_1".to_string(), vec!["price".to_string()], EntyType::Item);
    assert_eq!(res.is_ok(), true)
}

#[test]
fn test_upsert_features_on_new_entities(){
    let mut fs = create_feature_store();
    let b = HashMap::from([("item_1".to_string(), HashMap::from([("price".to_string(), FeatureValue::Numeric(1.0))])),
    ("item_2".to_string(), HashMap::from([("price".to_string(), FeatureValue::Numeric(10.0))]))]);
    fs.upsert(b, EntyType::Item);
    let res1 = fs.get("item_1".to_string(), vec!["price".to_string()], EntyType::Item);
    assert_eq!(res1.is_ok(), true);
    let res2 = fs.get("item_2".to_string(), vec!["price".to_string()], EntyType::Item);
    assert_eq!(res2.is_ok(), true);
}

#[test]
fn test_upsert_features_on_existing_entities(){
    let mut fs = create_feature_store();
    let b = HashMap::from([
        ("item_101".to_string(), HashMap::from([
            ("price".to_string(), FeatureValue::Numeric(19.99)),
            ("category".to_string(), FeatureValue::Categorical("electronics".to_string())),
            ("last_update".to_string(), FeatureValue::Timestamp(SystemTime::now())),
        ])),
        ("item_102".to_string(), HashMap::from([
            ("price".to_string(), FeatureValue::Numeric(99.99)),
            ("category".to_string(), FeatureValue::Categorical("books".to_string())),
            ("last_update".to_string(), FeatureValue::Timestamp(SystemTime::now())),
        ])),
    ]);
    fs.upsert(b, EntyType::Item);
    let res1 = fs.get("item_101".to_string(), vec!["price".to_string(), "category".to_string(), "last_update".to_string(),], EntyType::Item).unwrap();
    let is_value = *res1.get("price").unwrap() == FeatureValue::Numeric(19.99) ;
    assert_eq!(is_value, true);
    let res2 = fs.get("item_102".to_string(), vec!["price".to_string(), "category".to_string(), "last_update".to_string(),], EntyType::Item).unwrap();
    let is_value2 = *res2.get("price").unwrap() == FeatureValue::Numeric(99.99) ;
    assert_eq!(is_value2, true);
}

#[test]
fn test_upsert_features_on_mixing_entities(){
    let mut fs = create_feature_store();
    let b = HashMap::from([
        ("item_101".to_string(), HashMap::from([
            ("price".to_string(), FeatureValue::Numeric(19.99)),
            ("category".to_string(), FeatureValue::Categorical("electronics".to_string())),
            ("last_update".to_string(), FeatureValue::Timestamp(SystemTime::now())),
        ])),
        ("item_2".to_string(), HashMap::from([
            ("price".to_string(), FeatureValue::Numeric(99.99)),
            ("category".to_string(), FeatureValue::Categorical("books".to_string())),
            ("last_update".to_string(), FeatureValue::Timestamp(SystemTime::now())),
        ])),
    ]);
    fs.upsert(b, EntyType::Item);
    let res1 = fs.get("item_101".to_string(), vec!["price".to_string(), "category".to_string(), "last_update".to_string(),], EntyType::Item).unwrap();
    let is_value = *res1.get("price").unwrap() == FeatureValue::Numeric(19.99) ;
    assert_eq!(is_value, true);
    let res2 = fs.get("item_2".to_string(), vec!["price".to_string(), "category".to_string(), "last_update".to_string(),], EntyType::Item).unwrap();
    let is_value2 = *res2.get("price").unwrap() == FeatureValue::Numeric(99.99) ;
    assert_eq!(is_value2, true);
}

#[test]
fn test_upsert_empty_batch(){
    let mut fs = create_feature_store();
    let copy = fs.clone();
    let b = HashMap::new();
    fs.upsert(b, EntyType::User);
    assert_eq!(copy==fs, true);
}
