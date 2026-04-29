pub struct GameState {
    pub quit: bool,
    pub startup: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            quit: false,
            startup: true,
        }
    }
}
