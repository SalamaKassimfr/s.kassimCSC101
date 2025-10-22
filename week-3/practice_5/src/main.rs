
// Rust program to output name and age

use std::io;

fn main() {
    println!("\nStudent Information Management System!");

    // Input name
    println!("\nPlease enter your name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    let name = name.trim(); // remove newline characters
    println!("Your name is: {}", name);

    // Input age
    println!("\nEnter your age:");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read input");
    let age: i32 = age.trim().parse().expect("Input not an integer");
    println!("Your age is: {}", age);
}