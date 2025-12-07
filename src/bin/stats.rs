use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut arr = generate_array(100);
    arr.sort();
    println!("This is the array: {arr:?}");
    println!("This is the median: {}", median(&arr).unwrap_or(0.0));
    println!("This is the mode: {}", mode(&arr).unwrap_or(&0));
}

fn median(n: &Vec<u32>) -> Option<f32> {
    if n.len() == 0 {
        return None;
    }
    let m1 = (n.len() / 2) + 1;
    if n.len() % 2 == 1 {
        return Some(*n.get(m1).unwrap() as f32);
    } else {
        Some((*n.get(m1).unwrap() as f32 + *n.get(m1 + 1).unwrap() as f32) / 2 as f32)
    }
}

fn mode(n: &Vec<u32>) -> Option<&u32> {
    let mut map = HashMap::new();
    let mut largest: Option<&u32> = None;
    let mut cur = 0;
    for x in n {
        let count = map.entry(x).or_insert(0);
        *count += 1;
        if let Some(_) = largest {
            if *count > cur {
                largest = Some(x);
                cur = *count;
            }
        } else {
            largest = Some(x);
        }
    }
    largest
}

fn generate_array(n: usize) -> Vec<u32> {
    let mut x: Vec<u32> = Vec::with_capacity(n);
    for _ in 0..n {
        x.push(rand::rng().random_range(0..100))
    }
    x
}
