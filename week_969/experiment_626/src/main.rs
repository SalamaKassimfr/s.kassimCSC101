//Rust program to calculates average of students test scores📝
use std::io;
fn main() 
{
    let mut input = String::new();
    println!("\nEnter students name:");
     io::stdin().read_line(&mut input).expect("Failed to read input");
 //Ask for three scores
     let mut score1 = String::new();
     let mut score2 = String::new();
     let mut score3 = String::new();

    println!("Enter first score:");
     io::stdin().read_line(&mut score1).expect("Failed to read input");

    println!("Enter second score:");
     io::stdin().read_line(&mut score2).expect("Failed to read input");

    println!("Enter third score:");
     io::stdin().read_line(&mut score3).expect("Failed to read input");

 //Convert scores to numbers1️⃣
     let score1: f64 = score1.trim().parse().expect("Invalid input!Please enter a number");
     let score2: f64 = score2.trim().parse().expect("Invalid input!Please enter a number");
     let score3: f64 = score2.trim().parse().expect("Invalid input!Please enter a number");

 //Calculate average
     let average = (score1 + score2 + score3)/3.0;
    printline!("Average score = {}",average);//Shows two decimal place

// Determine grade
    if average >= 70.0 {
        println!("Grade: A");
    } else if average >= 60.0 {
        println!("Grade: B");
    } else if average >= 50.0 {
        println!("Grade: C");
    } else if average >= 45.0 {
        println!("Grade: D");
    } else {
        println!("Grade: F");
    }

}