mod hosts {
    fn seat_customers() {}
}
pub mod servers {
    use super::super::back;
    pub fn add_to_waitlist() {}
    pub fn take_order(food: &str) -> back::Order {
        back::Order::summer(food.to_string())
    }
    fn serve_order() {}
    fn take_payment() {}
}
