use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Core simulation state tracking game time progression.
#[derive(Resource, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct SimulationState {
    /// Total number of simulation ticks elapsed.
    pub game_time: u64,
    /// Number of full in-game days that have passed.
    pub game_days: u32,
    /// How many ticks make up one in-game day.
    pub ticks_per_day: u32,
    /// Current simulation speed multiplier (0 = paused).
    pub speed: u32,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            game_time: 0,
            game_days: 0,
            ticks_per_day: 600,
            speed: 1,
        }
    }
}

impl SimulationState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_paused(&self) -> bool {
        self.speed == 0
    }

    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }

    pub fn pause(&mut self) {
        self.speed = 0;
    }

    pub fn toggle_pause(&mut self) {
        if self.is_paused() {
            self.speed = 1;
        } else {
            self.speed = 0;
        }
    }
}

/// System sets for ordering simulation work within `FixedUpdate`.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SimulationSystems {
    /// Advance the game clock.
    AdvanceTime,
    /// Process character / squad actions.
    ProcessActions,
    /// Update economy (resource production, costs, etc.).
    UpdateEconomy,
    /// Sync any per-tick UI data.
    UpdateUI,
}

/// Run condition: returns `true` when the simulation is not paused.
pub fn simulation_not_paused(state: Res<SimulationState>) -> bool {
    !state.is_paused()
}

/// Advance the simulation clock by one tick.
fn advance_time(mut state: ResMut<SimulationState>) {
    state.game_time += 1;
    if state.game_time % state.ticks_per_day as u64 == 0 {
        state.game_days += 1;
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<SimulationState>();
    app.register_type::<SimulationState>();

    // Configure simulation system sets as a chain inside FixedUpdate,
    // gated on the simulation not being paused.
    app.configure_sets(
        FixedUpdate,
        (
            SimulationSystems::AdvanceTime,
            SimulationSystems::ProcessActions,
            SimulationSystems::UpdateEconomy,
            SimulationSystems::UpdateUI,
        )
            .chain()
            .run_if(simulation_not_paused),
    );

    app.add_systems(
        FixedUpdate,
        advance_time.in_set(SimulationSystems::AdvanceTime),
    );
}
