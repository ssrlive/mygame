use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ImageAssets>()
            .init_collection::<AudioAssets>()
            .init_collection::<FontAssets>();
    }
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/flappybird.ttf")]
    pub outfit: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flap.ogg")]
    pub flap: Handle<AudioSource>,
    #[asset(path = "audio/hit.ogg")]
    pub hit: Handle<AudioSource>,
    #[asset(path = "audio/point.ogg")]
    pub point: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "sprites/bird.png")]
    pub bird: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 32, tile_size_y = 32, columns = 2, rows = 2, padding_x = 0, padding_y = 0))]
    pub bird_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "sprites/cloud_1.png")]
    pub cloud_1: Handle<Image>,
    #[asset(path = "sprites/cloud_2.png")]
    pub cloud_2: Handle<Image>,
    #[asset(path = "sprites/GameOverText.png")]
    pub game_over_text: Handle<Image>,
    #[asset(path = "sprites/mountain.png")]
    pub mountain: Handle<Image>,
    #[asset(path = "sprites/pipe.png")]
    pub pipe: Handle<Image>,
    #[asset(path = "sprites/SpaceToStart.png")]
    pub space_to_start: Handle<Image>,
}
