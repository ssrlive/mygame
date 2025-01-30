use bevy::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};

use super::{
    level::level_plugin::LevelPlugin, player::player_plugin::PlayerPlugin, ui::ui_plugin::UiPlugin,
    window::WindowSettingsPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            WindowSettingsPlugin,
            LevelPlugin,
            PlayerPlugin,
            UiPlugin,
        ));
    }
}
