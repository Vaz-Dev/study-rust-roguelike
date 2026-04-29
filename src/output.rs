use std::io::stdout;

use crate::state::GameState;

pub fn cli_output(state: &GameState) {
    if state.quit {
        println!("Thanks for playing!");
    } else if state.startup {
        println!("Welcome to my mini rogue-like project (a Rust study)");
    }
}
