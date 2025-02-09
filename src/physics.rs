use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Resource)]
pub struct Gravity(pub f32);

#[derive(Component)]
pub struct AffectedByGravity;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (velocity_system, gravity_system));
    }
}

fn gravity_system(
    gravity: Res<Gravity>,
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &AffectedByGravity)>,
) {
    for (mut velocity, _) in query.iter_mut() {
        velocity.0.y -= gravity.0 * time.delta_secs();
    }
}

fn velocity_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    let delta = time.delta_secs();
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * delta;
        transform.translation.y += velocity.0.y * delta;
    }
}
