// Examples of collections in Rust

pub fn collections_demo() {
    // Vector
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    
    // Access elements
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    // Iterating
    for i in &v {
        println!("{}", i);
    }
    
    // Mutable iteration
    for i in &mut v {
        *i += 50;
    }
    
    // String (growable, UTF-8 encoded)
    let mut s = String::new();
    
    let data = "initial contents";
    let s = data.to_string();
    // or
    let s = "initial contents".to_string();
    
    // Using the + operator (actually calls the add method)
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    
    // Using format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    
    // Hash Map
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // Access values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    
    match score {
        Some(s) => println!("{} team score: {}", team_name, s),
        None => println!("Team not found"),
    }
    
    // Iterating over hash maps
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}