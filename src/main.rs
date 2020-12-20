mod game;

fn main() {
    games_menu();
}

fn games_menu(){
    use terminal_menu::*;

    //create the menu
    let menu = menu(vec![
        label("\n"),
        label("------------------------------"),
        label("|         GAMES MENU         |"),
        label("------------------------------"),
        label("(Use arrow keys or wasd to select an option)"),
        scroll("Select Game", vec!["Guessing Game", "Exit"]),
        button("Confirm")
    ]);

    //open the menu
    activate(&menu);

    //wait for the menu to exit
    wait_for_exit(&menu);
    
    match selection_value(&menu, "Select Game").as_str(){
        "Guessing Game" => run_game(String::from("guessing")),
        _ => exit_games()
    }
}

fn run_game(game_type: String){
    use game::*;
    let curent_game = Game{game_type: game_type};
    if curent_game.play(){
        games_menu();
    }
}

fn exit_games(){
    use std::process::exit;
    println!("\nExiting!\n");
    exit(0);
}
