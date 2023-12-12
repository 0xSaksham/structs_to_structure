# Rust Structs Exploration

This Rust program showcases the use of structs, demonstrating their creation, update syntax, and practical applications. It defines two structs, `User` and `Rectangle`, highlighting their usage and the field initialization shorthand.

## User Struct

The `User` struct represents user data with fields for activity status, username, email, and sign-in count. Instances are created and updated, showcasing both conventional and update syntax.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

## Rectangle Struct

The `Rectangle` struct is designed for calculating the area of rectangles, with width and height as its fields. The program demonstrates the usage of this struct in the area calculation function.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

## Main Function

The `main` function illustrates the creation, update, and usage of instances of the `User` and `Rectangle` structs. It also showcases the field initialization shorthand and the application of structs in practical scenarios.

```rust
fn main() {
    // Creating Instance of Struct
    let mut user1 = User {
        active: true,
        username: String::from("Saksham"),
        email: String::from("sakshamg45@gmail.com"),
        sign_in_count: 1,
    };

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
}
```

## Functions

The program includes functions such as `build_user` for creating `User` instances with field initialization shorthand and `area` for calculating the area of rectangles.

```rust
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
```

Explore this code to understand the fundamentals of struct usage and field initialization in Rust.
