use std::time::Duration;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::building::Building;
use crate::game::character::{CharacterInfo, Health};
use crate::game::data::GameData;
use crate::game::resources::BaseState;
use crate::screens::Screen;

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
    /// Speed before pausing, restored on unpause.
    #[serde(default = "default_previous_speed")]
    pub previous_speed: u32,
}

fn default_previous_speed() -> u32 {
    1
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            game_time: 0,
            game_days: 0,
            ticks_per_day: 600,
            speed: 1,
            previous_speed: 1,
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
        self.speed = speed.min(5);
    }

    pub fn pause(&mut self) {
        if !self.is_paused() {
            self.previous_speed = self.speed;
        }
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

/// Handle keyboard shortcuts for simulation speed control.
/// Space: toggle pause, Digit1: 1x, Digit2: 2x, Digit3: 5x.
fn speed_controls(input: Res<ButtonInput<KeyCode>>, mut state: ResMut<SimulationState>) {
    if input.just_pressed(KeyCode::Space) {
        state.toggle_pause();
    }
    if input.just_pressed(KeyCode::Digit1) {
        state.set_speed(1);
    }
    if input.just_pressed(KeyCode::Digit2) {
        state.set_speed(2);
    }
    if input.just_pressed(KeyCode::Digit3) {
        state.set_speed(5);
    }
}

/// Adjust the `FixedUpdate` timestep when simulation speed changes.
fn adjust_tick_rate(state: Res<SimulationState>, mut time: ResMut<Time<Fixed>>) {
    if state.is_changed() {
        if state.is_paused() {
            // When paused, FixedUpdate systems are gated by simulation_not_paused
            // so there is nothing to adjust here — just leave the timestep as-is.
        } else {
            time.set_timestep(Duration::from_secs_f64(1.0 / state.speed as f64));
        }
    }
}

/// Drain hunger by 1 every 10 ticks for all characters.
fn drain_hunger(mut characters: Query<(&mut Health, &CharacterInfo)>, sim: Res<SimulationState>) {
    if sim.game_time % 10 != 0 {
        return;
    }
    for (mut health, _info) in &mut characters {
        if health.hunger > 0 {
            health.hunger = health.hunger.saturating_sub(1);
        }
    }
}

/// Sum power generation across all completed buildings and update BaseState.power.
fn update_power_grid(
    buildings: Query<&Building>,
    game_data: Res<GameData>,
    mut base_state: ResMut<BaseState>,
) {
    let mut total_generation: i32 = 0;
    let mut total_consumption: i32 = 0;

    for building in &buildings {
        if !building.complete {
            continue;
        }
        if let Some(def) = game_data.get_building(&building.def_id) {
            if def.power_generation > 0 {
                total_generation += def.power_generation;
            } else if def.power_generation < 0 {
                total_consumption += def.power_generation.abs();
            }
        }
    }

    base_state.power.generation = total_generation.max(0) as u32;
    base_state.power.consumption = total_consumption.max(0) as u32;
    base_state.power.current = base_state
        .power
        .generation
        .saturating_sub(base_state.power.consumption);
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

    app.add_systems(
        FixedUpdate,
        (drain_hunger, update_power_grid).in_set(SimulationSystems::UpdateEconomy),
    );

    // Speed controls and tick-rate adjustment run in Update (not FixedUpdate)
    // so they are responsive even while paused, and only during gameplay.
    app.add_systems(
        Update,
        (speed_controls, adjust_tick_rate).run_if(in_state(Screen::Gameplay)),
    );
}
