#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::{prelude::*, render::camera::ScalingMode};

mod animation;
mod assets;
mod bird;
mod bounds_deletion;
mod clouds;
mod gamestate;
mod mountains;
mod physics;
mod pipes;
mod screens;

use animation::*;
use assets::AssetsPlugin;
use bird::*;
use clouds::*;
use gamestate::*;
use mountains::*;
use physics::*;
use pipes::*;
use screens::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            AssetsPlugin,
            PipePlugin,
            BirdPlugin,
            CloudPlugin,
            MountainPlugin,
            BirdAnimationPlugin,
            PhysicsPlugin,
            ScreensPlugin,
            GameStatePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::WindowSize,
        scale: 1.4,
        ..OrthographicProjection::default_2d()
    });
    commands.spawn((Camera2d, projection));
}
