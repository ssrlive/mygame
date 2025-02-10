use bevy::prelude::*;

#[derive(Component)]
pub struct StartScreen;

#[derive(Component)]
pub struct EndScreen;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.34, 0.75, 0.79)))
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Start Screen
    let start_texture_handle = asset_server.load("SpaceToStart.png");
    commands.spawn((Sprite::from_image(start_texture_handle), StartScreen));

    let game_over_texture_handle = asset_server.load("GameOverText.png");
    commands.spawn((Visibility::Hidden, Sprite::from_image(game_over_texture_handle), EndScreen));
}
