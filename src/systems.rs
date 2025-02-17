use bevy::prelude::*;
use std::collections::VecDeque;

use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use crate::utils::*;

pub fn blink_space_bar_text(
    time: Res<Time>,
    mut query: Query<(&mut PressSpaceBarText, &mut Visibility)>,
) {
    // Panic if there are more than one entity coming back from the query
    let (mut space, mut visibility) = query.single_mut();
    let timer = &mut space.0;

    // Tick the timer
    timer.tick(time.delta());

    if timer.finished() {
        if *visibility == Visibility::Hidden {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn move_background(time: Res<Time>, mut query: Query<&mut Transform, With<Background>>) {
    let mut background_transform = query.single_mut();
    let delta = time.delta().as_secs_f32();

    // 20 unit per delta
    let delta_x = 20. * delta;

    background_transform.translation.x -= delta_x;

    // Repeat at about half way total length
    if background_transform.translation.x < -288.0 {
        background_transform.translation.x = 0.;
    }
}

pub fn move_ground(time: Res<Time>, mut query: Query<&mut Transform, With<Ground>>) {
    let mut ground_transform = query.single_mut();
    let delta = time.delta().as_secs_f32();

    // Move faster because it's closer to the camera perspective
    let delta_x = 150. * delta;

    ground_transform.translation.x -= delta_x;

    if ground_transform.translation.x < -288.0 {
        ground_transform.translation.x = 0.;
    }
}

pub fn animate_bird(time: Res<Time>, mut query: Query<(&mut Bird, &mut Sprite)>) {
    for (mut bird, mut sprite) in query.iter_mut() {
        let delta = time.delta();
        bird.timer.tick(delta);

        if bird.timer.finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == 2 { 0 } else { atlas.index + 1 };
            }
        }
    }
}

pub fn reset_game(
    mut next_state: ResMut<NextState<GameState>>,
    mut game_over_query: Query<&mut Visibility, (With<GameOverText>, Without<PressSpaceBarText>)>,
    mut bird_query: Query<(&mut Bird, &mut Transform)>,
    mut upper_pipe_query: Query<(&mut Transform, &mut UpperPipe), (With<UpperPipe>, Without<Bird>)>,
    mut lower_pipe_query: Query<
        &mut Transform,
        (With<LowerPipe>, Without<Bird>, Without<UpperPipe>),
    >,
) {
    // Drag pipes back and Rerandomize visible pipes
    let mut lower_ys: VecDeque<f32> = VecDeque::new();

    for (i, (mut transform, mut upper_pipe)) in upper_pipe_query.iter_mut().enumerate() {
        let delta_x = i as f32 * 200.0 + 200.;
        upper_pipe.passed = false;
        transform.translation.x = 0.;
        transform.translation.x += delta_x;
        let (lower_y, upper_y) = random_pipe_position();
        transform.translation.y = upper_y;
        lower_ys.push_back(lower_y);
    }

    for (i, mut transform) in lower_pipe_query.iter_mut().enumerate() {
        let delta_x = i as f32 * 200.0 + 200.;
        transform.translation.x = 0.;
        transform.translation.x += delta_x;
        transform.translation.y = lower_ys.pop_front().unwrap();
    }

    // Reset bird
    for (mut bird, mut transform) in bird_query.iter_mut() {
        bird.velocity = 0.0;
        transform.translation.y = 0.0;
        transform.rotation = Quat::from_rotation_z(0.0);
    }

    // Hiding the GameOverText
    let mut game_over_visibility = game_over_query.single_mut();
    *game_over_visibility = Visibility::Hidden;

    next_state.set(GameState::Inactive);
}

pub fn start_game(
    mut next_state: ResMut<NextState<GameState>>,
    mut space_query: Query<(&mut PressSpaceBarText, &mut Visibility)>,
) {
    next_state.set(GameState::Active);

    // Hiding the PressSpaceBarText
    let (mut space, mut visibility) = space_query.single_mut();
    space.0.reset();
    *visibility = Visibility::Hidden;
}

pub fn gravity(
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut query: Query<(&mut Bird, &mut Transform)>,
    mut game_over_query: Query<&mut Visibility, With<GameOverText>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (mut bird, mut transform) in query.iter_mut() {
        let delta = time.delta().as_secs_f32();
        let gravity = 9.8;
        // 150.0 to make the bird fall faster ...
        let delta_v = gravity * 150. * delta;

        let delta_y = bird.velocity * delta;
        let new_y = (transform.translation.y + delta_y).min(260.0); // prevent out of bounds
        transform.translation.y = new_y;

        bird.velocity -= delta_v;
        transform.translation.y += bird.velocity * delta;

        // Rotate the bird
        let rotation = bird.velocity / 600.0;
        let max_rotation = 0.5;
        transform.rotation = Quat::from_rotation_z(rotation.max(-max_rotation).min(max_rotation));

        // stop the bird when it touches the ground
        let ground_y = -250.0;
        let ground_height = 112.0;
        let bird_height = 24.0;

        let collision_point = ground_y + ground_height / 2.0 + bird_height / 2.0;

        if transform.translation.y < collision_point {
            transform.translation.y = collision_point;
            // Stop the bird
            bird.velocity = 0.0;

            // Gameover
            next_state.set(GameState::GameOver);
            *game_over_query.single_mut() = Visibility::Visible;

            // play game over sound
            commands.spawn((
                AudioPlayer::new(asset_server.load("audio/hit.ogg")),
                // Removed by Bevy right after the sound finishes playing.a
                PlaybackSettings::DESPAWN,
            ));
        }
    }
}

pub fn jump(mut query: Query<&mut Bird>, mut commands: Commands, asset_server: Res<AssetServer>) {
    // if keyboard pressed
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/wing.ogg")),
        // Removed by Bevy right after the sound finishes playing.a
        PlaybackSettings::DESPAWN,
    ));

    for mut bird in query.iter_mut() {
        // this affects "delta_y" in "gravity"
        bird.velocity = 300.0;
    }
}

