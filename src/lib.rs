// #![allow(dead_code)]
// mod front_house {
//     mod hosting {
//         fn add_towaitlist() {

//         }
//     }
// }

// mod server {
//     fn take_order() {

//     }
//     fn order_serve() {

//     }
//     fn payment () {

//     }
// }

// mod customer_experience{
// pub mod front_house{
//     pub mod hosting{
//         pub fn add_towaitlist() {

//         }
//     }
// }
// mod dining{
// pub fn eat_at_restaurant() {
//     //Absolute path
//     crate::customer_experience::front_house::hosting::add_towaitlist();
//     //Relative path
//     super::front_house::hosting::add_towaitlist();
// }
// }
// }

// mod front_house {
//     #[derive(Debug)]
//     pub struct Breakfast{
//         pub toast:String,
//         seasonal_fruit:String,
//     }
//     impl Breakfast {
//         pub fn new (toast:String) -> Breakfast{
//             Breakfast{
//             toast,
//             seasonal_fruit:String::from("Oranges")
//         }
//     }
// }
// }
// fn eat_at_restaurant () {
//     let mut meal = front_house::Breakfast::new(String::from("Wheat"));
//     println!("{:?}",meal);
//     meal.toast = String::from("Barley");
//     println!("{:?}",meal.toast);
    
// }

// #![allow(dead_code)]
// #![allow(unused_variable)]
// mod front_house {
//     pub enum appetizer {
//         Soap,
//         Salad,
//     }
// }
// fn eat_at_restaurant () {
//     let meal1 = front_house::appetizer::Soap;
//     let meal2 = front_house::appetizer::Salad;
// }
// use std::collections::HashMap;
// fn main () {
//     let mut contacts = HashMap::new();
//     contacts.insert("1","xyz@hotmail.com");
// }

// use std::fmt::Result;
// use std::io::Result as IoResult;
// fn main () {
//     fn function1 () -> fmt:: Result {

//     }
//     fn function2 () -> IoResult{

//     }
// }

use rand::Rng;
fn main () {
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("{}",secret_number);
}