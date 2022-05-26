use crate::front_of_house::hosting;
mod front_of_house;

mod back_of_house {
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
                toast: String::from("toast"),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}

mod customer {
    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}
fn deliver_order() {}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    let mut meal = back_of_house::Breakfast::summer("Rye");
    hosting::add_to_waitlist();
    meal.toast = String::from("Wheat");
    println!("i'd like {} toast please", meal.toast);
}
