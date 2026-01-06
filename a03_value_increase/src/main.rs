// 3. Write a Rust program that declares a mutable variable counter and initializes it with 0. Then increase it by 1 and decrease it by 1. At the end, print the variable value for each stage.

fn main() {
    let mut value: i8 = 0;
    println!("Initial Value: {value}");

    value += 1;
    println!("Value Increased By 1: {value}");

    value -= 1;
    println!("Value Decreased By 1: {value}");
}
