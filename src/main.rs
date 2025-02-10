#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
mod animation;
mod bird;
mod bounds_deletion;
mod clouds;
mod gamedata;
mod gamestate;
mod mountains;
mod physics;
mod pipes;
mod screens;

use animation::*;
use bird::*;
use clouds::*;
use gamedata::*;
use gamestate::*;
use mountains::*;
use physics::*;
use pipes::*;
use screens::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PipePlugin,
            BirdPlugin,
            CloudPlugin,
            MountainPlugin,
            MyAnimationPlugin,
            PhysicsPlugin,
            ScreensPlugin,
            GameStatePlugin,
        ))
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::srgb(0.34, 0.75, 0.79)))
        .insert_resource(JumpHeight(23.0 * 40.0))
        .insert_resource(Gravity(45.0 * 40.0))
        .insert_resource(GameData {
            game_state: GameState::Menu,
            score: 0,
        })
        .run();
}

fn setup(mut commands: Commands, mut asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>) {
    commands.spawn(Camera2d);
    bird::spawn_bird(&mut commands, &mut asset_server, &mut texture_atlases);
}
