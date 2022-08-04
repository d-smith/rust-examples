use std::io; //prelude

fn main() {
    println!("Guess the number");
    println!("Enter your guess");

    let mut guess = String::new(); //new mutable string var, vars immutable by default


    // read_line returns a Result enum, Ok and Err can be returned
    // if Err is returned expect will cause the program to crash and
    // print the given error message
    io::stdin()
        .read_line(&mut guess) // & for reference, no copying it around...
        .expect("Failed to read line");

    println!("You entered {guess}"); // {} are place holders for var vals in println
}
