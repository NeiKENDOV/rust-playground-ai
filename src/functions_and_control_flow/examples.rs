// Examples of functions and control flow in Rust

pub fn functions_demo() {
    // Function with parameters and return type
    fn add_numbers(a: i32, b: i32) -> i32 {
        a + b // expression-based return (no semicolon)
    }
    
    let result = add_numbers(5, 10);
    println!("5 + 10 = {}", result);
    
    // If expressions
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
    println!("The value of number is: {}", number);
    
    // Loop
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {}", result);
    
    // While loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    // For loop
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    
    // Range in for loop
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}