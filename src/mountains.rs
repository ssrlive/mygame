use bevy::prelude::*;

use crate::bounds_deletion::*;
use crate::physics::*;

// Spawn mountains with a delay
#[derive(Resource)]
pub struct MountainTimer(pub Timer);

pub struct MountainPlugin;

impl Plugin for MountainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mountain_spawn_system)
            .insert_resource(MountainTimer(Timer::from_seconds(3.0, TimerMode::Repeating)));
    }
}

fn mountain_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut mountain_timer: ResMut<MountainTimer>,
    asset_server: Res<AssetServer>,
) {
    use rand::Rng;
    let mut rng = rand::rng();
    let mountain_texture = match rng.random_bool(0.5) {
        true => asset_server.load("mountain.png"),
        false => asset_server.load("mountain.png"),
    };

    let mut sprite = Sprite::from_image(mountain_texture);
    sprite.custom_size = Some(Vec2::new(640.0, 400.0) * 3.0);

    mountain_timer.0.tick(time.delta());
    if mountain_timer.0.finished() {
        commands.spawn((
            sprite.clone(),
            BackgroundColor(Color::srgb(0.36, 0.36, 0.36)),
            Transform::from_translation(Vec3::new(1920.0 * 0.5 + 30.0 * 43.0, -1280.0 * 0.5, 0.2)),
            OffsceenDeletion,
            Velocity(Vec2::new(-200.0, 0.0)),
        ));
        commands.spawn((
            sprite,
            BackgroundColor(Color::srgb(0.26, 0.26, 0.26)),
            Transform::from_translation(Vec3::new(1920.0 * 0.5 + 30.0 * 43.0, -1280.0 * 0.5 - 100.0, 0.3)),
            OffsceenDeletion,
            Velocity(Vec2::new(-400.0, 0.0)),
        ));
    }
}
