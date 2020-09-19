extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!\n");
    
    // generate a secret random number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        // get the guess
        println!("Input guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {}", guess);

        // wow! matchers are pretty damn good
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid number!");
                continue;
            }
        };

        // compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("That number is too low!"),
            Ordering::Equal => {
                println!("Yeah! It's the right number!");
                break;
            },
            Ordering::Greater => println!("That number is too high!"),
        }
    }
}
