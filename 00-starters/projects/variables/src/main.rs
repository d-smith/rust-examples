fn main() {
    //Variables are immuable by default 
    let x = 5;
    println!("The value of x is: {x}");
    
    // Uncommenting the assignment 6 to results in an 
    // "cannot assign twice to immutable variable" errpr
    //x = 6;
    //println!("The value of x is: {x}");

    // mut keyword allows mutability
    let mut y = 5;
    println!("The value of y is: {y}");

    y = 6;
    println!("The value of y is: {y}");

    // constants - always immutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    
}
