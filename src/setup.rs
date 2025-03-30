use bevy::prelude::*;

// setup.rs
use crate::{
    components::*,
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    utils::random_pipe_position,
};
// AssetServer is a Bevy resource(Res)

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawn a 2D camera
    commands.spawn(Camera2d::default());
    commands.spawn(Transform::from_xyz(100.0, 200.0, 0.0));
    // Spawn the background
    // 0.14 "SpriteBundle.texture" is equivalent to "Sprite.image"
    commands.spawn((
        Sprite {
            // Bevy expects the assets to be in 'assets/'
            image: asset_server.load("texture/background.png"),
            custom_size: Some(Vec2::new(
                WINDOW_WIDTH + 288.0 * 2.,
                WINDOW_HEIGHT,
            )), // Adding a custom size
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.,
            },
            ..default()
        },
        Background,
    ));

    commands.spawn((
        Sprite {
            // Bevy expects the assets to be in 'assets/'
            image: asset_server.load("texture/base.png"),
            // Adding a custom size
            custom_size: Some(Vec2::new(WINDOW_WIDTH + 288. * 2., 112.)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.,
            },
            ..default()
        },
        Transform::from_xyz(0., -250., 1.),
        Ground,
    ));

    // Game Over Text
    commands.spawn((
        Sprite {
            image: asset_server.load("texture/game-over.png"),
            ..default()
        },
        Transform::from_xyz(0., 0., 1.),
        Visibility::Hidden,
        GameOverText,
    ));

    // Space Bar Text
    commands.spawn((
        Sprite {
            image: asset_server.load("texture/space.png"),
            ..default()
        },
        Transform::from_xyz(0.0, -50.0, 1.0),
        // Blink every 0.5 seconds
        PressSpaceBarText(Timer::from_seconds(0.5, TimerMode::Repeating)),
    ));

    // Score Text
    let number_layout: TextureAtlasLayout =
        TextureAtlasLayout::from_grid(UVec2::new(24, 36), 1, 10, None, None);
    let number_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(number_layout);

    for i in 0..3 {
        let starting_point = -350. + (i as f32 * (24. + 2.)); // 24 is the width + 0.2 is the space between the numbers
        commands.spawn((
            Transform::from_xyz(starting_point, 200., 1.),
            Sprite::from_atlas_image(
                asset_server.load("texture/numbers.png"),
                TextureAtlas {
                    index: 0,
                    layout: number_texture_atlas_layout.clone(),
                },
            ),
            ScoreText,
        ));
    }

    // Spawn the bird
    commands.spawn((
        Transform::from_xyz(0., 0., 2.),
        Sprite::from_atlas_image(
            asset_server.load("texture/bird.png"),
            TextureAtlas {
                index: 1,
                layout: texture_atlas_layouts.add(
                    TextureAtlasLayout::from_grid(
                        UVec2::new(34, 24),
                        3,
                        1,
                        None,
                        None,
                    ),
                ),
            },
        ),
        Bird {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            velocity: 0.,
        },
        //Bird,
    ));

    // Spawn the pipes
    for i in 0..5 {
        let delta_x = i as f32 * 200.; // Space between pairs of pipes
        let (lower_y, upper_y) = random_pipe_position();
        let mut transform = Transform::from_xyz(350. + delta_x, lower_y, 0.5);

        // Spawn Lower Pipe
        commands.spawn((
            Sprite {
                image: asset_server.load("texture/pipe.png"),
                ..default()
            },
            transform,
            LowerPipe,
        ));

        // Rotating the upper pipe
        transform.rotate(Quat::from_rotation_z(std::f32::consts::PI));
        // Changing the y position of the upper pipe
        transform.translation.y = upper_y;

        // Spawn Upper Pipe
        commands.spawn((
            Sprite {
                image: asset_server.load("texture/pipe.png"),
                ..default()
            },
            transform,
            UpperPipe { passed: false },
        ));
    }
}
