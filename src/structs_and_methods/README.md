# Structs and Methods in Rust

## Overview
This module covers struct definitions, creating instances, and implementing methods on structs.

## Key Concepts

### Struct Definitions
Structs allow you to create custom data types with named fields:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

### Creating Instances
You create instances of structs using the field names:

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

### Accessing Fields
You access fields using dot notation:

```rust
println!("User: {}", user1.username);
```

### Mutable Structs
You can make a struct instance mutable to modify its fields:

```rust
let mut user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername456"),
    active: true,
    sign_in_count: 1,
};
user2.email = String::from("updated@example.com");
```

### Struct Update Syntax
You can use the struct update syntax to create a new instance with some fields from an old instance:

```rust
let user3 = User {
    email: String::from("third@example.com"),
    username: String::from("thirdusername789"),
    ..user1 // Use values from user1 for remaining fields
};
```

### Tuple Structs
You can define structs that look similar to tuples:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

### Methods
Methods are functions defined within the context of a struct:

```rust
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
```

## Exercises

1. Define a struct to represent a book with title, author, and year
2. Create an instance of your book struct
3. Implement a method on the book struct that returns a formatted string
4. Create an associated function that serves as a constructor for books

## Further Reading
- [The Rust Programming Language - Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [The Rust Programming Language - Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)