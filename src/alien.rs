use bevy::prelude::*;

use crate::resolution::Resolution;

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_aliens)
            .add_systems(Update, (update_aliens, manage_alien_logic));
    }
}

#[derive(Component)]
pub struct Alien {
    pub dead: bool,
    pub original_position: Vec3,
}

/// a marker component to prevent querying any dead aliens in our updates after they have already died
#[derive(Component)]
pub struct Dead;

#[derive(Resource)]
pub struct AlienManager {
    pub direction: f32,
    pub shift_aliens_down: bool,
    pub dist_from_boundary: f32,
    pub reset: bool,
}

const WIDTH: i32 = 10;
const HEIGHT: i32 = 5;
const SPACING: f32 = 24.;
const SPEED: f32 = 100.0;
const ALIEN_SHIFT_AMOUNT: f32 = 32.;

fn setup_aliens(mut commands: Commands, asset_server: Res<AssetServer>, resolution: Res<Resolution>) {
    commands.insert_resource(AlienManager {
        direction: 1.0,
        shift_aliens_down: false,
        dist_from_boundary: 0.0,
        reset: false,
    });
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
                Alien {
                    dead: false,
                    original_position: position,
                },
            ));
        }
    }
}

fn update_aliens(
    mut commands: Commands,
    // only query aliens that are still alive
    mut alien_query: Query<(Entity, &Alien, &mut Transform, &mut Visibility), Without<Dead>>,
    mut alien_manager: ResMut<AlienManager>,
    resolution: Res<Resolution>,
    time: Res<Time>,
) {
    for (entity, alien, mut transform, mut visibility) in alien_query.iter_mut() {
        transform.translation.x += time.delta_secs() * alien_manager.direction * SPEED;
        if transform.translation.x.abs() > resolution.screen_dimensions.x * 0.5 {
            alien_manager.shift_aliens_down = true;
            alien_manager.dist_from_boundary = resolution.screen_dimensions.x * alien_manager.direction * 0.5 - transform.translation.x;
        }
        if alien.dead {
            commands.entity(entity).insert(Dead);
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }
        // if the alien has made it out of the bottom of the screen, we have lost the game and should reset
        if transform.translation.y < -resolution.screen_dimensions.y * 0.5 {
            alien_manager.reset = true;
        }
    }
}

fn manage_alien_logic(
    mut commands: Commands,
    mut alien_query: Query<(Entity, &mut Alien, &mut Transform)>,
    mut alien_manager: ResMut<AlienManager>,
) {
    if alien_manager.shift_aliens_down {
        // reverse direction and move aliens downwards
        alien_manager.shift_aliens_down = false;
        alien_manager.direction *= -1.0;
        for (_entity, _alien, mut transform) in alien_query.iter_mut() {
            transform.translation.x += alien_manager.dist_from_boundary;
            transform.translation.y -= ALIEN_SHIFT_AMOUNT;
        }
    }
    if alien_manager.reset {
        alien_manager.reset = false;
        alien_manager.direction = 1.0;
        for (entity, mut alien, mut transform) in alien_query.iter_mut() {
            transform.translation = alien.original_position;
            if alien.dead {
                // revive the alien from dead unit pool
                alien.dead = false;
                commands.entity(entity).remove::<Dead>();
            }
        }
    }
}
