use crossterm::event::EnableMouseCapture;

use crate::{input::Input, state::GameState};

pub fn engine(input: Input, mut state: GameState) -> GameState {
    match input {
        Input::NewGame(save_name) => {
            state.current = Some(save_name);
            state.startup = false;
        }
        Input::Quit => {
            state.quit = true;
        }
        Input::Save => {
            match state.current {
                None => println!("No game found to save!"),
                Some(_) => GameState::save(&state).unwrap(),
            };
        }
        Input::Load(save_name) => {
            GameState::load(&mut state, save_name).unwrap();
        }
    }
    state
}
