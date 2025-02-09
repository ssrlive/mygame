use crate::bounds_deletion;
use crate::gamedata;
use crate::gamestate;
use crate::physics;
use bevy::prelude::*;
use bounds_deletion::*;
use gamedata::*;
use gamestate::*;
use physics::*;

#[derive(Component)]
pub struct Pipe;

#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer,
    // center pos of pipes, in precentage
    pub last_pos: f32,
}

#[derive(Resource)]
pub struct PipeSpawnSettings {
    pub min_time: f32,
    pub max_time: f32,
    pub speed: f32,
    // distance from upper and lower pipe, in precentage
    pub min_pipe_distance: f32,
    pub max_pipe_distance: f32,
    pub max_center_delta: f32,
}

#[derive(Component, PartialEq)]
pub enum Collider {
    Solid,
    ScoreGiver,
}

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_pipe_system)
            .insert_resource(SpawnTimer {
                timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                last_pos: 0.5,
            })
            .insert_resource(PipeSpawnSettings {
                min_time: 0.9,
                max_time: 1.2,
                speed: -700.0,
                min_pipe_distance: 300.0,
                max_pipe_distance: 600.0,
                max_center_delta: 0.4,
            });
    }
}

fn spawn_pipe_system(
    mut commands: Commands,
    pipe_settings: Res<PipeSpawnSettings>,
    game_data: Res<GameData>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    if game_data.game_state != GameState::Playing {
        return;
    }

    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.finished() {
        return;
    }

    use rand::Rng;
    let mut rng = rand::rng();

    spawn_timer
        .timer
        .set_duration(std::time::Duration::from_secs_f32(
            rng.random_range(pipe_settings.min_time..pipe_settings.max_time),
        ));

    let mut new_center_pos = spawn_timer.last_pos
        - rng.random_range(-pipe_settings.max_center_delta..pipe_settings.max_center_delta);

    // sorry for the hardcoded values
    // This is the extent from the center in Y, a pipe can go maximum, until it flies in the air
    let clamp_range = (1280.0 - (6.0 * 128.0)) / 1280.0;

    // Clamp func seem to be nightly only for now
    new_center_pos = new_center_pos.min(clamp_range);
    new_center_pos = new_center_pos.max(-clamp_range);
    spawn_timer.last_pos = new_center_pos;
    // to world units
    new_center_pos *= 1280.0 * 0.5;

    let pipe_texture_handle = asset_server.load("assets/pipe.png");

    let pipe_offset_y = (6.0 * 128.0) * 0.5;
    let pipe_offset_x = (6.0 * 32.0) * 0.5;
    let mut pipe_delta =
        rng.random_range(pipe_settings.min_pipe_distance..pipe_settings.max_pipe_distance);
    // half the size because both pipes will be offseted in opposide direction
    pipe_delta *= 0.5;
    let x_pos = 1920.0 * 0.5 + pipe_offset_x;

    let mut sprite = Sprite::from_image(pipe_texture_handle.clone());
    sprite.custom_size = Some(Vec2::new(32.0, 128.0) * 6.0);

    // lower pipe
    commands.spawn((
        sprite.clone(),
        Transform::from_translation(Vec3::new(
            x_pos,
            -pipe_offset_y + new_center_pos - pipe_delta,
            3.0,
        )),
        Velocity(Vec2::new(pipe_settings.speed, 0.0)),
        Pipe,
        OffsceenDeletion,
        Collider::Solid,
    ));

    // higher pipe
    commands.spawn((
        sprite,
        Transform::from_translation(Vec3::new(
            x_pos,
            pipe_offset_y + new_center_pos + pipe_delta,
            3.0,
        ))
        .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
        Pipe,
        OffsceenDeletion,
        Velocity(Vec2::new(pipe_settings.speed, 0.0)),
        Collider::Solid,
    ));

    // score collider offseted by half player size
    let score_offset = Vec3::new(32.0 * 6.0 * 0.5, 0.0, 0.0);
    commands.spawn((
        Visibility::default(),
        Transform::from_translation(score_offset + Vec3::new(x_pos, 0.0, 0.0)),
        Collider::ScoreGiver,
        Velocity(Vec2::new(pipe_settings.speed, 0.0)),
        OffsceenDeletion,
    ));
}
