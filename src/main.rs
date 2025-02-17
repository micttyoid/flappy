use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use plugin::MyPlugin;
use resources::*;
use setup::setup;
use systems::*;

mod components;
mod constants;
mod plugin;
mod resources;
mod setup;
mod systems;
mod utils;

fn main() {
    App::new()
        .insert_resource(Score(0))
        .add_plugins(MyPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            blink_space_bar_text.run_if(in_state(GameState::Inactive)),
        )
        .add_systems(Update, move_background.run_if(in_state(GameState::Active)))
        .add_systems(Update, move_ground.run_if(in_state(GameState::Active)))
        .add_systems(Update, animate_bird.run_if(in_state(GameState::Active)))
        .add_systems(
            Update,
            start_game
                .run_if(in_state(GameState::Inactive))
                .run_if(input_just_pressed(KeyCode::Space)),
        )
        .add_systems(
            Update,
            reset_game
                .run_if(in_state(GameState::GameOver))
                .run_if(input_just_pressed(KeyCode::Space)),
        )
        .add_systems(Update, gravity.run_if(in_state(GameState::Active)))
        .add_systems(
            Update,
            jump.run_if(in_state(GameState::Active))
                .run_if(input_just_pressed(KeyCode::Space)),
        )
        .add_systems(Update, pipes.run_if(in_state(GameState::Active)))
        .add_systems(Update, update_score.run_if(in_state(GameState::Active)))
        .add_systems(Update, render_score.run_if(in_state(GameState::Active)))
        .run();
}
