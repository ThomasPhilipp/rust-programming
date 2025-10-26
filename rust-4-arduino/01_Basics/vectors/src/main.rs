fn main() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(10);
    vec.push(20);
    vec.push(30);

    println!("Vec contents: {:?}", vec);
    println!("Vec length: {}", vec.len());
    
    let vec2: Vec<&str> = vec!["Arduino Uno", "Arduino Nano", "Arduino Mega"];
    println!("Vec2 contents: {:?}", vec2);
    println!("Vec2 length: {}", vec2.len());
    
    let search_term: &str = "Mega";
    let found: bool = vec2.iter().any(|s: &&str| s.contains(search_term));
    println!("Contains '{}': {}", search_term, found);
}
