/*
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
*/

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng()
        .gen_range(1, 101);

    println!("Secret number is {}.", &secret_number);

    println!("Guessing game");

    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Please type a number!");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Your guess is too low!"),
        Ordering::Equal => println!("Your guess is correct!"),
        Ordering::Greater => println!("Your guess is too big!")
    }
}
