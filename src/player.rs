use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerMovement {
    pub speed: f32,
}

pub fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&PlayerMovement, &mut Transform)>,
) {
    for (player_movement, mut transform) in query.iter_mut() {
        let delta = player_movement.speed * time.delta_secs();
        if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
            transform.translation.y += delta;
        }
        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= delta;
        }
        if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= delta;
        }
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            transform.translation.x += delta;
        }
    }
}
