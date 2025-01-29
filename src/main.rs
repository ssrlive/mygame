use bevy::prelude::*;
use game::game_plugin::GamePlugin;

mod game;

fn main() {
    App::new().add_plugins((GamePlugin, DefaultPlugins)).run();
}
