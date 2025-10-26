fn main() {
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];
    let mut sum: i32 = 0;

    for &num in &numbers {
        sum += num;
    }

    println!("The sum of the array elements is: {}", sum);
}
