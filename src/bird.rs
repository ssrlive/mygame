use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::{input::keyboard::KeyCode, prelude::*};

use crate::animation::*;
use crate::assets::{AudioAssets, ImageAssets};
use crate::gamestate::*;
use crate::physics::*;
use crate::pipes::*;
use crate::screens::*;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash, Resource)]
pub struct Score(pub i32);

#[derive(Component)]
pub struct ScoreText;

#[derive(Component, Default)]
pub struct Bird {
    dead: bool,
}

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
        app.init_resource::<Score>()
            .insert_resource(JumpHeight(23.0 * 40.0))
            .add_systems(Startup, spawn_bird)
            .add_systems(
                Update,
                (
                    update_score_text,
                    handle_stay_in_screen.run_if(in_state(GameState::Menu)),
                    (
                        handle_jump,
                        player_bounds_system,
                        player_collision_system,
                        velocity_rotator_system,
                        velocity_animator_system,
                        deal_with_bird_death,
                    )
                        .run_if(in_state(GameState::Playing)),
                ),
            );
    }
}

fn update_score_text(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        for mut text in &mut query {
            text.0 = score.0.to_string();
        }
    }
}

// Auto jump until input is given
fn handle_stay_in_screen(jump_height: Res<JumpHeight>, mut query: Query<(&mut Velocity, &mut Transform, &Bird)>) {
    let Ok((mut velocity, transform, _player)) = query.get_single_mut() else {
        return;
    };

    if transform.translation.y < 0.0 {
        velocity.0.y = jump_height.0;
    }
}

fn handle_jump(
    mut commands: Commands,
    mut query: Query<(&mut Velocity, &mut Transform, &Bird)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    jump_height: Res<JumpHeight>,
    audio_assets: Res<AudioAssets>,
) {
    let Ok((mut velocity, _transform, _bird)) = query.get_single_mut() else {
        return;
    };
    if keyboard_input.just_pressed(KeyCode::Space) {
        velocity.0.y = jump_height.0;
        commands.spawn((AudioPlayer::new(audio_assets.flap.clone()), PlaybackSettings::DESPAWN));
    }
}

#[allow(clippy::complexity)]
fn player_bounds_system(mut player_query: Query<(&mut Bird, &mut Transform, &mut Velocity), Without<Pipe>>) {
    let half_screen_size = 1280.0 * 0.5;
    let player_size = 32.0 * 6.0;
    let Ok((mut bird, mut transform, mut velocity)) = player_query.get_single_mut() else {
        return;
    };
    // bounce against ceiling
    if transform.translation.y > half_screen_size - player_size {
        velocity.0.y = -3.0;
        transform.translation.y = half_screen_size - player_size;
    }
    // death on bottom touch
    if transform.translation.y < -half_screen_size {
        bird.dead = true;
    }
}

#[allow(clippy::complexity)]
fn player_collision_system(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut player_query: Query<(&mut Bird, &Transform, &Sprite)>,
    pipe_query: Query<(&Pipe, &Transform, &Collider, &Sprite), Without<Bird>>,
    score_collider_query: Query<(&ScoreGiver, &Transform, &Collider, Entity), Without<Bird>>,
    audio_assets: Res<AudioAssets>,
) {
    let Ok((mut player, player_translation, bird)) = player_query.get_single_mut() else {
        return;
    };
    let hitbox_size = bird.custom_size.unwrap() / 2.0; // Note the hitbox is half size, to feel more fair

    for (_s, translation, _collider, entity) in &mut score_collider_query.iter() {
        let collision = collide(
            player_translation.translation,
            hitbox_size,
            translation.translation,
            Vec2::new(10.0, 1280.0),
        );
        if collision {
            score.0 += 1;
            bevy::log::info!("got score!: {}", score.0);
            commands.spawn((AudioPlayer::new(audio_assets.point.clone()), PlaybackSettings::DESPAWN));
            commands.entity(entity).remove::<Collider>();
        }
    }
    // Check for collision
    for (_pipe, pipe_translation, _collider, pipe_sprite) in &mut pipe_query.iter() {
        let collision = collide(
            player_translation.translation,
            hitbox_size,
            pipe_translation.translation,
            pipe_sprite.custom_size.unwrap(),
        );
        if collision {
            player.dead = true;
            break;
        }
    }
}

fn collide(pos1: Vec3, size1: Vec2, pos2: Vec3, size2: Vec2) -> bool {
    let volume = Aabb2d::new(pos1.xy(), size1 / 2.0);
    Aabb2d::new(pos2.xy(), size2 / 2.0).intersects(&volume)
}

#[allow(clippy::complexity)]
fn deal_with_bird_death(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut player_query: Query<&mut Bird>,
    pipe_query: Query<(&Pipe, Entity), Without<Bird>>,
    score_query: Query<(&ScoreGiver, Entity), Without<Bird>>,
    mut end_screen_query: Query<(&EndScreen, &mut Visibility)>,
    mut game_state: ResMut<NextState<GameState>>,
    audio_assets: Res<AudioAssets>,
) {
    let Ok(mut bird) = player_query.get_single_mut() else {
        return;
    };
    if !bird.dead {
        return;
    }
    commands.spawn((AudioPlayer::new(audio_assets.hit.clone()), PlaybackSettings::DESPAWN));
    bird.dead = false;

    game_state.set(GameState::Dead);
    score.0 = 0;
    // Despawn all pipes
    for (_p, pipe_entity) in pipe_query.iter() {
        commands.entity(pipe_entity).despawn_recursive();
    }
    // Despawn all score colliders
    for (_s, score_entity) in &mut score_query.iter() {
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

fn spawn_bird(mut commands: Commands, image_assets: Res<ImageAssets>) {
    let mut bird = Sprite::from_atlas_image(image_assets.bird.clone(), image_assets.bird_layout.clone().into());
    bird.custom_size = Some(Vec2::splat(32.0 * 6.0));

    commands.spawn((
        bird,
        Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
        PlayerTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Bird::default(),
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
