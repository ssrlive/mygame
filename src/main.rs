use bevy::prelude::*;
use iyes_perf_ui::{
    prelude::{PerfUiEntryFPS, PerfUiEntryFPSWorst, PerfUiRoot},
    PerfUiPlugin,
};

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        PerfUiRoot {
            display_labels: false,
            layout_horizontal: true,
            ..default()
        },
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFPS::default(),
    ));
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
