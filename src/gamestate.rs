use bevy::prelude::*;

use crate::bird::*;
use crate::physics::*;
use crate::screens::*;

#[derive(PartialEq, States, Debug, Hash, Eq, Clone, Copy, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Dead,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_systems(Update, handle_gamestate_system);
    }
}

fn handle_gamestate_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform, &mut Velocity)>,
    mut end_screen_query: Query<(&EndScreen, &mut Visibility), Without<StartScreen>>,
    mut start_screen_query: Query<(&StartScreen, &mut Visibility), Without<EndScreen>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match game_state.get() {
        GameState::Menu => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                next_state.set(GameState::Playing);
                for (_ss, mut draw) in &mut start_screen_query.iter_mut() {
                    *draw = Visibility::Hidden;
                }
            }
        }
        GameState::Playing => {}
        GameState::Dead => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                next_state.set(GameState::Playing);
                for (_p, mut translation, mut velocity) in &mut player_query.iter_mut() {
                    translation.translation = Vec3::new(0.0, 0.0, 100.0);
                    velocity.0.y = 0.0;
                }
                for (_es, mut draw) in &mut end_screen_query.iter_mut() {
                    *draw = Visibility::Hidden;
                }
            }
        }
    }
}
