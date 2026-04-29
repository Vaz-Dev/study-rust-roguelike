use crate::{
    engine::{engine, GameState},
    input::cli_input,
    output::cli_output,
};

mod engine;
mod input;
mod output;

fn main() {
    println!("Welcome to my mini rogue-like project (a Rust study)");
    let mut game_loop = true;
    let mut prev_state: GameState = GameState {
        quit: false,
        startup: true,
    };
    while game_loop {
        let input = cli_input();
        let state: GameState = engine(input, prev_state);
        if state.quit {
            game_loop = false;
        }
        cli_output(&state);
        prev_state = state;
    }
    println!("Thanks for playing!")
}
