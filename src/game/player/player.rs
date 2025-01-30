use bevy::prelude::*;
use bevy_rapier3d::prelude::{CharacterLength, Collider, KinematicCharacterController, RigidBody};

use super::{
    camera_controller::{update_camera_controller, CameraController},
    input::PlayerInput,
    movement::{update_movement, update_movement_input},
    shooting::update_player,
};
use crate::game::shooting::tracer::TracerPlugin;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TracerPlugin)
            .init_resource::<PlayerInput>()
            .add_systems(Startup, init_player)
            .add_systems(
                Update,
                (
                    update_movement_input,
                    update_player,
                    update_camera_controller,
                ),
            )
            .add_systems(FixedUpdate, update_movement); // physics timestep
    }
}

#[derive(Component)]
pub struct Player {
    pub velocity: Vec3,
    pub gravity: f32,
    pub speed: f32,
}

impl Player {
    pub fn new(velocity: Vec3, gravity: f32, speed: f32) -> Self {
        Self {
            velocity,
            gravity,
            speed,
        }
    }
}

fn init_player(mut commands: Commands) {
    let fov = 103.0_f32.to_radians();
    let camera_entity = commands
        .spawn((
            Transform::IDENTITY,
            Camera3d::default(),
            Projection::Perspective(PerspectiveProjection { fov, ..default() }),
            CameraController {
                rotation: Vec2::ZERO,
                rotation_lock: 88.0,
                sensitivity: 0.035,
            },
        ))
        .id();
    let player_entity = commands
        .spawn((
            Player::new(Vec3::ZERO, 9.81, 20.0),
            Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
            Visibility::default(),
            Collider::cuboid(1.0, 10.0, 1.0),
            RigidBody::KinematicPositionBased,
            KinematicCharacterController {
                up: Vec3::Y,
                offset: CharacterLength::Absolute(0.01),
                ..default()
            },
        ))
        .id();
    commands.entity(player_entity).add_child(camera_entity);
}
