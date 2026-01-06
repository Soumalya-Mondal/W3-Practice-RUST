// 12. Write a Rust function that takes a reference to a variable as a parameter and modifies its value.

fn main() {
    let mut number: u8 = 12;
    println!("Previous Number Is: {number}");

    // Calling "modify_value" function
    modify_value(&mut number);

    println!("After Modification The New Number Is: {number}");
}

fn modify_value(value: &mut u8) {
    *value += 1
}