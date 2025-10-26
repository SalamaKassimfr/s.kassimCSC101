use std::io;

fn main() {
    println!("--- Employee Incentive Calculator ---");

    // Get Age Input
    println!("Enter employee's age:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).unwrap();
    let age: i32 = age_input.trim().parse().unwrap(); 

    // Get Experience Input ('yes' or 'no')
    println!("Is the employee experienced (type 'yes' or 'no'):");
    let mut exp_input = String::new();
    io::stdin().read_line(&mut exp_input).unwrap();
    let is_experienced = exp_input.trim().to_lowercase() == "yes";

    let incentive: f64;

    // Decision Logic based on criteria
    if is_experienced {
        // Experienced Path
        
        if age >= 40 {
            // Incentive: N1,560,000
            incentive = 1_560_000.0; 

        } else if age >= 30 && age < 40 {
            // Incentive: N1,480,000
            incentive = 1_480_000.0;
        
        } else if age < 28 {
            // Incentive: N1,300,000
            incentive = 1_300_000.0;
        
        } else {
            // Experienced AND Age 28 or 29 (Assigning the N1.48m rate)
            incentive = 1_480_000.0;
        }

    } else {
        // Inexperienced Path (Incentive: N100,000)
        incentive = 100_000.0;
    }

    // Output Result
    println!("\nThe annual incentive is: N{:.2}", incentive);
    println!("(Age: {}) (Experienced: {})", age, is_experienced);
}