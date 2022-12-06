mod back_of_house;
mod front_of_house;

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
