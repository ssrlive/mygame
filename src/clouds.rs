use bevy::prelude::*;

use crate::{assets::ImageAssets, physics::*};

#[derive(Resource)]
pub struct CloudTimer(Timer);

pub struct CloudPlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cloud_spawn_system)
            .insert_resource(CloudTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
    }
}

fn cloud_spawn_system(mut commands: Commands, time: Res<Time>, mut cloud_timer: ResMut<CloudTimer>, image_assets: Res<ImageAssets>) {
    use rand::Rng;
    let mut rng = rand::rng();
    let cloud_texture = match rng.random_bool(0.5) {
        true => image_assets.cloud_1.clone(),
        false => image_assets.cloud_2.clone(),
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
                rng.random_range(-2.0..3.0),
            )),
            Visibility::default(),
            Velocity(Vec2::new(rng.random_range(-700.0..-400.0), rng.random_range(-10.0..10.0))),
        ));
    }
}
