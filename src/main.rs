mod variables_and_mutability;
mod data_types;
mod functions_and_control_flow;
mod ownership_and_borrowing;
mod structs_and_methods;
mod enums_and_pattern_matching;
mod error_handling;
mod collections;

fn main() {
    println!("Welcome to the Rust Learning Project!");
    println!("This project demonstrates fundamental Rust concepts through modular examples.");
    println!("\nRunning examples from each module:\n");
    
    // Variables and Mutability
    println!("=== Variables and Mutability ===");
    variables_and_mutability::examples::variables_demo();
    println!("\n");
    
    // Data Types
    println!("=== Data Types ===");
    data_types::examples::data_types_demo();
    println!("\n");
    
    // Functions and Control Flow
    println!("=== Functions and Control Flow ===");
    functions_and_control_flow::examples::functions_demo();
    println!("\n");
    
    // Ownership and Borrowing
    println!("=== Ownership and Borrowing ===");
    ownership_and_borrowing::examples::ownership_demo();
    println!("\n");
    
    // Structs and Methods
    println!("=== Structs and Methods ===");
    structs_and_methods::examples::structs_demo();
    println!("\n");
    
    // Enums and Pattern Matching
    println!("=== Enums and Pattern Matching ===");
    enums_and_pattern_matching::examples::enums_demo();
    println!("\n");
    
    // Error Handling
    println!("=== Error Handling ===");
    error_handling::examples::error_handling_demo();
    println!("\n");
    
    // Collections
    println!("=== Collections ===");
    collections::examples::collections_demo();
    println!("\n");
    
    println!("Check the README.md files in each module for detailed explanations and exercises!");
}
