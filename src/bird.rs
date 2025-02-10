use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::{input::keyboard::KeyCode, prelude::*};

use crate::animation;
use crate::gamedata;
use crate::gamestate;
use crate::physics;
use crate::pipes;
use crate::screens;

use animation::*;
use gamedata::*;
use gamestate::*;
use physics::*;
use pipes::*;
use screens::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct PlayerTimer(pub Timer);

#[derive(Resource)]
pub struct JumpHeight(pub f32);

// data for animating rotation
#[derive(Component)]
pub struct VelocityRotator {
    pub angle_up: f32,
    pub angle_down: f32,
    // The amount of velocity to reach the min or max angle
    pub velocity_max: f32,
}

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_input,
                player_bounds_system,
                player_collision_system,
                velocity_rotator_system,
                velocity_animator_system,
            ),
        );
    }
}

fn player_input(
    game_data: Res<GameData>,
    jump_height: Res<JumpHeight>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform, &Player)>,
) {
    let Ok((mut velocity, translation, _player)) = query.get_single_mut() else {
        return;
    };
    match game_data.game_state {
        GameState::Menu => {
            handle_stay_in_screen(jump_height, &mut velocity, &translation);
        }
        GameState::Playing => {
            handle_jump(keyboard_input, jump_height, velocity);
        }
        GameState::Dead => {}
    }
}

// Auto jump until input is given
fn handle_stay_in_screen(jump_height: Res<JumpHeight>, velocity: &mut Mut<'_, Velocity>, transform: &Mut<'_, Transform>) {
    if transform.translation.y < 0.0 {
        velocity.0.y = jump_height.0;
    }
}

fn handle_jump(keyboard_input: Res<ButtonInput<KeyCode>>, jump_height: Res<JumpHeight>, mut velocity: Mut<Velocity>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        velocity.0.y = jump_height.0;
    }
}

#[allow(clippy::complexity)]
fn player_bounds_system(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut player_query: Query<(&Player, &mut Transform, &mut Velocity), Without<Pipe>>,
    mut pipe_query: Query<(&Pipe, &Transform, &Collider, &Sprite, Entity), Without<Player>>,
    mut score_collider_query: Query<(&ScoreGiver, &Transform, &Collider, Entity), Without<Player>>,
    mut end_screen_query: Query<(&EndScreen, &mut Visibility)>,
) {
    let half_screen_size = 1280.0 * 0.5;
    let player_size = 32.0 * 6.0;
    for (_p, mut transform, mut velocity) in &mut player_query.iter_mut() {
        // bounce against ceiling
        if transform.translation.y > half_screen_size - player_size {
            velocity.0.y = -3.0;
            transform.translation.y = half_screen_size - player_size;
        }
        // death on bottom touch
        if transform.translation.y < -half_screen_size {
            trigger_death(
                &mut commands,
                &mut game_data,
                &mut pipe_query,
                &mut score_collider_query,
                &mut end_screen_query,
            );
        }
    }
}

#[allow(clippy::complexity)]
fn player_collision_system(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    player_query: Query<(&Player, &Transform)>,
    mut pipe_query: Query<(&Pipe, &Transform, &Collider, &Sprite, Entity), Without<Player>>,
    mut score_collider_query: Query<(&ScoreGiver, &Transform, &Collider, Entity), Without<Player>>,
    mut end_screen_query: Query<(&EndScreen, &mut Visibility)>,
) {
    // Player size can't be fetched from AtlasTextureSprite, so I'm hard coding it here...
    let mut player_size = 6.0 * 32.0;
    // Make player hitbox half size, to feel more fair
    player_size *= 0.4;
    let player_size_vec = (player_size, player_size);
    for (_player, player_translation) in &mut player_query.iter() {
        for (_s, translation, _collider, entity) in &mut score_collider_query.iter() {
            let collision = collide(
                player_translation.translation,
                player_size_vec.into(),
                translation.translation,
                Vec2::new(10.0, 1280.0),
            );
            if collision {
                game_data.score += 1;
                println!("got score!: {}", game_data.score);
                // Remove coin collider, quick simple solution
                commands.entity(entity).despawn_recursive();
            }
        }
        // Check for collision
        let mut did_collide = false;
        for (_pipe, pipe_translation, _collider, pipe_sprite, _pipe_entity) in &mut pipe_query.iter() {
            let collision = collide(
                player_translation.translation,
                player_size_vec.into(),
                pipe_translation.translation,
                pipe_sprite.custom_size.unwrap(),
            );
            if collision {
                did_collide = true;
                break;
            }
        }
        if did_collide {
            trigger_death(
                &mut commands,
                &mut game_data,
                &mut pipe_query,
                &mut score_collider_query,
                &mut end_screen_query,
            );
        }
    }
}

