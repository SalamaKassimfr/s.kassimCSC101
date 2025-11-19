fn main() {
    // create an empty vector "city"
    let mut city: Vec<String> = Vec::new();

    // print empty vector
    println!("The empty vector has element {}", city.len());

    // Push new elements into
    // prompt for the number of cities the user wants to enter
    let mut num = String::new();
    println!("How many cities do you want to enter?");
    std::io::stdin().read_line(&mut num).expect("Failed to read input");
    let num_cities: i32 = num.trim().parse().expect("Invalid input");

    for count in 0..num_cities {
        let mut input2 = String::new();
        println!("Enter city {}", count + 1);
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");

        let city_name: String = input2.trim().parse().expect("Invalid input");

        city.push(city_name);
    }

    print!("Your preferred cities are:\n");

    // print vector elements
    let mut count = 1;
    for c in city {
        // iterating through i on the vector
        println!("{} ({})", c, count);
        count += 1;
    }
}