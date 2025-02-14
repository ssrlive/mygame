#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;

use tiny_shooter::animation::AnimationPlugin;
use tiny_shooter::camera::FollowCameraPlugin;
use tiny_shooter::collision::CollisionPlugin;
use tiny_shooter::enemy::EnemyPlugin;
use tiny_shooter::gui::GuiPlugin;
use tiny_shooter::gun::GunPlugin;
use tiny_shooter::player::PlayerPlugin;
use tiny_shooter::state::GameState;
use tiny_shooter::world::WorldPlugin;
use tiny_shooter::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // mode: bevy::window::WindowMode::Fullscreen,
                        resizable: true,
                        focused: true,
                        resolution: (WW, WH).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .add_plugins(FollowCameraPlugin)
        .add_plugins(GuiPlugin)
        .add_plugins(GunPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CollisionPlugin)
        .run();
}
