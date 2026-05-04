use crate::{engine::engine, input::input, output::output, state::GameState};

mod engine;
mod input;
mod output;
mod state;

fn main() {
    let mut prev_state = GameState::new();
    loop {
        let input = input(&prev_state);
        let state = engine(input, prev_state);
        output(&state);
        if state.quit {
            break;
        }
        prev_state = state;
    }
}
