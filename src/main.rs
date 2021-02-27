// use std::io::prelude::*;
use std::io;

use rand::Rng;
use std::cmp::Ordering;
use std::io::Write;
use std::process::exit;

fn main() {
    println!("Guess the number (1..255)!");

    let secret_number: u8 = rand::thread_rng().gen_range(1..=255);

    let mut number_of_tries: u8 = 0;

    loop {
        number_of_tries = match number_of_tries.checked_add(1) {
            Some(v) => v,
            None => {
                // Enough is enough :))
                // Change type for more tries (ie. u32, u64)
                println!("Too many attempts. It was {}", secret_number);
                exit(0);
            }
        };
        print!("[#{:3}] Please input your guess: ", number_of_tries);
        io::stdout().flush().expect("Could not write");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!();


        let guess: u8 = match guess.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win after {} tries!", number_of_tries);
                break;
            }
        }
    }
}
