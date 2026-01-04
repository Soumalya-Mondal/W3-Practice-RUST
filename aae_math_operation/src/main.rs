// 5. Write a Rust program that performs basic Math operations - addition, subtraction, multiplication, and division operations on two integers.

use std::io::{self, Write};

fn main() {
    // Taking number_1 from user
    print!("Enter First Number: ");
    io::stdout().flush().unwrap();
    let mut number_1: String = String::new();
    io::stdin()
        .read_line(&mut number_1)
        .expect("Failed To Read Lines");
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
        .expect("Failed To Read Lines");
    let number_2: f32 = match number_2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter Valid Number!!");
            0.0
        }
    };

    // Addition
    println!("{number_1} + {number_2} = {}", number_1 + number_2);

    // Subtraction
    println!("{number_1} - {number_2} = {}", number_1 - number_2);

    // Multiplication
    println!("{number_1} * {number_2} = {}", number_1 * number_2);

    // Division
    println!("{number_1} / {number_2} = {}", number_1 / number_2);
}
