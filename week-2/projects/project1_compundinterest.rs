// Project I: Compound Interest Calculator
// Formula: A = P[1 + (R/100)]^n
// CI = A - P

fn main() {
    // Given Values
    let principal: f64 = 520_000_000.0; // Loan amount (P)
    let rate: f64 = 10.0; // Interest rate (R)
    let time: f64 = 5.0; // Number of years (n)

 // Calculation
 // 1. Calculate the base factor: [1 + (R/100)]
    let rate_factor: f64 = 1.0 + (rate / 100.0);
 // 2. Calculate the Final Amount (A): P * (rate_factor)^n
 // The .powf() function is used for exponentiation.
    let amount: f64 = principal * rate_factor.powf(time);

 // 3. Calculate the Compound Interest (CI)
    let compound_interest: f64 = amount - principal;

 // Output Results
    println!("--- Project I: Compound Interest ---");
    println!("Principal (P): \t\tN{:.2}", principal);
    println!("Rate (R): \t\t{:.0}%", rate);
    println!("Time (n): \t\t{:.0} years", time);
    println!("------------------------------------");
    println!("Final Amount (A): \tN{:.2}", amount);
    println!("Compound Interest (CI): N{:.2}", compound_interest);
}