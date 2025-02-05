use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    window::{PresentMode, WindowResolution},
};

pub const CLEAR: Color = Color::srgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

mod ascii;
mod audio;
mod combat;
mod debug;
mod fadeout;
mod graphics;
mod player;
mod tilemap;

use ascii::AsciiPlugin;
use audio::GameAudioPlugin;
use combat::CombatPlugin;
use debug::DebugPlugin;
use fadeout::FadeoutPlugin;
use graphics::GraphicsPlugin;
use player::PlayerPlugin;
use tilemap::TileMapPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, States, Default)]
pub enum GameState {
    #[default]
    Overworld,
    Combat,
}

fn main() {
    let height = 900.0;
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: WindowResolution::new(height * RESOLUTION, height),
                        title: "Bevy RPG Tutorial".to_string(),
                        present_mode: PresentMode::Fifo,
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<GameState>()
        .insert_resource(ClearColor(CLEAR))
        .add_systems(Startup, spawn_camera)
        .add_plugins((
            PlayerPlugin,
            GameAudioPlugin,
            GraphicsPlugin,
            CombatPlugin,
            FadeoutPlugin,
            AsciiPlugin,
            DebugPlugin,
            TileMapPlugin,
        ))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let projection = OrthographicProjection {
        area: Rect {
            min: Vec2::new(-1.0 * RESOLUTION, -1.0),
            max: Vec2::new(1.0 * RESOLUTION, 1.0),
        },
        scaling_mode: ScalingMode::Fixed {
            width: 2.0 * RESOLUTION,
            height: 2.0,
        },
        ..OrthographicProjection::default_2d()
    };

    commands.spawn((Camera2d, projection));
}
