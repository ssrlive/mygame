use bevy::prelude::*;

use crate::resolution::Resolution;

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_aliens).add_systems(Update, update_aliens);
    }
}

#[derive(Component)]
pub struct Alien;

#[derive(Resource)]
pub struct AlienManager {
    pub direction: f32,
}

const WIDTH: i32 = 10;
const HEIGHT: i32 = 5;
const SPACING: f32 = 24.;
const SPEED: f32 = 100.0;

fn setup_aliens(mut commands: Commands, asset_server: Res<AssetServer>, resolution: Res<Resolution>) {
    commands.insert_resource(AlienManager { direction: 1.0 });
    let alien_texture = asset_server.load("alien.png");
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let position = Vec3::new(x as f32 * SPACING, y as f32 * SPACING, 0.0)
                - (Vec3::X * WIDTH as f32 * SPACING * 0.5)
                - (Vec3::Y * HEIGHT as f32 * SPACING * 1.0)
                + (Vec3::Y * resolution.screen_dimensions.y * 0.5);
            commands.spawn((
                Sprite::from_image(alien_texture.clone()),
                Transform::from_translation(position).with_scale(Vec3::splat(resolution.pixel_ratio)),
                Alien,
            ));
        }
    }
}

fn update_aliens(mut alien_query: Query<(&Alien, &mut Transform)>, alien_manager: ResMut<AlienManager>, time: Res<Time>) {
    for (_alien, mut transform) in alien_query.iter_mut() {
        transform.translation.x += time.delta_secs() * alien_manager.direction * SPEED;
    }
}
