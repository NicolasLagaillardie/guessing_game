// Input package
use std::io;

// Random package
use rand::Rng;

// Comparison Package
use std::cmp::Ordering;

// Name of the function
fn main(){
    println!("Guess the number");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 100);

    // Print it
    // println!("The secret number is: {}", secret_number);

    // loop too infinity
    loop {
        println!("Please input your guess.");

        // Declare var mutable which is a new String
        let mut guess = String::new();

        // Wait for input from user, read it and write it on the var guess
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

        // Try to transform guess into a 32-bits number, or ask for a new input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Using reference and pointer
        println!("You guessed: {}", guess);

        // Compare guess and secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                // Break the loop
                break;
            }
        }
    }
}
