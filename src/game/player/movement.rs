use bevy::prelude::*;
use bevy_rapier3d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};

use super::{camera_controller::CameraController, input::PlayerInput, player::Player};

pub fn update_movement_input(keys: Res<ButtonInput<KeyCode>>, mut input: ResMut<PlayerInput>) {
    input.movement = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        input.movement.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        input.movement.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        input.movement.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        input.movement.y += 1.0;
    }
}

pub fn update_movement(
    time: Res<Time<Fixed>>,
    input: Res<PlayerInput>,
    camera_query: Query<&CameraController>,
    mut player_query: Query<(
        &mut Player,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
    )>,
) {
    let camera = camera_query.get_single().unwrap();
    for (mut player, mut controller, output) in player_query.iter_mut() {
        if let Some(output) = output {
            if output.grounded {
                player.velocity = Vec3::ZERO;
            }
        }
        let camera_rotation_converted = -camera.rotation.y.to_radians() - 90.0_f32.to_radians();

        let forward = Vec2::new(
            f32::cos(camera_rotation_converted),
            f32::sin(camera_rotation_converted),
        );
        let right = Vec2::new(-forward.y, forward.x);

        if let Some(movement_direction) =
            (forward * input.movement.x + right * input.movement.y).try_normalize()
        {
            player.velocity.x = movement_direction.x * player.speed;
            player.velocity.z = movement_direction.y * player.speed;
        }
        player.velocity.y -= player.gravity * time.timestep().as_secs_f32();
        // delta
        controller.translation = Some(player.velocity * time.timestep().as_secs_f32());
    }
}
