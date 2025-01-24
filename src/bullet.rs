use bevy::prelude::*;

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
