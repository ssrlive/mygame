use super::UI_Z;
use crate::{cleanup, has_user_input, GameState};
use bevy::prelude::*;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(OnExit(GameState::Menu), cleanup::<MenuEntity>)
            .add_systems(
                Update,
                start_playing.run_if(in_state(GameState::Menu).and(has_user_input)),
            );
    }
}

#[derive(Component)]
struct MenuEntity;

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/start.png")),
        Transform::from_xyz(0.0, 80.0, UI_Z),
        MenuEntity,
    ));
}

fn start_playing(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Playing);
}