fn collide(pos1: Vec3, size1: Vec2, pos2: Vec3, size2: Vec2) -> bool {
    let volume = Aabb2d::new(pos1.xy(), size1 / 2.0);
    Aabb2d::new(pos2.xy(), size2 / 2.0).intersects(&volume)
}

#[allow(clippy::complexity)]
fn trigger_death(
    commands: &mut Commands,
    game_data: &mut ResMut<GameData>,
    pipe_query: &mut Query<(&Pipe, &Transform, &Collider, &Sprite, Entity), Without<Player>>,
    score_query: &mut Query<(&ScoreGiver, &Transform, &Collider, Entity), Without<Player>>,
    end_screen_query: &mut Query<(&EndScreen, &mut Visibility)>,
) {
    game_data.game_state = GameState::Dead;
    game_data.score = 0;
    // Despawn all pipes
    for (_p, _pt, _c, _ps, pipe_entity) in &mut pipe_query.iter() {
        commands.entity(pipe_entity).despawn_recursive();
    }
    // Despawn score colliders
    for (_s, _t, _collider, score_entity) in &mut score_query.iter() {
        commands.entity(score_entity).despawn_recursive();
    }
    for (_es, mut draw) in &mut end_screen_query.iter_mut() {
        *draw = Visibility::Visible;
    }
}

fn velocity_rotator_system(mut query: Query<(&Velocity, &mut Transform, &VelocityRotator)>) {
    let Ok((velocity, mut transform, velocity_rotator)) = query.get_single_mut() else {
        return;
    };
    let mut procentage = velocity.0.y / velocity_rotator.velocity_max;
    procentage = procentage.max(-1.0);
    procentage = procentage.min(1.0);
    // convert from -1 -> 1 to: 0 -> 1
    procentage = (procentage + 1.0) * 0.5;

    // Lerp from lower angle to upper angle
    let rad_angle = (1.0 - procentage) * velocity_rotator.angle_down + procentage * velocity_rotator.angle_up;

    transform.rotation = Quat::from_rotation_z(rad_angle);
}

fn velocity_animator_system(mut query: Query<(&mut Animations, &Velocity)>) {
    for (mut animations, velocity) in &mut query.iter_mut() {
        if velocity.0.y > 0.0 {
            animations.current_animation = 0;
        } else {
            animations.current_animation = 1;
        }
    }
}

pub fn spawn_bird(commands: &mut Commands, asset_server: &mut Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 2, 2, None, None);
    let texture_atlas_layout = texture_atlases.add(layout);

    let image = asset_server.load("bird.png");
    let mut bird = Sprite::from_atlas_image(image, texture_atlas_layout.into());
    bird.custom_size = Some(Vec2::splat(32.0 * 6.0));

    commands.spawn((
        bird,
        Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
        PlayerTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
        AffectedByGravity,
        VelocityRotator {
            angle_up: std::f32::consts::PI * 0.5 * 0.7,
            angle_down: -std::f32::consts::PI * 0.5 * 0.5,
            velocity_max: 400.0,
        },
        Velocity(Vec2::ZERO),
        Animations {
            animations: vec![
                Animation {
                    current_frame: 0,
                    frames: vec![
                        AnimationFrame { index: 0, time: 0.1 },
                        AnimationFrame { index: 1, time: 0.1 },
                        AnimationFrame { index: 2, time: 0.3 },
                        AnimationFrame { index: 1, time: 0.1 },
                    ],
                },
                Animation {
                    current_frame: 0,
                    frames: vec![AnimationFrame { index: 3, time: 0.2 }],
                },
            ],
            current_animation: 0,
        },
    ));
}
