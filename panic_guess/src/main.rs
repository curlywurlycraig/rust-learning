extern crate rand;

mod guess;

use guess::{Guess, GuessErr};
use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game! Please guess a number between 0 and 100");

    let number: i32 = thread_rng().gen_range(0, 101);

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Please enter valid input.");
        let input: i32 = input.trim().parse().expect("That doesn't look like a valid number.");
        let guess: i32 = match Guess::new(input as i32) {
            Ok(value) => value.value(),
            Err(GuessErr::TooLow) => {
                println!("Your guess must be greater than 0!");
                continue;
            },
            Err(GuessErr::TooHigh) => {
                println!("Your guess must be less than 101");
                continue;
            }
        };

        match guess.cmp(&number) {
            Ordering::Less => println!("That number is too low!"),
            Ordering::Greater => println!("That number is too high!"),
            Ordering::Equal => break
        }
    }

    println!("Congratulations you mad man! The number was indeed {}", number);
}
