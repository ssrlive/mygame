use bevy::prelude::*;

use super::camera_controller::{update_camera_controller, CameraController};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_player)
            .add_systems(Update, update_camera_controller);
    }
}

#[derive(Component)]
pub struct Player;

fn init_player(mut commands: Commands) {
    let fov = 103.0_f32.to_radians();
    commands.spawn((
        Player,
        Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection { fov, ..default() }),
        CameraController {
            rotation: Vec2::ZERO,
            rotation_lock: 88.0,
            sensitivity: 0.035,
        },
    ));
}
