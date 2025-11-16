use std::io;

fn main() {
    println!("🌟 MTH 101 Geometry Calculator 🌟");

    // Main loop for repeated calculations
    loop {
        display_menu();

        // Reading input using the helper function
        let choice = get_user_input("Enter your selection (1-4) or 'q' to quit: ");

        if choice.to_lowercase().trim() == "q" {
            println!("\n👋 Goodbye!");
            break;
        }

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("🚨 Invalid input. Please enter a number (1-4) or 'q'.");
                continue;
            }
        };

        // Perform the selected calculation
        match choice {
            1 => calculate_trapezium_area(),
            2 => calculate_rhombus_area(),
            3 => calculate_parallelogram_area(),
            4 => calculate_3d_calculations(), // Grouping the two 3D formulas
            _ => println!("🚨 Selection out of range. Please choose from 1 to 4."),
        }
        
        println!("\n--------------------");
    }
}

// --- Helper Functions ---

/// Displays the calculation menu to the user.
fn display_menu() {
    println!("\n## Select a Calculation:");
    println!("* **1.** Area of Trapezium");
    println!("* **2.** Area of Rhombus");
    println!("* **3.** Area of Parallelogram");
    println!("* **4.** 3D Shapes (Cube Area & Cylinder Volume)");
}

/// Reads a line of input from the user.
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    let mut input = String::new();
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

/// Prompts the user for a single f64 value.
fn read_dimension(dimension_name: &str) -> f64 {
    loop {
        let input = get_user_input(&format!("Enter the {}: ", dimension_name));
        match input.trim().parse::<f64>() {
            Ok(num) if num >= 0.0 => return num,
            _ => println!("🚫 Invalid input. Please enter a non-negative number."),
        }
    }
}

// --- Calculation Functions ---

/// Calculates and prints the Area of a Trapezium.
fn calculate_trapezium_area() {
    println!("\n🔢 **Area of Trapezium** ( $h/2 \times (b_1 + b_2)$ )");
    let h = read_dimension("height (h)");
    let b1 = read_dimension("base 1 ($b_1$)");
    let b2 = read_dimension("base 2 ($b_2$)");

    let area = (h / 2.0) * (b1 + b2);
    
    println!("✅ Area: **{:.2}**", area);
}

/// Calculates and prints the Area of a Rhombus.
fn calculate_rhombus_area() {
    println!("\n🔢 **Area of Rhombus** ( $1/2 \times d_1 \times d_2$ )");
    let d1 = read_dimension("diagonal 1 ($d_1$)");
    let d2 = read_dimension("diagonal 2 ($d_2$)");

    let area = 0.5 * d1 * d2;
    
    println!("✅ Area: **{:.2}**", area);
}

/// Calculates and prints the Area of a Parallelogram.
fn calculate_parallelogram_area() {
    println!("\n🔢 **Area of Parallelogram** ( Base $\times$ Altitude )");
    let base = read_dimension("base");
    let altitude = read_dimension("altitude (height)");

    let area = base * altitude;
    
    println!("✅ Area: **{:.2}**", area);
}

/// Performs both the Cube Area and Cylinder Volume calculations.
fn calculate_3d_calculations() {
    // --- Area of Cube ---
    println!("\n--- Area of Cube --- ( $6 \times side^2$ )");
    let side = read_dimension("side length of the cube");
    let cube_area = 6.0 * side.powi(2);
    println!("✅ Cube Area: **{:.2}**", cube_area);

    // --- Volume of Cylinder ---
    const PI: f64 = std::f64::consts::PI; 
    println!("\n--- Volume of Cylinder --- ( $\\pi \times radius^2 \times height$ )");
    let radius = read_dimension("radius of the cylinder");
    let height = read_dimension("height of the cylinder");

    let cylinder_volume = PI * radius.powi(2) * height;

    println!("✅ Cylinder Volume: **{:.2}**", cylinder_volume);
}