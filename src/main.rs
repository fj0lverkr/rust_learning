use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    //later we will put a menu to allow user to choose a game
    game_guess();
}

fn game_guess(){
    let secret_number = rand::thread_rng()
        .gen_range(1, 101);

    let mut number_guesses = 0;

    println!("Guessing game");
    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess
            .trim()
            .parse(){
                Ok(num) => num,
                Err(_) => continue
            };
        
        number_guesses += 1;

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Your guess is too low!"),
            Ordering::Equal => {
                println!("You guessed correct in {} guesses.", number_guesses);
                break;
            }
            Ordering::Greater => println!("Your guess is too big!")
        }
    }
}
