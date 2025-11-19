fn main() {
    // Name vector
    let name = vec!["Laila", "Cameron", "Niana", "Laura", "Halley", "Pierre", "Quen", "Billie"];

    // Age vector
    let age = vec![16, 17, 19, 22, 20, 21, 18, 23];

    // print age allocation
    print!("AGE allocation: \n");

    // loop to iterate elements in vector
    for i in 0..age.len() {
        // iterating through i on the vector
        print!("{} is {} years old\n", name[i], age[i]);
    }
}