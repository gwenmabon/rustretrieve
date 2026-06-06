use std::cmp::{Ordering, Reverse};
use crate::index_representation::{IndexRepresentation, compute_similarity};
use std::collections::BinaryHeap;


#[derive(Debug, PartialEq, PartialOrd)]
struct Score(f32);

impl Eq for Score {}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering{
       return self.0.partial_cmp(&other.0).expect("A score shoud always be a not NaN value.")
    }
}

fn get_min_heap(k: usize, vec: Vec<(String, f32)> ) -> Vec<(String, f32)> {
let mut heap: BinaryHeap<(Reverse<Score>, String)> = BinaryHeap::new();
for (item, score) in vec {
    if heap.len() < k { 
        heap.push((Reverse(Score(score)), item))
    }
    else {
        if heap.peek().unwrap().0.0.0 < score  {
            heap.pop();
            heap.push((Reverse(Score(score)), item))
        }
    }
}
let vec = heap.into_sorted_vec();
return vec.into_iter().map(|(x, y)| (y, x.0.0)).collect()
}

pub fn top_k_retrieval(k: usize, user_id: String, index: &IndexRepresentation) -> Result<Vec<(String, f32)>, String> {
if k== 0 { return Err("K has to be greater than 0.".to_string())} 
let items_emb = index.get_item_embeddings();
let user_emb = index.get_user_embeddings(user_id)?;

let c: Vec<(String, f32)>  = items_emb.iter().filter_map(|(x, y)| (compute_similarity(&user_emb, y).ok().map(|score| (x.clone(), score)))).collect();
return Ok(get_min_heap(k, c));
} 

#[cfg(test)]
use crate::index_representation::{create_index};
#[test]
fn test_result_decreasing_order(){
    let index = create_index();
    let res = top_k_retrieval(2, "2".to_string(), &index) ;
    //println!("{:?}", res.unwrap())
    assert_eq!(res.unwrap().windows(2).all(|w| w[0].1 >= w[1].1), true); 
}


#[test]
fn test_max_len_k(){
    let index = create_index();
    let res = top_k_retrieval(2, "2".to_string(), &index) ;
    assert_eq!(res.unwrap().len(), 2)
}

#[test]
fn test_no_user_id(){
    let index = create_index();
    let res = top_k_retrieval(2, "gwen".to_string(), &index) ;
    assert_eq!(res.is_ok(), false)
}

#[test]
fn test_k_zero(){
    let index = create_index();
    let res = top_k_retrieval(0, "gwen".to_string(), &index) ;
    assert_eq!(res.is_ok(), false)
}

#[test]
fn test_k_greater_than_index_size(){
    let index = create_index();
    let res = top_k_retrieval(10, "gwen".to_string(), &index) ;
    assert_eq!(res.is_ok(), false)
}