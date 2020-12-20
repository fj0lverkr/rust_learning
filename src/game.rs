pub struct Game{
    pub game_type: String
}

pub trait Play{
    fn play(&self) -> bool;
}

pub trait Replay{
    fn replay(&self) -> bool;
}

impl Play for Game{
    fn play(&self) -> bool{
        match self.game_type.as_str(){
            "guessing" => run_guessing_game(),
            _ => std::process::exit(0)
        }
        if self.replay(){
            return self.play();
        }
        return true;
    }
}

impl Replay for Game{
    fn replay(&self) -> bool{
        use terminal_menu::*;
        let menu = menu(vec![
            label("\n"),
            label("------------------------------"),
            label("|         GAMES MENU         |"),
            label("------------------------------"),
            label("(Use arrow keys or wasd to select an option)"),
            list("Play Again?", vec!["Yes", "No"]),
            button("Confirm")
        ]);
        activate(&menu);

        wait_for_exit(&menu);

        let selection = selection_value(&menu, "Play Again?");

        if selection == "Yes"{
            return true;
        }else{
            return false;
        }

    }
}

fn run_guessing_game(){
    use std::io;
    use rand::Rng;
    use std::cmp::Ordering;

    let secret_number = rand::thread_rng()
        .gen_range(1, 101);

    let mut number_guesses = 0;

    println!("-------------------------------");
    println!("|        GUESSING GAME        |");
    println!("-------------------------------\n");
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
            Ordering::Less => println!("\nYour guess is too low!\n"),
            Ordering::Equal => {
                println!("\nYou guessed correct in {} guesses.", number_guesses);
                break;
            }
            Ordering::Greater => println!("\nYour guess is too high!\n")
        }
    }
}