// 11. Write a Rust program that creates two variables p and q, assigns a value to p, then assigns p to q and try to use p again.
#![allow(unused_variables)]

fn main() {
    // Declare variable
    let p: u8 = 123;

    // Declare another variable
    let q: u8 = p;

    println!("The Value Of p Is: {p}");
}