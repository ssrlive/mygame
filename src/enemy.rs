use bevy::prelude::*;

use crate::player::PlayerMovement;

#[derive(Component)]
pub struct EnemyConfig {
    pub health: f32,
    pub speed: f32,
}

impl EnemyConfig {
    pub fn new(health: f32, speed: f32) -> Self {
        Self { health, speed }
    }
}

pub fn update_enemies(
    time: Res<Time>,
    mut enemy_query: Query<(&EnemyConfig, &mut Transform, Entity), Without<PlayerMovement>>,
    player_query: Query<(&PlayerMovement, &mut Transform), Without<EnemyConfig>>,
    mut commands: Commands,
) {
    if let Ok((_player_movement, player_transform)) = player_query.get_single() {
        for (enemy, mut transform, entity) in enemy_query.iter_mut() {
            let moving = Vec3::normalize(player_transform.translation - transform.translation) * enemy.speed * time.delta_secs();
            transform.translation += moving;
            if enemy.health <= 0.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}
