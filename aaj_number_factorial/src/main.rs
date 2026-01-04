// 10. Write a Rust program that defines a function that calculates the factorial of a given number and returns the result.

use std::io::{self, Write};

fn main() {
    // Taking number input from user
    print!("Enter Any Number: ");
    io::stdout().flush().unwrap();
    let mut number: String = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed To Read Lines");
    let number: u8 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter Valid Number!!");
            return;
        }
    };

    // Calling "factorial()" function
    println!("The Factorial Of Number: {number} Is {}", factorial(number));
}

// Creating factorial function
fn factorial(n: u8) -> u128 {
    let mut result: u128 = 1;
    for i in 1..=n {
        result *= i as u128;
    }
    result
}
