// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;

use ball_stars_pkg::{
    exit_game, handle_game_over, spawn_camera, transition_to_game_state,
    transition_to_main_menu_state, AppState, GamePlugin, MainMenuPlugin,
};

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        // My Plugins
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        // Startup Systems
        .add_systems(Startup, spawn_camera)
        // Systems
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update, transition_to_main_menu_state)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();
}
