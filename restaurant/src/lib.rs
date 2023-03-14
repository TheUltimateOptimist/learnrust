// use std::cmp::Ordering;
// use std::io;
//instead:
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
//instead:
use std::io::{self, Write};

use std::collections::*;//bring all defs into scope

mod front_of_house;
pub use crate::front_of_house::hosting;//reexporting with pub use

pub fn eat_at_restaurant() {
    //Absolute path
    hosting::add_to_waitlist();

    //relative path
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


//stating relative paths with super
fn deliver_order() {
    
}

mod back_of_house{
    pub enum Appetizer {//all fields are automatically public too
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
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {
        
    }
}
