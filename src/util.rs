use rand::Rng;

pub fn generate_numbers(n: usize) -> Vec<u64> {
    let mut x: Vec<u64> = Vec::with_capacity(n);
    for _ in 0..n {
        x.push(rand::rng().random_range(0..1000))
    }
    x
}
