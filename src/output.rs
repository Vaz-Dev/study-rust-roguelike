use std::io::stdout;

use crate::state::GameState;

pub fn output(state: &GameState) {
    if state.quit {
        println!("Thanks for playing!");
    }
}
