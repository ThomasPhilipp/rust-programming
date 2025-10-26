fn main() {
    let my_int = 42; // Integer type
    println!("Integer: {}", my_int);

    let my_float: f32 = 3.14; // Floating-point type
    println!("Float: {}", my_float);

    let my_char = 'R'; // Character type
    println!("Character: {}", my_char);

    let my_byte: u8 = 255; // Byte type
    println!("Byte: {}", my_byte);

    let my_bool = true; // Boolean type
    println!("Boolean: {}", my_bool);

    let my_string1: &str = "Hello Stack (immutable & faster) Memory!"; // String type
    println!("String: {}", my_string1); // String slice type

    let my_string2: String = String::from("Hello Heap (mutable & slower) Memory!"); // String type
    println!("String: {}", my_string2);

    let my_tupples: (i32, f32, char) = (42, 3.14, 'B'); // Tuple type
    println!("Tupples #1: {:?}", my_tupples);
    println!("Tupples #2: ({}, {}, {})", my_tupples.0, my_tupples.1, my_tupples.2);

}
