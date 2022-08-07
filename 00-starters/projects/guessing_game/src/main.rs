use std::io; //prelude
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Guess the number");
        println!("Enter your guess");

        let mut guess = String::new(); //new mutable string var, vars immutable by default


        // read_line returns a Result enum, Ok and Err can be returned
        // if Err is returned expect will cause the program to crash and
        // print the given error message
        io::stdin()
            .read_line(&mut guess) // & for reference, no copying it around...
            .expect("Failed to read line");

    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
