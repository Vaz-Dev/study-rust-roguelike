use std::io::stdin;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::state::GameState;

pub enum Input {
    NewGame(String),
    Quit,
    Save,
    Load(String),
}

pub fn input(state: &GameState) -> Input {
    // if state.menu | state.startup {
    cli_input(state)
    // } else {
    // tui_input(state)
    // }
}

fn cli_input(state: &GameState) -> Input {
    if state.startup {
        println!("Welcome to my mini rogue-like project (a Rust study)");
    }

    println!("Menu options ->");
    println!("new -> Start new game");
    println!("load -> Load save");
    println!("save -> Save your current game");
    println!("quit -> Quit game");

    loop {
        let input = prompt("");

        match input.to_lowercase().trim() {
            "new" => match confirm(
                "start a new game? unsaved progress will be lost and this save name could be overwritten.",
                Input::NewGame(prompt("Name your new game")),
            ) {
                Some(confirmed) => return confirmed,
                None => continue,
            },
            "quit" => match confirm(
                "quit the game? all unsaved progress will be lost.",
                Input::Quit,
            ) {
                Some(confirmed) => return confirmed,
                None => continue,
            },
            "save" => return Input::Save,
            "load" => {
                let file_name = prompt("what save file do you want to load?");
                let confirm_message = format!(
                    "load the saved game {file_name}? this can override your current game."
                );
                match confirm(&confirm_message, Input::Load(file_name)) {
                    Some(confirmed) => return confirmed,
                    None => continue,
                }
            }
            _ => {
                println!("Input not matched, try again: {}", input);
                continue;
            }
        }
    }
}

fn tui_input(state: &GameState) -> Input {
    enable_raw_mode();
    disable_raw_mode();
    todo!();
}

fn confirm(message: &str, result: Input) -> Option<Input> {
    loop {
        println!("Are you sure you want to {} (y/n)", &message);
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line!");
        return match input.to_lowercase().trim() {
            "y" | "yes" | "ya" | "yeah" => Some(result),
            "n" | "no" | "nah" | "nope" => None,
            _ => continue,
        };
    }
}

fn prompt(message: &str) -> String {
    println!("{}", message);

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line!");

    String::from(input.to_lowercase().trim())
}
