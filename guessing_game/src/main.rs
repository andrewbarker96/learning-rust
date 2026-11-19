use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);
    let result = format!("The secret number was: {secret_number}.");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mut = mutable... allows variable to be changed

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Just like TS, we change the string to a number and parse extra spaces using trim().parse().
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Please type a number!"); continue;},
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win! {result}");
                break;
            }
        };

    }
}