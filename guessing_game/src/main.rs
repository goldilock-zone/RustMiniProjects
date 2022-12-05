use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Hello, User");

    let low: i8 = 1;
    let high: i8 = 101;
 
    let secret_number = rand::thread_rng().gen_range(low, high) as u32;
    println!("Secret Number: {}", secret_number);

    loop {
        println!("Input you r guess. ");

        let mut guess: String = String::new(); //new is a static function or associated function reference
        io::stdin().read_line(&mut guess).expect("Error reading input");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_)=> continue,
        };

        println!("Your guess is: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());

                let a: [i32; 3] = [3,2,1];

                for element in a.iter(){
                    println!("{}", element)
                };
                println!("Lift Off!");
                break;
            }
        }
    }
}
