// 13. Write a Rust function that takes two string references and returns the smallest one.

fn main() {
    // Define two string variables
    let string1: &str = "Soumalya Mondal";
    let string2: &str = "Soumalya";

    // Call the 'find_smallest' function and pass the string references
    let smallest: &str = find_smallest(string1, string2);

    println!("The smallest string is: {}", smallest);
}

fn find_smallest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    // Check if 'str1' is less than 'str2' lexicographically
    if str1 < str2 {
        str1
    } else {
        str2
    }
}