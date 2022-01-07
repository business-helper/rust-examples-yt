// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
  }

  mod serving {
    fn take_order() {}

    fn server_order() {}

    fn take_payment() {}
  }
}

// mod back_of_house {
//   fn fix_incorrect_order() {
//     cook_order();
//     super::serve_order();
//   }

//   fn cook_order() {}
// }

mod back_of_house {
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

pub fn eat_at_restaurant() {
  // Absolute path
  crate::front_of_house::hosting::add_to_waitlist();

  // Relative path
  front_of_house::hosting::add_to_waitlist();
}

pub fn eat_at_back_of_house() {
  let mut meal = back_of_house::Breakfast::summer("Rye");

  meal.toast = String::from("Wheat");
}

// --------------------------- Enum in module --------------

mod back_of_house2 {
  pub enum Appetizer {
    Soup,
    Salad,
  }
}

pub fn eat_at_restaurant2() {
  let order1 = back_of_house2::Appetizer::Soup;
  let order2 = back_of_house2::Appetizer::Salad;
}

// ----------------------------------------------

mod front_of_house3 {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

// use crate::front_of_house3::hosting;
use self::front_of_house3::hosting;

pub fn eat_at_restaurant3() {
  front_of_house3::hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  front_of_house3::hosting::add_to_waitlist();
}

use std::fmt;
use std::io;
use std::fmt::Result;
use std::io::Result as IoResult;

// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}

fn function1() -> Result {
  Ok(())
}
fn function2() -> IoResult<()> {
  Ok(())
}




