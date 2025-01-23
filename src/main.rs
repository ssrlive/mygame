use std::collections::HashMap;

use bevy::prelude::*;

mod animation;
mod player;

use animation::{animate_sprite, Animation, Animator};
use player::{move_player, PlayerMovement};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup_env)
        .add_systems(Update, (animate_sprite, move_player))
        .run();
}

fn setup_env(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("player.png");
    let texture_atlas =
        TextureAtlasLayout::from_grid(UVec2::new(8 + 1, 9 + 1), 3, 1, Some(UVec2::new(1, 1)), None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_atlas_image(
            texture_handle,
            TextureAtlas {
                layout: texture_atlas_handle,
                index: 0,
            },
        ),
        Transform::from_scale(Vec3::splat(5.0)),
        Animator::new(create_player_anim_hashmap(), "Walk", 0.0, 0.05),
        PlayerMovement { speed: 100.0 },
    ));
}

pub fn create_player_anim_hashmap() -> HashMap<String, Animation> {
    let mut hash_map = HashMap::new();
    hash_map.insert("Walk".to_string(), Animation::new(1, 3, true, 0.1));
    hash_map.insert("Idle".to_string(), Animation::new(1, 1, true, 0.1));
    hash_map
}
