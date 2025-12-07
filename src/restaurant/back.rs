#[derive(Debug)]
pub struct Order {
    pub toast: String,
    fruit: String,
}

impl Order {
    pub fn summer(toast: String) -> Self {
        Self {
            toast,
            fruit: String::from("Peaches"),
        }
    }
}

fn cook() {}
fn wash() {}
