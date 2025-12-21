use std::io;
use std::cmp::Ordering; // Import the standard library for input/output and comparison
use rand::Rng; // Import the standard input/output library

fn main() {// Main function - entry point of the program
    println!("Guess the number!");
    
    let secret_number = rand::rng().random_range(1..=100);// Generate a random number between 1 and 100
    println!("The secret number is: {secret_number}"); // For debugging purposes, print the secret number

    println!("Please input your guess.");//macro to print text to the console

    let mut guess = String::new();// Create a mutable variable 'guess' to store user input
    //let guess: String; // Immutable variable example

    io::stdin()//stdin function to handle standard input from io module
        .read_line(&mut guess)//to read a line from standard input and store it in 'guess' the &mut guess indicates that we are passing a mutable reference to the variable
        .expect("Failed to read line");//error handling in case reading input fails
    let guess:i32 = guess.trim().parse().expect("Please type a number!");// Convert the input string to an unsigned 32-bit integer

    println!("You guessed: {guess}");
    match guess.cmp(&secret_number) {// Compare the guess with the secret number
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
        
    }
}
