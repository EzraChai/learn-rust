use rand::Rng;
use std::{cmp::Ordering, io};
use colored:: Colorize;

fn main() {
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..101);
    let mut total_guesses:u8 = 0;

    println!("Guess the number");

    loop {
        println!("\nPlease input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err( _) => continue,
        };
        println!("You guessed: {}", guess);
        total_guesses += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too Small!".red()),
            Ordering::Greater => println!("{}","Too Big!".red()),
            Ordering::Equal => {
                println!("{}","You Win!".green());
                println!("You tried {} times to guess the correct number.", total_guesses);
                break;
            }
        }
    }
}
