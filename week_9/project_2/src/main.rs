use std::io::Write;
use std::fs::File;
use std::path::Path;

// --- 1. Function to define the data structure ---
// Uses tuples (category, drinks_vector) and a main Vector.
fn get_drink_data() -> Vec<(&'static str, Vec<&'static str>)> {
    // Data extracted from the table in the image
    let lager_drinks = vec![
        "33 Export", 
        "Desperados", 
        "Goldberg", 
        "Gulder", 
        "Heineken", 
        "Star"
    ];
    
    let stout_drinks = vec![
        "Legend", 
        "Turbo King", 
        "Williams"
    ];
    
    let non_alcoholic_drinks = vec![
        "Maltina", 
        "Amstel Malta", 
        "Malta Gold", 
        "Fayrouz"
    ];

    // Returns a Vector of Tuples: (Category Name, Vector of Drinks)
    vec![
        ("Lager", lager_drinks),
        ("Stout", stout_drinks),
        ("Non-Alcoholic", non_alcoholic_drinks),
    ]
}

// --- 2. Function to write data to file ---
fn write_categories_to_file(data: Vec<(&'static str, Vec<&'static str>)>) -> Result<(), std::io::Error> {
    let file_name = "drink_categories.txt";
    let path = Path::new(file_name);
    
    // Create or open the file for writing
    let mut file = match File::create(&path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    // Iterate through the Vector of Tuples
    for (category, drinks) in data {
        // Write the category heading
        let category_line = format!("## {} ({} varieties)\n", category, drinks.len());
        file.write_all(category_line.as_bytes())?;
        
        // Write each drink on a new line
        for drink in drinks {
            let drink_line = format!("- {}\n", drink);
            file.write_all(drink_line.as_bytes())?;
        }
        
        // Add a separator for clarity
        file.write_all(b"\n---\n")?; 
    }

    Ok(()) // Return Ok(()) on success
}

// --- 3. Main execution function ---
fn main() {
    let drink_data = get_drink_data();
    
    println!("Starting file creation for drink categories...");
    
    match write_categories_to_file(drink_data) {
        Ok(_) => println!("\n✅ Success! All drink categories were written to: drink_categories.txt"),
        Err(e) => eprintln!("\n❌ Error writing to file: {}", e),
    }
}