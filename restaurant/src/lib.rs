//PART 1
// mod front_of_house{
//     mod hosting {
//         fn add_to_waitlist(){

//         }

//         fn seat_at_table(){

//         }
//     }

//     mod serving{
//         fn take_order(){

//         }

//         fn serve_order(){

//         }

//         fn take_payment(){

//         }
//     }
// }

//PART 2
// mod front_of_house{
//     pub mod hosting { //this is private by default
//         pub fn add_to_waitlist(){ //this is private by default

//         }
//     }
// }

// pub fn eat_at_restaurant(){
//     //Absolute path 
//     crate::front_of_house::hosting::add_to_waitlist();

//     //Relative Path
//     front_of_house::hosting::add_to_waitlist();
// }

// fn serve_order(){}

// mod back_of_house{
//     fn fix_incorrect_order(){
//         cook_order();
//         super::serve_order(); //serve order function is in the crate (parent module)
//     }
//     fn cook_order(){}
// }

//PART 3 - Using Structs
// mod back_of_house{
//     pub struct Breakfast{
//         pub toast: String, //To allow reassignment, the feild must be assigned as public
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast{
//             Breakfast{
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant(){
//     let mut meal: Breakfast = back_of_house::Breakfast::summer("Rye");

//     meal.toast = String::from("Wheat"); //This is by default private
// }

//Part 4 - Using ENUMS
// mod back_of_house {
//     pub enum Appetzer { //This is by default private
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant (){
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }

// Part 5 - Use keyword
// mod front_of_house {
//     pub mod hosting{
//         pub fn add_to_waitlist(){}
//     }
// }

// use crate::front_of_house::hosting;
// // use self::front_of_house::hosting; //this is a relative path where self is just referncing the current module

// pub fn eat_at_restaurant(){
//     // use keyword allows us to get a module into scope

//     //front_of_house::hosting::add_to_waitlist(); <- no need to use this if you're using use keyword
//     add_to_waitlist();// just use this while using the use keyword
//     front_of_house::hosting::add_to_waitlist();
// }

//Part 6 - naming conflicts 
// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function() -> fmt::Result{ //Result clashes with Result<()> from io. Therefore we use 'as' to get the module
//     // --snip--
// }

// fn function() -> io::IoResult<()>{
//     // --snip--
// }

//Part 7 - if we want external files to access stuff internal to the file
// use rand::Rng;//bringing rand into scope
// //Using nested paths
// use rand::{Rng, CryptoRng, ErrorKind::Transient};
// use std::io::{self, Write};
// use std::io::*; //This is the glob operator that brings all public objects into scope

// mod front_of_house {
//     pub mod hosting{
//         pub fn add_to_waitlist(){}
//     }
// }

// pub use crate::front_of_house::hosting; //We add the pub keyword to allow files oustide to access the hosting module

// pub fn eat_at_restaurant(){
//     add_to_waitlist();// just use this while using the use keyword
//     front_of_house::hosting::add_to_waitlist();
// }

//Part 8 - Modules from other files 
mod front_of_house; //This specifies that we will refernce the implementation of the front of house module from another file with the same name

use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){
    front_of_house::hosting::add_to_waitlist();
}

pub fn main(){
    eat_at_restaurant();
}