use super::resources::GameState;
use bevy::prelude::*;

pub fn tick_passive_income(_time: Res<Time>, mut state: ResMut<GameState>) {
    if state.is_paused {
        return;
    }

    state.game_time += 1.0;
}
