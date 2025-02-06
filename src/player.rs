use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    combat::CombatStats,
    fadeout::create_fadeout,
    tilemap::{EncounterSpawner, TileCollider},
    GameState, TILE_SIZE,
};

pub struct PlayerPlugin;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EncounterTracker {
    timer: Timer,
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    pub active: bool,
    just_moved: bool,
    pub exp: usize,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Overworld), show_player)
            .add_systems(OnExit(GameState::Overworld), hide_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    player_encounter_checking.after(player_movement),
                    camera_follow.after(player_movement),
                )
                    .run_if(in_state(GameState::Overworld)),
            )
            .add_systems(Startup, spawn_player);
    }
}

fn hide_player(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let mut player_vis = player_query.single_mut();
    *player_vis = Visibility::Hidden;

    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                *child_vis = Visibility::Hidden;
            }
        }
    }
}

fn show_player(
    mut player_query: Query<(&mut Player, &mut Visibility)>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let Ok((mut player, mut player_vis)) = player_query.get_single_mut() else {
        bevy::log::info!("No player found");
        return;
    };
    player.active = true;
    *player_vis = Visibility::Visible;

    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                *child_vis = Visibility::Visible;
            }
        }
    }
}

fn player_encounter_checking(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut EncounterTracker, &Transform)>,
    encounter_query: Query<&Transform, (With<EncounterSpawner>, Without<Player>)>,
    ascii: Res<AsciiSheet>,
    time: Res<Time>,
) {
    let (mut player, mut encounter_tracker, player_transform) = player_query.single_mut();
    let player_translation = player_transform.translation;

    if player.just_moved
        && encounter_query
            .iter()
            .any(|&transform| wall_collision_check(player_translation, transform.translation))
    {
        encounter_tracker.timer.tick(time.delta());

        if encounter_tracker.timer.just_finished() {
            player.active = false;
            create_fadeout(&mut commands, GameState::Combat, &ascii);
        }
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn player_movement(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut transform) = player_query.single_mut();
    player.just_moved = false;

    if !player.active {
        return;
    }

    let mut y_delta = 0.0;
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        y_delta += player.speed * TILE_SIZE * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        y_delta -= player.speed * TILE_SIZE * time.delta_secs();
    }

    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        x_delta -= player.speed * TILE_SIZE * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        x_delta += player.speed * TILE_SIZE * time.delta_secs();
    }

    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    if !wall_query
        .iter()
        .any(|&transform| wall_collision_check(target, transform.translation))
    {
        if x_delta != 0.0 {
            player.just_moved = true;
        }
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if !wall_query
        .iter()
        .any(|&transform| wall_collision_check(target, transform.translation))
    {
        if y_delta != 0.0 {
            player.just_moved = true;
        }
        transform.translation = target;
    }
}

fn wall_collision_check(target_player_pos: Vec3, wall_translation: Vec3) -> bool {
    let wall = Aabb2d::new(wall_translation.xy(), Vec2::splat(TILE_SIZE / 2.0));
    let player = Aabb2d::new(target_player_pos.xy(), Vec2::splat(TILE_SIZE * 0.9 / 2.0));
    player.intersects(&wall)
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::srgb(0.3, 0.3, 0.9),
        Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
        Vec3::splat(1.0),
    );

    let _ = commands
        .entity(player)
        .insert(Visibility::Hidden)
        .insert(Name::new("Player"))
        .insert(Player {
            speed: 3.0,
            active: true,
            just_moved: false,
            exp: 0,
        })
        .insert(EncounterTracker {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .insert(CombatStats {
            health: 10,
            max_health: 10,
            attack: 2,
            defense: 1,
        })
        .id();

    let background = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        0,
        Color::srgb(0.5, 0.5, 0.5),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::splat(1.0),
    );

    let _ = commands
        .entity(background)
        .insert(Name::new("Background"))
        .id(); //id() gives back the entity after creation

    commands.entity(player).add_children(&[background]);
}
