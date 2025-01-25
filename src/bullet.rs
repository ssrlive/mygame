use bevy::prelude::*;

use crate::enemy::EnemyParameters;

#[derive(Component)]
pub struct BulletParameters {
    pub lifetime: f32,
    pub speed: f32,
    pub direction: Vec2,
}

impl BulletParameters {
    pub fn new(lifetime: f32, speed: f32, direction: Vec2) -> Self {
        Self {
            lifetime,
            speed,
            direction,
        }
    }
}

pub fn update_bullets(mut bullet_query: Query<(&mut BulletParameters, &mut Transform, Entity)>, time: Res<Time>, mut commands: Commands) {
    for (mut bullet_param, mut transform, entity) in bullet_query.iter_mut() {
        bullet_param.lifetime -= time.delta_secs();
        let moving = bullet_param.speed * bullet_param.direction * time.delta_secs();
        transform.translation += moving.extend(0.0);
        if bullet_param.lifetime <= 0. {
            commands.entity(entity).despawn();
        }
    }
}

pub struct BulletInfo {
    pub translation: Vec2,
    pub entity: Entity,
}

impl BulletInfo {
    pub fn new(translation: Vec2, entity: Entity) -> Self {
        Self { translation, entity }
    }
}

#[allow(clippy::type_complexity)]
pub fn update_bullet_hits(
    mut commands: Commands,
    bullet_query: Query<(&Transform, Entity), (With<BulletParameters>, Without<EnemyParameters>)>,
    mut enemy_query: Query<(&mut EnemyParameters, &mut Transform), Without<BulletParameters>>,
) {
    let mut bullet_list: Vec<_> = bullet_query
        .iter()
        .map(|(transform, entity)| BulletInfo::new(transform.translation.truncate(), entity))
        .collect();

    for (mut enemy, transform) in enemy_query.iter_mut() {
        bullet_list.retain(|bullet| {
            if Vec2::distance(bullet.translation, transform.translation.truncate()) <= 36.0 {
                enemy.health -= 1.0;
                commands.entity(bullet.entity).despawn();
                false
            } else {
                true
            }
        });
    }
}
