use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::{animation::Animator, create_player_anim_hashmap, enemy::EnemyConfig};

#[derive(Component)]
pub struct EnemySpawner {
    pub cooldown: f32,
    pub timer: f32,
}

impl EnemySpawner {
    pub fn new(cooldown: f32, timer: f32) -> Self {
        Self { cooldown, timer }
    }
}

pub fn update_spawning(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut spawner_query: Query<&mut EnemySpawner>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for mut spawner in spawner_query.iter_mut() {
        spawner.timer -= time.delta_secs();
        if spawner.timer <= 0.0 {
            let Ok(primary) = primary_query.get_single() else {
                return;
            };
            spawner.timer = spawner.cooldown;
            let texture_handle: Handle<Image> = asset_server.load("player.png");
            let texture_atlas = TextureAtlasLayout::from_grid(UVec2::new(9, 10), 3, 1, Some(UVec2::new(1, 1)), None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            let mut spawn_transform = Transform::from_scale(Vec3::splat(5.0));

            let mut rng = rand::thread_rng();
            spawn_transform.translation = if rng.gen_range(0..2) == 1 {
                let random_y = rng.gen_range(-primary.height() / 2.0..primary.height() / 2.0);
                if rng.gen_range(0..2) == 1 {
                    Vec3::new(primary.width() / 2.0, random_y, 0.0)
                } else {
                    Vec3::new(-primary.width() / 2.0, random_y, 0.0)
                }
            } else {
                let random_x = rng.gen_range(-primary.width() / 2.0..primary.width() / 2.0);
                if rng.gen_range(0..2) == 1 {
                    Vec3::new(random_x, primary.height() / 2.0, 0.0)
                } else {
                    Vec3::new(random_x, -primary.height() / 2.0, 0.0)
                }
            };

            let enemy_sprite = Sprite::from_atlas_image(texture_handle, texture_atlas_handle.into());
            let animator = Animator::new(create_player_anim_hashmap(), "Walk", 0.0, 0.05);
            commands.spawn((enemy_sprite, spawn_transform, EnemyConfig::new(1.0, 100.0), animator));
        }
    }
}
