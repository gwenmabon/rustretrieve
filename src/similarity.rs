

pub fn dot_product( vec_a: &[f32], vec_b:&[f32]) -> f32 {
    vec_a.iter().zip(vec_b.iter()).map(|(x, y)| x * y).sum()
}


#[test]
fn identical_vector(){
    let vec_a = [1.0, 0.0];
    let vec_b = [1.0, 0.0];
    assert_eq!(dot_product(&vec_a, &vec_b), 1.0)
}
#[test]
fn orthogonal_vector(){
    let vec_a = [1.0, 0.0];
    let vec_b = [0.0, 1.0];
    assert_eq!(dot_product(&vec_a, &vec_b), 0.0)
}