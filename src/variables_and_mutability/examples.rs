// Examples of variables and mutability in Rust

pub fn variables_demo() {
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