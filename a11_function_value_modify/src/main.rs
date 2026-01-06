// 12. Write a Rust function that takes a reference to a variable as a parameter and modifies its value.

fn main() {
    let mut number: u8 = 12;

    // Calling "modify_value" function
    modify_value(&mut number);

    println!("The Modified Value Is: {number}");
}

fn modify_value(value: &mut u8) {
    *value += 1;
}
