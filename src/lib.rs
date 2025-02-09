use bevy::prelude::*;

mod events;
mod game;
mod main_menu;
mod systems;

pub use game::GamePlugin;
pub use main_menu::MainMenuPlugin;
pub use systems::{
    exit_game, handle_game_over, spawn_camera, transition_to_game_state,
    transition_to_main_menu_state,
};

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
