// Examples of structs and methods in Rust

pub fn structs_demo() {
    // Defining a struct
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    // Creating an instance
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // Accessing fields
    println!("User: {}", user1.username);
    
    // Mutable struct
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername456"),
        active: true,
        sign_in_count: 1,
    };
    
    user2.email = String::from("updated@example.com");
    
    // Struct update syntax
    let user3 = User {
        email: String::from("third@example.com"),
        username: String::from("thirdusername789"),
        ..user1 // Use values from user1 for remaining fields
    };
    
    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // Unit-like structs
    struct AlwaysEqual;
    
    let subject = AlwaysEqual;
    
    // Methods
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        // Method
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        // Method with parameter
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        
        // Associated function (constructor)
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    let square = Rectangle::square(10);
    println!("Square: {:?}", square);
}