use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Ground;

// components.rs
#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct PressSpaceBarText(pub Timer);

// components.rs
#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Bird {
    pub timer: Timer,
    pub velocity: f32,
}

#[derive(Component)]
pub struct UpperPipe {
    pub passed: bool,
}

#[derive(Component)]
pub struct LowerPipe;
