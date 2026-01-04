// 8. Write a Rust program that prints all even numbers from 1 to user number using loop.

use std::io::{self, Write};

fn main() {
    // Taking upper_number input from user
    print!("Enter Upper Bound Number: ");
    io::stdout().flush().unwrap();
    let mut upper_number: String = String::new();
    io::stdin()
        .read_line(&mut upper_number)
        .expect("Failed To Read Lines");
    let upper_number: u128 = match upper_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter Valid Number!!");
            0
        }
    };

    for number in 1..=upper_number {
        if number % 2 == 0 {
            println!("The Number: {number} Is Even");
        }
    }
}
