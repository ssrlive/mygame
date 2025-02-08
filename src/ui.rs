use crate::{board::*, pieces::*};
use bevy::prelude::*;

// Component to mark the Text entity
#[derive(Component)]
struct NextMoveText;

/// Initialize UiCamera and text
fn init_next_move_text(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        //.spawn_bundle(UiCameraBundle::default())
        // root node
        .spawn((
            Visibility::default(),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.),
                top: Val::Px(10.),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Text("Next move: White".to_string()),
                    TextFont::from_font(font.clone()).with_font_size(40.0),
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ))
                .insert(NextMoveText);
        });
}

/// Update text with the correct turn
fn next_move_text_update(turn: Res<PlayerTurn>, mut query: Query<(&mut Text, &NextMoveText)>) {
    if !turn.is_changed() {
        return;
    }
    for (mut text, _tag) in query.iter_mut() {
        text.0 = format!(
            "Next move: {}",
            match turn.0 {
                PieceColor::White => "White",
                PieceColor::Black => "Black",
            }
        );
    }
}

/// Demo system to show off Query transformers
fn log_text_changes(query: Query<&Text, Changed<Text>>) {
    for text in query.iter() {
        println!("New text: {}", text.0);
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_next_move_text)
            .add_systems(Update, next_move_text_update)
            .add_systems(Update, log_text_changes);
    }
}
