extern crate rand;
use std::io; // prelude
use std::cmp::Ordering;
use rand::Rng; // Rng is a trait, whatever that means

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);
    println!("Please input your guess.");

    let mut guess = String::new();
    // variables are immutable by default
    // new is an associated function of the String type
    // An associated function is implemented on a type
    // Some languages call this a static method

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too large!"),
        Ordering::Equal => println!("You win!"),
    }
}
