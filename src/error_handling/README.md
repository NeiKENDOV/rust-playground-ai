# Error Handling in Rust

## Overview
This module covers Rust's approach to error handling using the `Result` and `Option` enums.

## Key Concepts

### Result Enum
The `Result` enum is used for operations that might fail:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Unrecoverable Errors with panic!
When a program encounters an unrecoverable error, it can panic:

```rust
// This will cause the program to panic
// panic!("crash and burn");
```

### Recoverable Errors with Result
Most errors in Rust are recoverable and represented using `Result`:

```rust
use std::fs::File;

let f = File::open("hello.txt");

match f {
    Ok(file) => {
        // Handle the file
    }
    Err(error) => {
        // Handle the error
    }
}
```

### The ? Operator
The `?` operator propagates errors to the calling code:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

### unwrap and expect
For cases where you expect an operation to succeed, you can use `unwrap` or `expect`:

```rust
let f = File::open("hello.txt").unwrap(); // Panics if error
let f = File::open("hello.txt").expect("Failed to open hello.txt"); // Panics with custom message if error
```

### Option Enum
The `Option` enum is used to express the possibility of absence:

```rust
enum Option<T> {
    Some(T),
    None,
}

let some_number = Some(5);
let absent_number: Option<i32> = None;
```

## Exercises

1. Write a function that returns a Result based on a condition
2. Use match to handle a Result value
3. Use the ? operator to propagate errors in a function
4. Use Option to represent the result of a search operation

## Further Reading
- [The Rust Programming Language - Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [The Rust Programming Language - Panic!](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html)
- [The Rust Programming Language - To panic! or Not to panic!](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html)