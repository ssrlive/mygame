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
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut player_query: Query<(&Bird, &mut Transform, &mut Velocity)>,
    mut end_screen_query: Query<(&EndScreen, &mut Visibility), Without<StartScreen>>,
    mut start_screen_query: Query<(&StartScreen, &mut Visibility), Without<EndScreen>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match game_state.get() {
        GameState::Menu => {
            if keyboard_input.just_pressed(KeyCode::Space) || mouse_button_input.just_pressed(MouseButton::Left) {
                next_state.set(GameState::Playing);
                for (_ss, mut draw) in &mut start_screen_query.iter_mut() {
                    *draw = Visibility::Hidden;
                }
            }
        }
        GameState::Playing => {}
        GameState::Dead => {
            if keyboard_input.just_pressed(KeyCode::Space) || mouse_button_input.just_pressed(MouseButton::Left) {
                next_state.set(GameState::Playing);
                if let Ok((_p, mut transform, mut velocity)) = player_query.get_single_mut() {
                    transform.translation = Vec3::new(0.0, 0.0, 100.0);
                    velocity.0.y = 0.0;
                }
                for (_es, mut visibility) in &mut end_screen_query.iter_mut() {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}
