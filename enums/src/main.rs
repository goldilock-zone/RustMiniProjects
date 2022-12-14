//normal enum
enum IpAddrKind{ //used to define "variants, within a "class"
    V4(u8,u8,u8,u8),
    V6(u8,u8,u8,u8),
}

enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_function(){
        println!("Implementation of Message")
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

//Defining coin enum for use with match functions
#[derive(Debug)]
enum Coin{
    Penny, Nickel, Dime, Quarter,
}


fn main() {
    // let four:IpAddrKind = IpAddrKind::V4;
    // let six:IpAddrKind = IpAddrKind::V6;

    //Type 1
    // let localhost: IpAddrKind = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };

    let localhost = IpAddrKind::V4(127,0,0,1);

    //Option enum 
    // enum Option<T>{
    //     Some(T),
    //     None,
    // }

    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("A String");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);
    println!("{}", sum);
    
    println!("{}",value_in_cents(Coin::Quarter));
    println!("{:#?}",Coin::Quarter);

    //You can have nested enums
    // Eg: Coin::Quarter(UsState::Alaska)

    //Combining Match with option enum
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:#?}", six);
    println!("{:#?}", none);

    //if let syntax
    let some_value = Some(3);
    match some_value{
        Some(3) => println!("three"),
        _=> (),
    }

    if let Some(3) = some_value { //Only considers the some(3) case and ignores the rest
        println!("three") //slightly less verbose than match, but little moore confusing
    }
}

fn route(ip_kind:IpAddrKind){

}

fn value_in_cents(coin: Coin) -> usize{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

//Combining Match with option enum
fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i+1),
    }
}