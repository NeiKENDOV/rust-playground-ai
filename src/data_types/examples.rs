// Examples of data types in Rust

pub fn data_types_demo() {
    // Scalar types
    let integer: i32 = -42;
    let unsigned: u32 = 42;
    let floating: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'z';
    
    // Compound types - Tuple
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple; // destructuring
    println!("tuple values: {}, {}, {}", x, y, z);
    
    // Access tuple elements by index
    let first = tuple.0;
    
    // Compound types - Array
    let array = [1, 2, 3, 4, 5];
    let first_element = array[0];
    println!("first element = {}", first_element);
    
    // Array with type and size
    let explicit_array: [i32; 5] = [1, 2, 3, 4, 5];
}