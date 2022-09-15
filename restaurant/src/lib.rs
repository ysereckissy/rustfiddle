mod front_of_house;
mod back_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    hosting::add_to_waitlist();
    // relative
    hosting::add_to_waitlist();
    // order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change mind about bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // the followng line will fail
    // meal.seasonal_fruit = String::from("blueberries");

    // setting an enum public will make all its members public
    let order = back_of_house::Appetizer::Soup;
}

fn deliver_order() {

}