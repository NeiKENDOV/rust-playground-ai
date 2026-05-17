// Examples of ownership and borrowing in Rust

pub fn ownership_demo() {
    // Ownership rules:
    // 1. Each value has a variable that's called its owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value is dropped
    
    {
        let s = String::from("hello"); // s is valid from this point forward
        
        // do stuff with s
        println!("s = {}", s);
    } // this scope is now over, and s is no longer valid
    
    // Move semantics
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // println!("{}, world!", s1); // This would cause a compile error!
    
    // Clone
    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy
    println!("s1 = {}, s2 = {}", s1, s2); // This works!
    
    // References and borrowing
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1); // &s1 creates a reference to s1
    println!("The length of '{}' is {}.", s1, len);
    
    // Mutable references
    let mut s = String::from("hello");
    change(&mut s);
    println!("s = {}", s);
    
    // Multiple mutable references
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2); // This would cause a compile error!
    
    // Dangling references
    // let reference_to_nothing = dangle(); // This would cause a compile error!
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len() // s goes out of scope here, but the value it points to does not
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// This function will cause a compile error because it returns a reference to data that will be dropped
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // This would return a reference to s
// } // s goes out of scope and is dropped. Its memory goes away.