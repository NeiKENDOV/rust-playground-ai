# Enums and Pattern Matching in Rust

## Overview
This module covers enum definitions, the match expression, and pattern matching in Rust.

## Key Concepts

### Enum Definitions
Enums allow you to define a type by enumerating its possible variants:

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

### Enums with Data
Each variant of an enum can have data associated with it:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

### Match Expression
The `match` expression allows you to compare a value against a series of patterns:

```rust
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
```

### Patterns with Option
The `Option` enum is used to express the possibility of absence:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### if let
For cases where you only care about one pattern, you can use `if let`:

```rust
let some_value = Some(0u8);
if let Some(3) = some_value {
    println!("three");
} else {
    println!("not three");
}
```

## Exercises

1. Define an enum to represent different types of vehicles
2. Create a function that uses match to return different values based on the vehicle type
3. Use Option to represent the possibility of a user's middle name
4. Use if let to handle a specific case from an Option value

## Further Reading
- [The Rust Programming Language - Defining an Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [The Rust Programming Language - The match Control Flow Operator](https://doc.rust-lang.org/book/ch06-02-match.html)
- [The Rust Programming Language - Concise Control Flow with if let](https://doc.rust-lang.org/book/ch06-03-if-let.html)