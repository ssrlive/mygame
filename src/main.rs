use bevy::{prelude::*, window::PrimaryWindow};
use rand::{rngs::ThreadRng, thread_rng, Rng};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Bird".to_string(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(800.0, 600.0).into(),
                        ..Window::default()
                    }),
                    ..WindowPlugin::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .run();
}