pub fn pipes(
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut upper_pipe_query: Query<(&mut UpperPipe, &mut Transform)>,
    mut lower_pipe_query: Query<(&LowerPipe, &mut Transform), Without<UpperPipe>>,
    // Collision related
    mut bird_query: Query<&Transform, (With<Bird>, Without<LowerPipe>, Without<UpperPipe>)>,
    mut game_over_query: Query<&mut Visibility, With<GameOverText>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let delta = time.delta().as_secs_f32();
    let delta_x = 150. * delta; // The change of x position per refresh

    let utmost_right_pipe = upper_pipe_query // can be lower, doesn't matter
    .iter() // make it an iterator so that we can run `max_by` on it
    .max_by(|(_, a), (_, b)| a.translation.x.partial_cmp(&b.translation.x).unwrap())
    .unwrap()
    .1 // to get the transform and not `UpperPipe`
    .translation
    .x; // x position of the rightmost pipe

    // reset the pipes that are out of the screen by rightmost pipe
    let new_pipe_position = utmost_right_pipe + 200.0;
    let (lower_y, upper_y) = random_pipe_position();
    let out_of_screen_x = (-WINDOW_WIDTH / 2.) - 26.;

    for (mut upper_pipe, mut transform) in upper_pipe_query.iter_mut() {
        transform.translation.x -= delta_x;

        if transform.translation.x < out_of_screen_x {
            // Pipe out of the screen
            transform.translation.x = new_pipe_position;
            transform.translation.y = upper_y;
            upper_pipe.passed = false;
        }
    }

    for (_, mut transform) in lower_pipe_query.iter_mut() {
        transform.translation.x -= delta_x;
        // Pipe out of the screen
        if transform.translation.x < out_of_screen_x {
            transform.translation.x = new_pipe_position;
            transform.translation.y = lower_y;
        }
    }

    // Collision
    let is_collision = |bird_transform: &Transform, pipe_transform: &Transform| -> bool {
        let bird_x = bird_transform.translation.x;
        let bird_y = bird_transform.translation.y;
        let bird_width = 34.0;
        let bird_height = 24.0;

        let pipe_x = pipe_transform.translation.x;
        let pipe_y = pipe_transform.translation.y;
        let pipe_width = 52.0;
        let pipe_height = 320.0;

        let collision_x = bird_x + bird_width / 2.0 > pipe_x - pipe_width / 2.0
            && bird_x - bird_width / 2.0 < pipe_x + pipe_width / 2.0;
        let collision_y = bird_y + bird_height / 2.0 > pipe_y - pipe_height / 2.0
            && bird_y - bird_height / 2.0 < pipe_y + pipe_height / 2.0;

        collision_x && collision_y
    };

    for bird_transform in bird_query.iter_mut() {
        let mut game_over = || {
            next_state.set(GameState::GameOver);

            *game_over_query.single_mut() = Visibility::Visible;

            // Play game over sound
            commands.spawn((
                AudioPlayer::new(asset_server.load("audio/hit.ogg")),
                PlaybackSettings::DESPAWN,
            ));
        };

        for (_, transform) in upper_pipe_query.iter_mut() {
            if is_collision(bird_transform, &transform) {
                game_over();
            }
        }

        for (_, transform) in lower_pipe_query.iter_mut() {
            if is_collision(bird_transform, &transform) {
                game_over();
            }
        }
    }
}

pub fn update_score(
    mut score: ResMut<Score>,
    bird_query: Query<(&Bird, &Transform)>,
    mut upper_pipe_query: Query<(&mut UpperPipe, &Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (_, bird_transform) in bird_query.iter() {
        for (mut upper_pipe, transform) in upper_pipe_query.iter_mut() {
            let passed = transform.translation.x < bird_transform.translation.x;
            let passed_state = upper_pipe.passed;

            if passed && !passed_state {
                **score += 1;

                upper_pipe.passed = true;

                commands.spawn((
                    AudioPlayer::new(asset_server.load("audio/point.ogg")),
                    PlaybackSettings::DESPAWN,
                ));

                println!("Score: {}", score.to_string());
            }
        }
    }
}

pub fn render_score(score: Res<Score>, mut query: Query<&mut Sprite, With<ScoreText>>) {
    let score_string = format!("{:03}", score.value());
    let score_digits: Vec<usize> = score_string
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    for (digit, mut sprite) in score_digits.iter().zip(query.iter_mut()) {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = *digit;
        }
    }
}
