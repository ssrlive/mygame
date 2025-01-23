use bevy::prelude::*;

use crate::animation::Animator;

#[derive(Component)]
pub struct PlayerMovement {
    pub speed: f32,
}

pub fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&PlayerMovement, &mut Transform, &mut Animator)>,
) {
    for (player_movement, mut transform, mut animator) in query.iter_mut() {
        let delta = player_movement.speed * time.delta_secs();
        let mut walking = false;
        if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
            walking = true;
            transform.translation.y += delta;
        }
        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            walking = true;
            transform.translation.x -= delta;
            transform.rotation = Quat::from_rotation_y(-std::f32::consts::PI);
        }
        if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
            walking = true;
            transform.translation.y -= delta;
        }
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            walking = true;
            transform.translation.x += delta;
            transform.rotation = Quat::default();
        }
        animator.current_animation = if walking { "Walk" } else { "Idle" }.to_string();
    }
}
