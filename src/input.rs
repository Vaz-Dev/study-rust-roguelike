use std::io::stdin;

use crate::state::GameState;

pub enum Input {
    NewGame,
    Quit,
}

fn confirm(message: &str, result: Input) -> Option<Input> {
    loop {
        println!("Are you sure you want to {}?", &message);
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line!");
        return match input.to_lowercase().trim() {
            "y" => Some(result),
            "n" => None,
            _ => continue,
        };
    }
}

pub fn cli_input(prev_state: &GameState) -> Input {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line!");

        match input.to_lowercase().trim() {
            "new" => match confirm("start a new game", Input::NewGame) {
                Some(confirmed) => return confirmed,
                None => continue,
            },
            "quit" => match confirm("quit the game", Input::Quit) {
                Some(confirmed) => return confirmed,
                None => continue,
            },
            _ => {
                println!("Input not matched, try again: {}", input)
            }
        }
    }
}
