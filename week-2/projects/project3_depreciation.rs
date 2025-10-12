fn main() {
    // Given values
    let p: f64 = 10_000.0; // Initial price (₦10,000)
    let r: f64 = 3.0; // Depreciation rate (3%)
    let n: f64 = 3.0; // Number of years (3)

    // Calculate value after depreciation
    let a = p * (1.0 - (r / 100.0)).powf(n);

    // Display results
    println!("==== Depreciation Calculation ====");
    println!("Initial Value: ₦{:.2}", p);
    println!("Depreciation Rate: {}%", r);
    println!("Years: {}", n);
    println!("Value After Depreciation: ₦{:.2}", a);
}