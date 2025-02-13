use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Game {
    pub score: u32,
    pub state: GameState,
}

#[derive(PartialEq)]
pub enum GameState {
    Active,
    Inactive,
    GameOver,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Inactive
    }
}
