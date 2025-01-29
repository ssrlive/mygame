use bevy::{prelude::*, window::PrimaryWindow};

pub fn spawn_crosshair(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let crosshair_size = 2.0;
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                ImageNode::solid_color(Color::srgb(0.0, 1.0, 0.0)),
                Node {
                    width: Val::Px(crosshair_size),
                    height: Val::Px(crosshair_size),
                    left: Val::Px(window.width() / 2. - (crosshair_size / 2.)),
                    top: Val::Px(window.height() / 2. - (crosshair_size / 2.)),
                    ..default()
                },
            ));
        });
}
