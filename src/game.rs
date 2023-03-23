pub mod game_loop;

use super::data::kingdom::*;

/// A struct to hold general data about the running game. Currently it only holds the kingdom,
/// but will be extended by other variables in the future.
pub struct GameState{
    /// The current player's kingdom
    pub kingdom : Kingdom,
}

impl GameState{
    pub fn new(kingdom : Kingdom) -> GameState {
        GameState{
            kingdom
        }
    }
}