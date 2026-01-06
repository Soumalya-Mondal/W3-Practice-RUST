// 4. Write a Rust program that converts an integer to a string and vice versa and prints the result.

fn main() {
    // Convert int to string
    let int_value_1: i8 = 123;
    let string_value_1: String = int_value_1.to_string();
    println!("The String Value Is: {}", string_value_1);

    // Convert string to int
    let string_value: &str = "123";
    // Specify the target type when parsing so Result's generics are known
    let parsed_integer: Result<i8, std::num::ParseIntError> = string_value.parse::<i8>();
    match parsed_integer {
        Ok(int_value) => println!("String to integer: {}", int_value),
        Err(e) => println!("Failed to parse string to integer: {}", e),
    }
}
