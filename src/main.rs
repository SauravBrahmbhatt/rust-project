use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    loop {
        println!("please enter a number:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Less => println!("{}", "Too Small".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green().bold());
                break;
            }
        }
    }
}
