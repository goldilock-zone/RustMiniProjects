//Structs are nice way to group data in rust
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { //the implementation blocks house all the functions associated with the struct
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

//associated function or static function
impl Rectangle {
    fn square(size: u32) -> Rectangle { //associated functions dont get passed self
        Rectangle{
            width: size,
            height: size,
        }
    }
}

fn main() {
    //Ordinary Structs
    let mut user1 = User{
        email: String::from("sarbajitghosh2002@gmail.com"),
        username: String::from("goldilock_zone"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username = String::from("sarbajitghosh2002");

    let user2: User = build_user(
        String::from("mcpe_inventors@gmail.com"),
        String::from("mcpe_inventors"),
    );

    let user3: User = User{
        email: String::from("jason@gmail.com"),
        username: String::from("jason"),
        ..user2 //duplicates all other attributes from user 2
    };

    //Tuple Structs - used when we want ssame tuple structure or data structure, but different explicit type
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    //implementing the Rectangle struct 
    let rect: Rectangle = Rectangle{
        width:30,
        height:50,
    };

    let rect1: Rectangle = Rectangle{
        width:20,
        height:40,
    };

    let rect2: Rectangle = Rectangle{
        width:40,
        height:50,
    };

    println!("rect: {:#?}", rect);
    println!("The area of the rectangle is {}", rect.area());

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    //associated functions
    let rect3
}

fn build_user(email: String, username: String) -> User{
    User{
        email: email, // we can shorten this to just "email"
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

