# Data Types in Rust

## Overview
This module covers the fundamental data types in Rust, including scalar and compound types.

## Key Concepts

### Scalar Types
Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

#### Integers
Signed and unsigned integers of various sizes:

```rust
let signed: i32 = -42;    // 32-bit signed integer
let unsigned: u32 = 42;   // 32-bit unsigned integer
```

#### Floating-Point Numbers
Rust has two primitive floating-point types: `f32` and `f64`:

```rust
let x = 2.0;      // f64
let y: f32 = 3.0; // f32
```

#### Booleans
```rust
let t = true;
let f: bool = false;
```

#### Characters
```rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
```

### Compound Types

#### Tuple
A tuple groups together a number of values with a variety of types into one compound type:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// Destructuring
let (x, y, z) = tup;

// Access by index
let first = tup.0;
```

#### Array
Arrays in Rust have a fixed length and all elements must be of the same type:

```rust
let a = [1, 2, 3, 4, 5];
let first = a[0];

// With type annotation and size
let explicit_array: [i32; 5] = [1, 2, 3, 4, 5];
```

## Exercises

1. Create a tuple with different data types and access its elements
2. Create an array of temperatures and calculate the average
3. Declare variables of each scalar type
4. Use type annotations for variables

## Further Reading
- [The Rust Programming Language - Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)