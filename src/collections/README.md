# Collections in Rust

## Overview
This module covers Rust's collection types, including vectors, strings, and hash maps.

## Key Concepts

### Vectors
Vectors store multiple values of the same type in a single data structure:

```rust
let mut v: Vec<i32> = Vec::new();
v.push(5);
v.push(6);
v.push(7);

// Access elements
let third: &i32 = &v[2];

// Safe access with get
match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

### Iterating over Vectors
You can iterate over vectors using for loops:

```rust
// Immutable iteration
for i in &v {
    println!("{}", i);
}

// Mutable iteration
for i in &mut v {
    *i += 50;
}
```

### Strings
Rust's String type is a growable, UTF-8 encoded string type:

```rust
let mut s = String::new();

let data = "initial contents";
let s = data.to_string();
// or
let s = "initial contents".to_string();

// Using + operator (actually calls the add method)
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

// Using format! macro
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);
```

### Hash Maps
Hash maps store key-value pairs:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Access values
let team_name = String::from("Blue");
let score = scores.get(&team_name);

// Iterating over hash maps
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

## Exercises

1. Create a vector of numbers and calculate their sum
2. Use a for loop to iterate over a vector and modify its elements
3. Concatenate two strings using the + operator and the format! macro
4. Create a hash map to store student names and their grades

## Further Reading
- [The Rust Programming Language - Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [The Rust Programming Language - Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [The Rust Programming Language - Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)