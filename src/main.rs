use rand::Rng; // Imports the Rng Trait
use std::cmp::Ordering; // Imports cmp library from the standard library std
use std::io; // Imports io library from the standard library std

fn main() {
    println!("Guess the number! You only have 5 tries.");

    // Generate random number from 1 to 100
    let number = rand::thread_rng().gen_range(1..=100);

    let mut tries = 5;

    // Use loop to start an infinite loop
    loop {

        // Add losing/stop condition
        if tries < 1 {
            println!("Game Over!");
            break;
        }

        println!("Enter your guess! You have {tries} tries remaining");

        let mut guess = String::new(); // Creates a String object called guess

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to accept input");

        // Convert guess to unsigned integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input numbers only!");
                continue;
            },
        };

        // Compare the random number and user's guess
        match guess.cmp(&number) {
            Ordering::Equal => {
                println!("Correct! You win.");
                break;
            }
            Ordering::Greater => {
                tries -= 1;
                println!("Too big!");
            }
            Ordering::Less => {
                tries -= 1;
                println!("Too small!")
            }
        }
    }
}
