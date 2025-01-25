use bevy::prelude::*;

use crate::player::PlayerParameters;

#[derive(Component)]
pub struct EnemyParameters {
    pub health: f32,
    pub speed: f32,
}

impl EnemyParameters {
    pub fn new(health: f32, speed: f32) -> Self {
        Self { health, speed }
    }
}

pub fn update_enemies(
    time: Res<Time>,
    mut enemy_query: Query<(&EnemyParameters, &mut Transform, Entity), Without<PlayerParameters>>,
    player_query: Query<(&PlayerParameters, &mut Transform), Without<EnemyParameters>>,
    mut commands: Commands,
) {
    if let Ok((_player, player_transform)) = player_query.get_single() {
        for (enemy, mut transform, entity) in enemy_query.iter_mut() {
            let moving = Vec3::normalize(player_transform.translation - transform.translation) * enemy.speed * time.delta_secs();
            transform.translation += moving;
            if enemy.health <= 0.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}
