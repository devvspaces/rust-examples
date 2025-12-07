mod restaurant {
    pub mod front {
        mod hosts {
            fn seat_customers() {}
        }
        pub mod servers {
            pub fn add_to_waitlist() {}
            fn take_order() {}
            fn serve_order() {}
            fn take_payment() {}
        }
    }
    pub mod back {
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
    }
}

fn main() {
    crate::restaurant::front::servers::add_to_waitlist();
    use restaurant::front::servers;
    servers::add_to_waitlist();

    let mut order = restaurant::back::Order::summer("Rye".to_string());
    order.toast = "Pan".to_string();
}