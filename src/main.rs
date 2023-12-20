struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Method Syntax Impl
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }       
}

impl Rectangle{
    fn square(size:u32)-> Self{
        Self{
            width:size,
            height:size,
        }
    }
}

fn main() {
    // Creating Instance of Struct
    let mut user1 = User {
        active: true,
        username: String::from("Saksham"),
        email: String::from("sakshamg45@gmail.com"),
        sign_in_count: 1,
    };

    // To update the value using (.) dot operator
    // user1.email = String::from("sakshamgupta.ai@gmail.com");

    build_user(
        "Krishna".to_string(),
        "krishnathegod@loveall.com".to_string(),
    );

    // Using Struct Update Syntax
    let user2 = User {
        email: String::from("sakshamgupta.ai@gmail.com"),
        username: String::from("Yogi Saksham"),
        ..user1
    };

    // Structs usage for area calculation
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The Area of Rectangle is {} sq. pixels", area(&rect1));
    println!("The Rect1 is {:#?}", rect1);

    // using dgb! macro
    println!("\n");

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);

    // Method Syntax Usage
    println!("\n");
    println!("The area of the rectangle is {} sq pixels", rect1.area());

    // Associated Functions
    println!("\n");
    let sq = Rectangle::square(5);
    println!("The Area of Square is {}", sq.area());
}

// Field Init Shorthand used here
fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
