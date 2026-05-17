# Ownership and Borrowing in Rust

## Overview
This module covers Rust's ownership system, which is a set of rules that govern how memory is managed in Rust programs.

## Key Concepts

### Ownership Rules
1. Each value in Rust has a variable that's called its owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value is dropped

```rust
{
    let s = String::from("hello"); // s is valid from this point forward
    // do stuff with s
    println!("s = {}", s);
} // this scope is now over, and s is no longer valid
```

### Move Semantics
When assigning a value to another variable, Rust moves the value:

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is moved to s2
// println!("{}, world!", s1); // This would cause a compile error!
```

### Clone
To create a deep copy of a value, use the `clone` method:

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // deep copy
println!("s1 = {}, s2 = {}", s1, s2); // This works!
```

### References and Borrowing
References allow you to refer to a value without taking ownership:

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1); // &s1 creates a reference to s1

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // s goes out of scope here, but the value it points to does not
```

### Mutable References
You can create mutable references to modify values:

```rust
let mut s = String::from("hello");
change(&mut s);

fn change(s: &mut String) {
    s.push_str(", world");
}
```

### The Rules of References
1. At any given time, you can have either one mutable reference or any number of immutable references
2. References must always be valid

## Exercises

1. Demonstrate the move semantics with a String
2. Create a function that takes a reference to a String and returns its length
3. Create a function that modifies a String through a mutable reference
4. Try to create two mutable references to the same variable and observe the compiler error

## Further Reading
- [The Rust Programming Language - Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [The Rust Programming Language - References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)