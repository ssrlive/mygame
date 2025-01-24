use bevy::prelude::*;

use crate::enemy::EnemyConfig;

#[derive(Component)]
pub struct Bullet {
    pub lifetime: f32,
    pub speed: f32,
    pub direction: Vec2,
}

impl Bullet {
    pub fn new(lifetime: f32, speed: f32, direction: Vec2) -> Self {
        Self {
            lifetime,
            speed,
            direction,
        }
    }
}

pub fn update_bullets(mut bullet_query: Query<(&mut Bullet, &mut Transform, Entity)>, time: Res<Time>, mut commands: Commands) {
    for (mut bullet, mut transform, entity) in bullet_query.iter_mut() {
        bullet.lifetime -= time.delta_secs();
        let moving = bullet.speed * bullet.direction * time.delta_secs();
        transform.translation += moving.extend(0.0);
        if bullet.lifetime <= 0. {
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
    bullet_query: Query<(&Transform, Entity), (With<Bullet>, Without<EnemyConfig>)>,
    mut enemy_query: Query<(&mut EnemyConfig, &mut Transform), Without<Bullet>>,
) {
    let mut bullet_list = Vec::new();
    for (transform, entity) in bullet_query.iter() {
        bullet_list.push(BulletInfo::new(transform.translation.truncate(), entity));
    }
    let mut bullet_len = bullet_list.len() as i32;
    for (mut enemy, transform) in enemy_query.iter_mut() {
        let mut i = 0;
        while i < bullet_len {
            if Vec2::distance(bullet_list[i as usize].translation, transform.translation.truncate()) <= 36.0 {
                enemy.health -= 1.0;
                commands.entity(bullet_list[i as usize].entity).despawn();
                bullet_list.remove(i as usize);
                i -= 1;
                bullet_len -= 1;
            }
            i += 1;
        }
    }
}
