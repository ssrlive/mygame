use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowMode, WindowResolution},
};

use super::cursor::CursorPlugin;

pub struct WindowSettingsPlugin;

impl Plugin for WindowSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CursorPlugin)
            .add_systems(PreStartup, init_window);
    }
}

fn init_window(mut query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = query.get_single_mut() {
        window.resolution = WindowResolution::new(1920., 1080.);
        window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
    }
}
