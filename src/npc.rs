use bevy::prelude::*;

use crate::{
    ascii::{spawn_ascii_sprite, spawn_ascii_text, spawn_nine_slice, AsciiSheet, NineSliceIndices},
    combat::CombatStats,
    player::Player,
    GameState, CLEAR, TILE_SIZE,
};

pub struct NpcPlugin;

#[derive(Component)]
pub struct NpcText;

#[derive(Component)]
pub enum Npc {
    Healer,
}

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (npc_speech, clear_speech.after(npc_speech)).run_if(in_state(GameState::Overworld)),
        );
    }
}

fn clear_speech(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    speech_query: Query<Entity, With<NpcText>>,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
) {
    if keyboard.any_just_pressed([KeyCode::KeyE, KeyCode::Space]) {
        keyboard.clear();
        let mut player = player_query.single_mut();
        for ent in speech_query.iter() {
            player.active = true;
            commands.entity(ent).despawn_recursive();
        }
    }
}

fn spawn_textbox(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    indices: &NineSliceIndices,
    translation: Vec2,
    text: &str,
) -> Entity {
    let width = text.len() as f32 + 2.0;
    let text_nine_slice = spawn_nine_slice(commands, ascii, indices, width, 3.0);
    let background = spawn_ascii_sprite(
        commands,
        ascii,
        0,
        CLEAR,
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(width, 3.0, 1.0),
    );

    let x_offset = (-width / 2.0 + 1.5) * TILE_SIZE;
    let text = spawn_ascii_text(commands, ascii, text, Vec3::new(x_offset, 0.0, 0.0));

    commands
        .spawn((
            Transform::from_translation(translation.extend(900.0)),
            GlobalTransform::default(),
            Name::new("Npc Text"),
            NpcText,
            Visibility::default(),
        ))
        .add_child(text)
        .add_child(background)
        .add_child(text_nine_slice)
        .id()
}

fn npc_speech(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut CombatStats, &Transform)>,
    camera_query: Query<&Transform, With<Camera2d>>,
    npc_query: Query<(&Npc, &Transform)>,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
    ascii: Res<AsciiSheet>,
    indices: Res<NineSliceIndices>,
) {
    let (mut player, mut stats, transform) = player_query.single_mut();
    if !player.active {
        return;
    }

    if keyboard.clear_just_pressed(KeyCode::KeyE) {
        for (_npc, npc_transform) in npc_query.iter() {
            let lhs = npc_transform.translation.truncate();
            if Vec2::distance(lhs, transform.translation.truncate()) < TILE_SIZE * 1.5 {
                player.active = false;
                stats.health = stats.max_health;
                let camera_transform = camera_query.single();

                spawn_textbox(
                    &mut commands,
                    &ascii,
                    &indices,
                    Vec2::new(0.0, 1.0 - 1.5 * TILE_SIZE) + camera_transform.translation.truncate(),
                    "You seem weak, let me heal you!",
                );
                break;
            }
        }
    }
}
