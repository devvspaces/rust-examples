use example::restaurant::front::servers;

fn main() {
    let mut order = servers::take_order("Bread");
    println!("This is our order: {order:?}");
    order.toast = "Pan".to_string();
}
