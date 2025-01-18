use bevy::prelude::*;

use crate::resolution::Resolution;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_projectiles, update_alien_interactions));
    }
}

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
}

fn update_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Projectile, &mut Transform)>,
    time: Res<Time>,
    resolution: Res<Resolution>,
) {
    for (entity, projectile, mut transform) in projectile_query.iter_mut() {
        transform.translation.y += time.delta_secs() * projectile.speed;
        if transform.translation.y > resolution.screen_dimensions.y * 0.5 {
            commands.entity(entity).despawn();
        }
    }
}

fn update_alien_interactions() {}
