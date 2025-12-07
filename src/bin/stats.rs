use std::collections::HashMap;
use example::util::generate_numbers;

fn main() {
    let mut arr = generate_numbers(100);
    arr.sort();
    println!("This is the array: {arr:?}");
    println!("This is the median: {}", median(&arr).unwrap_or(0.0));
    println!("This is the mode: {}", mode(&arr).unwrap_or(&0));
}

fn median(n: &Vec<u64>) -> Option<f32> {
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

fn mode(n: &Vec<u64>) -> Option<&u64> {
    let mut map = HashMap::new();
    let mut largest: Option<&u64> = None;
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


