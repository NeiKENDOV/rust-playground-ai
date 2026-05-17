// Examples of enums and pattern matching in Rust

pub fn enums_demo() {
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