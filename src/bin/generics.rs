use std::ops::Mul;


#[derive(Debug)]
struct Point<T: Mul> ( T, T );

impl Point<f32> {
    fn distance(&self) -> f32 {
        (self.0.powf(2.0) + self.1.powf(2.0)).sqrt()
    }
}

fn main() {
    let p1 = Point(1.0, 2.3);
    let p2 = Point(1, 2);
    println!("This is our point: {p1:?}");
    println!("This is the distance from 0,0: {}", p1.distance());
}