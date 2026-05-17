# Variables and Mutability in Rust

## Overview
This module covers the fundamentals of variable declarations, mutability, shadowing, and constants in Rust.

## Key Concepts

### Variable Declarations
In Rust, variables are immutable by default. You declare a variable using the `let` keyword:

```rust
let x = 5;
// x = 6; // This would cause a compile-time error!
```

### Mutability
To make a variable mutable, use the `mut` keyword:

```rust
let mut y = 5;
y = 6; // This is allowed
println!("y = {}", y);
```

### Constants
Constants are values that are bound to an identifier and cannot be changed. They are declared with the `const` keyword and must have a type annotation:

```rust
const MAX_POINTS: u32 = 100_000;
```

### Shadowing
Rust allows you to declare a new variable with the same name as a previous variable. This is called shadowing:

```rust
let x = 5;
let x = x + 1; // Shadow the previous x
let x = x * 2; // Shadow the previous x again
println!("The value of x is: {}", x); // Prints 12
```

## Exercises

1. Create a mutable variable and modify its value
2. Try to modify an immutable variable and observe the compiler error
3. Use shadowing to convert a temperature from Celsius to Fahrenheit
4. Declare a constant for the number of seconds in a minute

## Further Reading
- [The Rust Programming Language - Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)