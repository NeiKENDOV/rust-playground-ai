// Playground file for experimenting with Rust code
// This file is intended for trying out concepts, testing snippets, and prototyping

// Uncomment sections as needed to experiment with different Rust concepts

// ====================
// 1. VARIABLES & MUTABILITY
// ====================
/*
fn variables_demo() {
    // Immutable variable
    let x = 5;
    println!("x = {}", x);
    
    // Mutable variable
    let mut y = 10;
    println!("y = {}", y);
    y = 15;
    println!("y after mutation = {}", y);
    
    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);
    
    // Shadowing
    let z = "hello";
    let z = z.len();
    println!("z (after shadowing) = {}", z);
}
*/

// ====================
// 2. DATA TYPES
// ====================
/*
fn data_types_demo() {
    // Scalar types
    let integer: i32 = -42;
    let unsigned: u32 = 42;
    let floating: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'z';
    
    // Compound types - Tuple
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple; // destructuring
    println!("tuple values: {}, {}, {}", x, y, z);
    
    // Access tuple elements by index
    let first = tuple.0;
    
    // Compound types - Array
    let array = [1, 2, 3, 4, 5];
    let first_element = array[0];
    println!("first element = {}", first_element);
    
    // Array with type and size
    let explicit_array: [i32; 5] = [1, 2, 3, 4, 5];
}
*/

// ====================
// 3. FUNCTIONS & CONTROL FLOW
// ====================
/*
fn functions_demo() {
    // Function with parameters and return type
    fn add_numbers(a: i32, b: i32) -> i32 {
        a + b // expression-based return (no semicolon)
    }
    
    let result = add_numbers(5, 10);
    println!("5 + 10 = {}", result);
    
    // If expressions
    let number = 6;
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    
    // Using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
    
    // Loop
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {}", result);
    
    // While loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    // For loop
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    
    // Range in for loop
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
*/

// ====================
// 4. OWNERSHIP & BORROWING
// ====================
/*
fn ownership_demo() {
    // Ownership rules:
    // 1. Each value has a variable that's called its owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value is dropped
    
    {
        let s = String::from("hello"); // s is valid from this point forward
        
        // do stuff with s
        println!("s = {}", s);
    } // this scope is now over, and s is no longer valid
    
    // Move semantics
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // println!("{}, world!", s1); // This would cause a compile error!
    
    // Clone
    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy
    println!("s1 = {}, s2 = {}", s1, s2); // This works!
    
    // References and borrowing
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1); // &s1 creates a reference to s1
    println!("The length of '{}' is {}.", s1, len);
    
    // Mutable references
    let mut s = String::from("hello");
    change(&mut s);
    println!("s = {}", s);
    
    // Multiple mutable references
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2); // This would cause a compile error!
    
    // Dangling references
    // let reference_to_nothing = dangle(); // This would cause a compile error!
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len() // s goes out of scope here, but the value it points to does not
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// This function will cause a compile error because it returns a reference to data that will be dropped
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // This would return a reference to s
// } // s goes out of scope and is dropped. Its memory goes away.
*/

// ====================
// 5. STRUCTS & METHODS
// ====================
/*
fn structs_demo() {
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
*/

// ====================
// 6. ENUMS & PATTERN MATCHING
// ====================
/*
fn enums_demo() {
    // Basic enum
    enum IpAddrKind {
        V4,
        V6,
    }
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    // Enum with data
    enum IpAddr {
        V4(String),
        V6(String),
    }
    
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    
    // Enum with different types of data
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    impl Message {
        fn call(&self) {
            // method body would be defined here
            match self {
                Message::Quit => println!("Quit message"),
                Message::Move { x, y } => println!("Move to ({}, {})", x, y),
                Message::Write(text) => println!("Write: {}", text),
                Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
            }
        }
    }
    
    let m = Message::Write(String::from("hello"));
    m.call();
    
    // Option enum (built-in)
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    // Pattern matching with match
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    
    println!("Value: {} cents", value_in_cents(Coin::Penny));
    
    // Match with Option
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    // if let (concise way to handle one pattern)
    let some_value = Some(0u8);
    if let Some(3) = some_value {
        println!("three");
    } else {
        println!("not three");
    }
}
*/

// ====================
// 7. ERROR HANDLING
// ====================
/*
fn error_handling_demo() {
    use std::fs::File;
    use std::io::{self, Read};
    
    // Result enum
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
        
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        
        let mut s = String::new();
        
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    
    // Using the ? operator (shorthand for the match above)
    fn read_username_from_file_short() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
    
    // Using match with Result
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(error) => println!("Error reading file: {}", error),
    }
    
    // Unwrap and expect
    // let s = read_username_from_file().unwrap(); // panics if Err
    // let s = read_username_from_file().expect("Failed to read username"); // panics with custom message if Err
    
    // Option and unwrap
    let v = vec![1, 2, 3];
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
*/

// ====================
// 8. COLLECTIONS
// ====================
/*
fn collections_demo() {
    // Vector
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    
    // Access elements
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    // Iterating
    for i in &v {
        println!("{}", i);
    }
    
    // Mutable iteration
    for i in &mut v {
        *i += 50;
    }
    
    // String (growable, UTF-8 encoded)
    let mut s = String::new();
    
    let data = "initial contents";
    let s = data.to_string();
    // or
    let s = "initial contents".to_string();
    
    // Using the + operator (actually calls the add method)
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    
    // Using format