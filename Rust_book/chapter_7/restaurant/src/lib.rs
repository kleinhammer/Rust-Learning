mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
        Calamari,
    }

    pub struct Breakfast {
        pub toast: String,
        season_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                season_fruit: String::from("Peaches"),
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

    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        let mut meal = back_of_house::Breakfast::summer("Rye");
        meal.toast == String::from("Wheat");
        println!("I'd like { } toast please", meal.toast);

        hosting::add_to_waitlist();
        // println!("The fruit is { }", meal.season_fruit);
        // crate::front_of_house::hosting::add_to_waitlist();
        // front_of_house::hosting::add_to_waitlist()
    }
}