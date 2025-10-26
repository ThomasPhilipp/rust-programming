fn main() {
    let mut my_string: String = String::from("Hello ..."); // String type
    println!("String: {}", my_string);
    my_string = String::from("Heap ...");
    println!("String: {}", my_string);
    my_string = String::from("Memory ...");
    println!("String: {}", my_string);

    let rev_string: String = my_string.chars().rev().collect();
    println!("Reversed String: {}", rev_string);
}
