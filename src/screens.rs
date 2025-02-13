use bevy::prelude::*;

use crate::{
    assets::{FontAssets, ImageAssets},
    bird::ScoreText,
};

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

fn setup(mut commands: Commands, image_assets: Res<ImageAssets>, font_assets: Res<FontAssets>) {
    // Start Screen
    let start_texture_handle = image_assets.space_to_start.clone();
    commands.spawn((Sprite::from_image(start_texture_handle), StartScreen));

    let game_over_texture_handle = image_assets.game_over_text.clone();
    commands.spawn((Visibility::Hidden, Sprite::from_image(game_over_texture_handle), EndScreen));

    commands
        .spawn((
            Visibility::default(),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|node| {
            node.spawn((
                ScoreText,
                Text::new("0"),
                TextFont::from_font(font_assets.outfit.clone()).with_font_size(80.0),
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });
}
