pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;
mod ui;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;
use ui::GameUIPlugin;

use bevy::prelude::*;

use crate::events::GameOver;
use crate::systems::game_over_event_clear;
use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            // Need to do manual event cleanup due to run conditions
            // If event is cleared before game over menu text is updated the final score will not be diplayed.
            // To fix it the game over event will be available until GameOver state exits
            //.add_event::<GameOver>()
            .init_resource::<Events<GameOver>>()
            // States
            .init_state::<SimulationState>()
            // OnEnter Systems
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            // My Plugins
            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(StarPlugin)
            .add_plugins(GameUIPlugin)
            // Systems
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            // Exit State Systems
            .add_systems(OnExit(AppState::Game), resume_simulation)
            // Clear game over envents  on GameOver state exit
            .add_systems(OnExit(AppState::GameOver), game_over_event_clear);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
