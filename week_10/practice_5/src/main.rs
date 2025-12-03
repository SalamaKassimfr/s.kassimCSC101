fn main(){
    // a list of nos
    let x = vec![100, 200, 300]; // x owns the vector
    borrow_vector(&x);           // x is immutably borrowed
    
    // Now x is still valid and can be used:
    println!("Printing the value from main() x[0]={}", x[0]); // Corrected syntax
    println!("*******************************");
}

fn borrow_vector(z: &Vec<i32>){ // Function accepts a reference (borrow)
    println!("*******************************");
    println!("Inside print_vector function {:?} \n",z);
    println!("-------------------------------");
}