# Functions and Control Flow in Rust

## Overview
This module covers functions, if/else expressions, and various loop constructs in Rust.

## Key Concepts

### Functions
Functions in Rust are defined using the `fn` keyword. Parameters must have type annotations, and the return type is specified after an arrow `->`:

```rust
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b  // Expression-based return (no semicolon)
}

let result = add_numbers(5, 10);
```

### If Expressions
In Rust, `if` is an expression that returns a value:

```rust
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
```

### Loops
Rust provides several ways to loop: `loop`, `while`, and `for`.

#### Loop
`loop` creates an infinite loop that can be exited with `break`:

```rust
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};
```

#### While
`while` loops as long as a condition is true:

```rust
let mut number = 3;
while number != 0 {
    println!("{}!", number);
    number -= 1;
}
```

#### For
`for` is used to iterate over a collection:

```rust
let a = [10, 20, 30, 40, 50];
for element in a.iter() {
    println!("the value is: {}", element);
}

// Range in for loop
for number in (1..4).rev() {
    println!("{}!", number);
}
```

## Exercises

1. Write a function that calculates the factorial of a number
2. Use a loop to find the first Fibonacci number greater than 1000
3. Write a function that returns different values based on an if expression
4. Use a for loop to iterate through a range and print only even numbers

## Further Reading
- [The Rust Programming Language - Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [The Rust Programming Language - Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)