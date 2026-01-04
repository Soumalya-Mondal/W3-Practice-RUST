// 6. Write a Rust program that checks if a number is even or odd and prints the result.

use std::io::{self, Write};
fn main() {
    // Taking number from user
    print!("Enter A Number: ");
    io::stdout().flush().unwrap();
    let mut number: String = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed To Read Lines");
    let number: u128 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter Valid Number!!");
            0
        }
    };

    // Check if number is even or odd
    if number % 2 == 0 {
        println!("The Number: {number} Is Even");
    } else {
        println!("The Number: {number} Is Odd");
    }
}
