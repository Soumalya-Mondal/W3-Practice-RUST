// 9. Write a Rust program that prints all elements of an array using a for loop.

fn main() {
    // Define array
    let number_array: [u8; 6] = [1, 2, 3, 4, 5, 6];

    for number in number_array.iter() {
        println!("The Value Is : {number}");
    }
}
