use bevy::{audio::AudioPlugin, prelude::*};

use crate::constants::{DEFAULT_VOLUME, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy".to_string(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume::new(DEFAULT_VOLUME),
                    ..default()
                }),
        );
    }
}
