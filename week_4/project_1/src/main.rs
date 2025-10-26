use std::io;
fn main() {
    println!("--- Quadratic Equation Solver (ax^2 + bx + c = 0) ---");

    // Input values for a, b, and c
    let a = read_input("Enter a: ");
    let b = read_input("Enter b: ");
    let c = read_input("Enter c: ");

    if a == 0.0 {
        println!("\n'a' cannot be zero. This is a linear, not a quadratic, equation.");
        return;
    }

    // Find the discriminant: Δ = b^2 - 4ac
    let discriminant = b * b - 4.0 * a * c;

    println!("\nDiscriminant (Δ) is: {}", discriminant);
    
    // Determine the nature of the roots using if/else if/else
    if discriminant > 0.0 {
        // Case 1: Two distinct real roots
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        
        println!("Result: Two distinct real roots.");
        println!("Root 1: {}", root1);
        println!("Root 2: {}", root2);

    } else if discriminant == 0.0 {
        // Case 2: Exactly one real root
        let root = -b / (2.0 * a);
        
        println!("Result: Exactly one real root.");
        println!("Root: {}", root);

    } else {
        // Case 3: No real roots (discriminant < 0)
        println!("Result: No real roots (roots are complex).");
    }
}

// Helper function to read a single floating-point value
fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    
    // Read the line from stdin
    io::stdin().read_line(&mut input).unwrap(); 
    
    // Trim whitespace and parse as a float (f64).
    // Using .unwrap() here keeps the code simple for a basic assignment.
    input.trim().parse::<f64>().unwrap()
}