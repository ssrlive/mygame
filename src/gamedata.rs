use bevy::prelude::*;

use crate::gamestate;
use gamestate::GameState;

#[derive(Resource)]
pub struct GameData {
    pub game_state: GameState,
    pub score: i32,
}
