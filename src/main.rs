#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
