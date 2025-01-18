use bevy::prelude::*;

use crate::{projectile::Projectile, resolution::Resolution};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player).add_systems(Update, update_player);
    }
}

#[derive(Component)]
pub struct Player {
    pub shoot_timer: f32,
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>, resolution: Res<Resolution>) {
    let player_texture = asset_server.load("player.png");
    commands.spawn((
        Sprite::from_image(player_texture),
        Transform::from_translation(Vec3::new(
            0.0,
            -resolution.screen_dimensions.y * 0.5 + resolution.pixel_ratio * 5.0,
            0.0,
        ))
        .with_scale(Vec3::splat(resolution.pixel_ratio)),
        Player { shoot_timer: 0.0 },
    ));
}

const SPEED: f32 = 200.0;
const BULLET_SPEED: f32 = 400.0;
const SHOOT_COOLDOWN: f32 = 0.5;

fn update_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    resolution: Res<Resolution>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let (mut player, mut transform) = player_query.single_mut();
    let mut horizontal = 0.0;
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        horizontal -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        horizontal += 1.0;
    }
    transform.translation.x += time.delta_secs() * horizontal * SPEED;

    let left_boundary = -resolution.screen_dimensions.x * 0.5;
    let right_boundary = resolution.screen_dimensions.x * 0.5;
    if transform.translation.x > right_boundary {
        transform.translation.x = right_boundary;
    } else if transform.translation.x < left_boundary {
        transform.translation.x = left_boundary;
    }

    player.shoot_timer -= time.delta_secs();

    if keys.pressed(KeyCode::Space) && player.shoot_timer <= 0.0 {
        player.shoot_timer = SHOOT_COOLDOWN;
        let bullet_texture = asset_server.load("bullet.png");
        commands.spawn((
            Sprite::from_image(bullet_texture),
            Transform::from_translation(transform.translation).with_scale(Vec3::splat(resolution.pixel_ratio)),
            Projectile { speed: BULLET_SPEED },
        ));
    }
}
