use crate::{input::Input, state::GameState};

pub fn engine(input: Input, mut state: GameState) -> GameState {
    match input {
        Input::NewGame => todo!(),
        Input::Quit => {
            state.quit = true;
            state
        }
    }
}
