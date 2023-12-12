struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}

struct Rectangle{
    width:u32,
    height:u32,

}

fn main(){
    // Creating Instance of Struct
    let mut user1 = User{
        active:true,
        username:String::from("Saksham"),
        email:String::from("sakshamg45@gmail.com"),
        sign_in_count:1
    };

    // To update the value using (.) dot operator
    // user1.email = String::from("sakshamgupta.ai@gmail.com");

    build_user("Krishna".to_string(),"krishnathegod@loveall.com".to_string());

    // Using Struct Update Syntax
    let user2 = User{
        email: String::from("sakshamgupta.ai@gmail.com"),
        username: String::from("Yogi Saksham"),
        ..user1
    };

    // Structs usage for area calculation
    let rect1 = Rectangle{
        width:30,
        height:50,
    };

    println!("The Area of Rectangle is {} sq. pixels",area(&rect1));
}


// Field Init Shorthand used here
fn build_user(username:String,email:String)-> User{
    User{
        active:true,
        username,
        email,
        sign_in_count:1
    }
}

fn area(rectangle: &Rectangle)->u32{
    rectangle.width * rectangle.height
}