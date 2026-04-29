use crate::{engine::engine, input::cli_input, output::cli_output, state::GameState};

mod engine;
mod input;
mod output;
mod state;

fn main() {
    let mut game_loop = true;
    let mut prev_state = GameState::new();
    cli_output(&prev_state);
    while game_loop {
        let input = cli_input(&prev_state);
        let state = engine(input, prev_state);
        if state.quit {
            game_loop = false;
        }
        cli_output(&state);
        prev_state = state;
    }
}
