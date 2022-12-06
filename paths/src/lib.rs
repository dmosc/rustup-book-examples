mod back_of_house {
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod customer {
    // If this is imported outside `customer` module, we could refer to the
    // modules as well using `super::Appetizer` for example.
    use crate::back_of_house::{Appetizer, Breakfast};
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        let mut meal = Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd {} like toast please", meal.toast);

        let order1 = Appetizer::Soup;
        let order2 = Appetizer::Salad;
        println!("{:?}\n{:?}", order1, order2);
    }
}
