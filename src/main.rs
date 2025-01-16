use bevy::prelude::*;
use iyes_perf_ui::{prelude::PerfUiAllEntries, PerfUiPlugin};

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(PerfUiAllEntries::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup)
        .run();
}
