use bevy::prelude::*;

use super::{
    level::level::LevelPlugin, player::player::PlayerPlugin, ui::ui::UiPlugin,
    window::WindowSettingsPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WindowSettingsPlugin, LevelPlugin, PlayerPlugin, UiPlugin));
    }
}
