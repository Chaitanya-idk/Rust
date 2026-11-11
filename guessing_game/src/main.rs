use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to guessing game.");

    let random = rand::rng().random_range(1..=10);
    let mut input = String::new();
    loop {
        input.clear();
        println!("Enter your input between 1 and 10: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to load line");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if !(1..=10).contains(&input) {
            println!("Please enter a number between 1 and 10!");
            continue;
        }
        match input.cmp(&random) {
            Ordering::Less => println!("The number is less !"),
            Ordering::Greater => println!("The number is greater !"),
            Ordering::Equal => {
                print!("The Guessed number is correct !");
                break;
            }
        }
    }
}
