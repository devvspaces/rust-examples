use example::util::generate_numbers;


fn largest<T: std::cmp::PartialOrd>(collection: &[T]) -> &T {
    let mut max = &collection[0];
    for item in collection {
        if item > max {
            max = item;
        }
    }
    max
}

fn main() {
    let v = generate_numbers(10);
    println!("The largest number is {}", largest(&v))
}