// 2. Write a Rust program that accepts two numbers from the user, adds them together, and displays the result.

use std::io::{self, Write};

fn main() {
    // Taking number_1 input from user
    print!("Enter First Number: ");
    io::stdout().flush().unwrap();
    let mut number_1: String = String::new();
    io::stdin()
        .read_line(&mut number_1)
        .expect("Failed To Read Number");
    let number_1: f32 = match number_1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter Valid Number!!");
            0.0
        }
    };

    // Taking number_2 input from user
    print!("Enter Second Number: ");
    io::stdout().flush().unwrap();
    let mut number_2: String = String::new();
    io::stdin()
        .read_line(&mut number_2)
        .expect("Failed To Read Number");
    let number_2: f32 = match number_2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter Valid Number!!");
            0.0
        }
    };

    // Adding two number
    println!("Sum Of {number_1} And {number_2} Is: {}", number_1 + number_2);
}