use std::io;
// std is crate, and io is module 
fn add_numbers(x: i32, y: i32) -> i32{
    println!("The sum is: {}", x + y);
    let number = {
        let x = 3;
        x + 1 //This is evalutaed expression that is returned to the variable
        // DO NOT USE SEMICOLON IN THE LAST LINE
    };
    println!("The number is: {}", number);
    return x + y + number; //last statement is returned
    // Just using the below, without semi colon statement works too
    //x + y + number
}



fn main() {
    // ------------------------
    // //implicit type 
    // let x = 4; //this is immutable by default 
    // println!("x is : {}", x);
    // ------------------------
    // // mutable
    // let mut y = 5; //this is mutable 
    // println!("y is : {}", y);
    // y = 6; //this is mutable 
    // println!("y is : {}", y);

    // ------------------------
    // //scope shadowing
    // {
    //     let x = x - 2;
    //     println!("x is : {}", x);
    // }
    // ------------------------
    // //constants
    // const SECONDS_IN_MINUTES: u32 = 60; //u means unsigned
    // println!("constants is : {}", SECONDS_IN_MINUTES);

    // //explicit type 
    // let n : i32 = 30; // i means unsigned
    // let booleans : bool = false;
    // let character: char = 'a';
    // /*
    // Types of integers (unsigned)
    // i8, i16, i32, i128
    // */
    // /*
    // Types of floats (unsigned)
    // f8, f16, f32, f128
    // */
    // ------------------------
    // //tuples and arrays
    // let mut tup: (i32, i32, char) = (1,2, 's');
    // tup.0 = 10;
    // println!("{}", tup.0);

    // let mut arr : [i32; 5] = [1,2,3,4,5]; //cant dynamically size
    // println!("{}", arr[2]);
    // ------------------------
    // //Console input
    // // def prelude: implicit packages loaded from rust std library
    // println!("Hi");
    // let mut input = String::new(); // :: is a path seperator, navigating packages
    // io::stdin().read_line(&mut input).expect("failed to read line"); //&mut is creating a mutable reference
    // println!("{}", input);
    // ------------------------

    //------------------------
    // //type casting 
    // let x = 255.0_f32;
    // let y = 10.0f32;
    // let n = i8::MAX as i8;
    // let z = x/(n as f32); //controlling for oveflow and decimal precision

    // println!("{}", z);
    // ------------------------
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("expected to read line");
    // let int_input: i64 = input.trim().parse().unwrap();
    // println!("{}", int_input + 2)
    // ------------------------

    //conditional and control flow 
    /*
    Boolean Operators: 
    && || !
    */
    /*
    if _conditional_ {} else if _condition_ {} else {}
    */
     

} 