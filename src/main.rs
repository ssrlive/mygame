use bevy::prelude::*;

mod alien;
mod game;
mod resolution;

fn main() {
    App::new()
        .add_plugins((
            // list of plugins
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Space Invaders".to_string(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(768.0, 512.0).into(),
                        ..Window::default()
                    }),
                    ..WindowPlugin::default()
                })
                .set(ImagePlugin::default_nearest()),
            game::GamePlugin,
        ))
        .run();
}
