use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use crate::enemy::Enemy;
use crate::player::{Health, Player};
use crate::state::GameState;
use crate::world::GameEntity;

pub struct GuiPlugin;

#[derive(Component)]
struct DebugText;
#[derive(Component)]
struct MainMenuItem;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnEnter(GameState::GameInit), spawn_debug_text)
            .add_systems(
                Update,
                update_debug_text.run_if(in_state(GameState::InGame)),
            );
    }
}

fn spawn_debug_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Visibility::default(),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            GameEntity,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(345.0),
                        height: Val::Px(125.0),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(8.0)),
                        margin: UiRect::px(10.0, 10.0, 10.0, 0.0),
                        ..default()
                    },
                    BackgroundColor::from(Color::BLACK.with_alpha(0.9)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text("Hello Bevy!".to_string()),
                        TextFont::from_font(asset_server.load("monogram.ttf")).with_font_size(40.0),
                        TextColor(Color::WHITE),
                        DebugText,
                    ));
                });
        });
}

fn update_debug_text(
    mut query: Query<&mut Text, With<DebugText>>,
    diagnostics: Res<DiagnosticsStore>,
    enemy_query: Query<(), With<Enemy>>,
    player_query: Query<&Health, With<Player>>,
) {
    if query.is_empty() || player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let num_enemies = enemy_query.iter().count();
    let player_health = player_query.single().0;
    let mut text = query.single_mut();
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            text.0 = format!("Fps: {value:.2}\nEnemies: {num_enemies}\nHealth: {player_health}");
        }
    }
}

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Visibility::default(),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text("Play".to_string()),
                        TextFont::from_font_size(40.0),
                        TextColor(Color::BLACK),
                    ));
                });
        })
        .insert(MainMenuItem);
}

fn handle_main_menu_buttons(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query.iter() {
        if interaction == &Interaction::Pressed {
            next_state.set(GameState::GameInit);
        }
    }
}

fn despawn_main_menu(mut commands: Commands, menu_items_query: Query<Entity, With<MainMenuItem>>) {
    for e in menu_items_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
