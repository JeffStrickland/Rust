use std::io; // Standar library io - input/output
use rand::Rng; // Random number generator from imported crate, added to dependencies in the Cargo.toml file
use std::cmp::Ordering; // Less, Greater, Equal

fn main() { // Main function
    println!("Guess the number!"); // Printing with macro indictated by !

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number); // Used for testing

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // Creating variable to store user input using the let statement
                                        // Variables are immutable by default, mut makes it mutable
        io::stdin()
            .read_line(&mut guess)  // & Indicates arguement is a reference
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Shadowing, often used to convert value from one type to anther

        println!("You guessed: {}", guess);

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
