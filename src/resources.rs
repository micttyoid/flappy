use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Resource, Deref, DerefMut)]
pub struct Score(pub u32);

impl Score {
    pub fn value(&self) -> u32 {
        self.0
    }
}
