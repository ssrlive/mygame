#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowResolution},
};
use game::{GROUND_WIDTH, GROUND_Z};

mod game;
mod game_over;
mod menu;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Flappy Bird"),
                        resolution: WindowResolution::new(288.0, 512.0),
                        resizable: false,
                        enabled_buttons: EnabledButtons {
                            maximize: false,
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<GameState>()
        .add_systems(Startup, scene_setup)
        .add_plugins((
            game::GamePlugin,
            game_over::GameOverPlugin,
            menu::MenuPlugin,
        ))
        .run();
}

#[derive(Resource)]
struct AudioHandles {
    flap: Handle<AudioSource>,
    hit: Handle<AudioSource>,
    point: Handle<AudioSource>,
}

#[derive(Component)]
struct Scroll;

#[derive(Component)]
struct Ground;

fn scene_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a 2D camera
    commands.spawn(Camera2d);

    // Spawn the background sprite
    commands.spawn(Sprite::from_image(
        asset_server.load("sprites/background.png"),
    ));

    // Spawn 2 ground sprites so that they can scroll infinitely
    let texture_handle = asset_server.load("sprites/ground.png");
    for i in 0..2 {
        commands.spawn((
            Ground,
            Scroll,
            Sprite::from_image(texture_handle.clone()),
            Transform::from_xyz(i as f32 * GROUND_WIDTH, -200.0, GROUND_Z),
        ));
    }

    // Load audio files
    commands.insert_resource(AudioHandles {
        flap: asset_server.load("audio/flap.ogg"),
        hit: asset_server.load("audio/hit.ogg"),
        point: asset_server.load("audio/point.ogg"),
    });
}

// Return true if the user has clicked, tapped or pressed the space bar
pub fn has_user_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
) -> bool {
    keyboard_input.just_pressed(KeyCode::Space)
        || mouse_button_input.just_pressed(MouseButton::Left)
        || touch_input.any_just_pressed()
}

// Despawn all entities recursively with a given component
pub fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
