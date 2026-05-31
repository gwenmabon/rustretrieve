

pub fn dot_product( vec_a: &[f32], vec_b:&[f32]) -> f32 {
    vec_a.iter().zip(vec_b.iter()).map(|(x, y)| x * y).sum()
}