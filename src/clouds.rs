use crate::physics;
use bevy::prelude::*;
use physics::*;

#[derive(Resource)]
pub struct CloudTimer(Timer);

pub struct CloudPlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cloud_spawn_system)
            .insert_resource(CloudTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
    }
}

fn cloud_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut cloud_timer: ResMut<CloudTimer>,
    asset_server: Res<AssetServer>,
) {
    use rand::Rng;
    let mut rng = rand::rng();
    let cloud_texture = match rng.random_bool(0.5) {
        true => asset_server.load("assets/cloud_1.png"),
        false => asset_server.load("assets/cloud_2.png"),
    };
    let mut sprite = Sprite::from_image(cloud_texture);
    sprite.custom_size = Some(Vec2::new(43.0, 8.0) * rng.random_range(6.0..30.0));

    cloud_timer.0.tick(time.delta());
    if cloud_timer.0.finished() {
        commands.spawn((
            sprite,
            Transform::from_translation(Vec3::new(
                1920.0 * 0.5 + 30.0 * 43.0,
                rng.random_range(-1280.0 * 0.5..1280.0 * 0.5),
                2.0,
            )),
            Visibility::default(),
            Velocity(Vec2::new(
                rng.random_range(-700.0..-400.0),
                rng.random_range(-10.0..10.0),
            )),
        ));
    }
}
