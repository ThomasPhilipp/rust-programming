fn main() {
    
    struct Point {
        x: i32,
        y: i32,
        v: String,
    }

    let p: Point = Point {
        x: 10,
        y: 10,
        v: String::from("Hello, Arduino Uno!"),
    };

    println!("x: {}, y: {} and v: {}", p.x, p.y, p.v);
}
