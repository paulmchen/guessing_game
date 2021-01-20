use rand::Rng;

use std::cmp::Ordering;
use std::io;

// Guess a number
// The range of a number to be guessed is between values of 'lower' and 'higher' parameters (inclusive) specified
fn guess_number(lower: u32, higher: u32) {
    if lower > higher {
        println!("Incorrect number range specified.");
        return;
    }

    let secret_number = rand::thread_rng().gen_range(lower, higher + 1);
    let mut attempt: u32 = 0;

    loop {
        println!(
            "Please input your guess (a number between {} and {})：",
            lower, higher
        );
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! (after {} attempts)", attempt);
                break;
            }
        }
        attempt += 1;
    }
}

fn main() {
    println!("Guess the number!");
    guess_number(1, 100);
}
