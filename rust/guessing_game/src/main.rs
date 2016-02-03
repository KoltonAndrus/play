// Rust
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    let mut guesses_taken = 0;

    //println!("The secret number is: {}", secret_number);

    loop {

        println!("Please input your guess between 1 and 100.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        guesses_taken = guesses_taken + 1;

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal    => {
                println!("You win! It took you {} guesses.", guesses_taken);
                break;
            }
        }
    }
}
