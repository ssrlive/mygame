use bevy::prelude::*;

use crate::resolution::Resolution;

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_aliens);
    }
}

fn setup_aliens(mut commands: Commands, asset_server: Res<AssetServer>, resolution: Res<Resolution>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("alien.png")),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_scale(Vec3::splat(resolution.pixel_ratio)),
    ));
}
