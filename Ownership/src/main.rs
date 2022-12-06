fn main() {
    /*
    *Ownership rules: 
    1. Each variable in rust has an owner
    2. There can be only one owner at a time
    3. When the owner goes out of scope, the value will be dropped.
     */

    /*
    The rules of refernces - 
    1. At any given time, you can have either one mutable refernce or any number of immutable refernces t
    2. References must always be valid 9no dangling refernces)
     */

    {
        let s: &str = "hello"; //memory is allocated on the heap
    }//this scope is over, and s does not exist anymore
    
    let x:i32 = 5;
    let y:i32 = x; //copying not a move

    let s1: String = String::from("hello");
    let s2: &str = &s1; //Move (not shallow copy)

    let s3: String = String::from("hello");
    let s4: String = s3.clone();

    println!("{}", s3);

    takes_ownership(s4);
    //println!("{}", s4); //this line will produce an error because of the function taing Ownership
    // this works only for strings
    let mut s5:String = gives_ownership();
    println!("{}", s5); //returning a value giives ownership to the variable


    //references
    let mut s1:String = String::from("hello");
    let len = calculate_length(&s1); //without refernce we are giving ownership to the function
    println!("The length of {} is {}", s1, len);

    change(&mut s1);
    println!("Changed String: {}", s1);

    //you can only have one mutable refernce for a particular peice of data
    //otherwise, there will be several functions mutating the same peice of data, which might create conflicts
    //The above is called a data race
    //Its okay to have multiple immutable refernces

    //dangling refernces
    //let refernce_to_nothing: &String = dangle();

    //string slices
    let mut s: String = String::from("hello world");
    // let word: usize = first_word(&s);
    // s.clear();
    // println!("First Word: {}", word);

    let hello: &str = &s[..5];
    let world: &str = &s[6..];
    let word: &str = first_word(&s);
    println!("First Word (using slices): {}", word);

    let a: [i32; 5] = [1,2,3,4,5];
    let slice: &[i32] = &a[0..2];

    println!("Array slice: ");
    for i in slice.iter() {
        print!("{} ", i);
    }
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn gives_ownership() -> String{
    let some_string: String = String::from("hello");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string //this takes and gives back ownership
    //ownership lasts only for the duration of the scope of the function

}

//references
fn calculate_length(s: &String) -> usize { //passing refernces as function arguments is called borrowing
    let length: usize = s.len();
    length
}

fn change(some_string: &mut String){
    //make changes using the refernce to the underlying object
    some_string.push_str(", world");
}

//dangling refernces
// fn dangle() -> &String {
//     let s: String = String::from("hello");
//     //this is a dangling refernce because, ownership is dropped after the function
//     //Therefore, this returns a refernce to nothing
//     &s
// }

//slice
fn first_word(s: &String) -> &str { //we cant return part of the string so we retunr the index to the end of the last word
    let bytes: &[u8] = s.as_bytes(); //converts string to byte array
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}