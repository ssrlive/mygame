use bevy::prelude::*;

use crate::alien::AlienPlugin;
use crate::player::PlayerPlugin;
use crate::resolution::ResolutionPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AlienPlugin, ResolutionPlugin, PlayerPlugin))
            .add_systems(Startup, setup_scene);
    }
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d);
}
