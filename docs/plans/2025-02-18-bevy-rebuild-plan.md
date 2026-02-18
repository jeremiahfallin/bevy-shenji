# Shenji Bevy Rebuild — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Port the Shenji web prototype's game systems into the existing Bevy 0.17 shell, creating a fully playable management sim with characters, economy, research, crafting, building, and exploration.

**Architecture:** ECS-native design — actions as components on character entities, game data in RON asset files, simulation driven by Bevy's `Time<Fixed>` at 1 tick/second. Existing UI shell (screens, save/load, layout, inspector) is preserved and extended.

**Tech Stack:** Rust (nightly-2025-06-26), Bevy 0.17.3 with `hotpatching` + `experimental_bevy_ui_widgets`, bevy_immediate 0.4, serde/serde_json, RON for game data.

**Reference:** Design doc at `docs/plans/2025-02-18-bevy-rebuild-design.md`. Web prototype at `.claude/worktrees/musing-kirch/shenji/`.

---

## Phase 1: Simulation Core

**Goal:** Time passes in game-seconds and game-days. Player can pause and change speed. Hunger drains over time.

---

### Task 1.1: Simulation Tick Resource & Time System

**Files:**
- Create: `src/game/simulation.rs`
- Modify: `src/game/mod.rs`
- Modify: `src/game/systems.rs`

**Step 1: Create simulation module with SimulationState resource**

Create `src/game/simulation.rs`:

```rust
use bevy::prelude::*;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct SimulationState {
    /// Accumulated game-seconds (ticks)
    pub game_time: u64,
    /// Completed game days
    pub game_days: u32,
    /// Ticks per game day (default: 600 = 10 real-minutes at 1x)
    pub ticks_per_day: u32,
    /// Current speed multiplier (0 = paused, 1 = normal, 2 = fast, 5 = fastest)
    pub speed: u32,
}

impl SimulationState {
    pub fn new() -> Self {
        Self {
            game_time: 0,
            game_days: 0,
            ticks_per_day: 600,
            speed: 1,
        }
    }

    pub fn is_paused(&self) -> bool {
        self.speed == 0
    }

    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed.min(5);
    }

    pub fn pause(&mut self) {
        self.speed = 0;
    }

    pub fn toggle_pause(&mut self) {
        if self.speed == 0 {
            self.speed = 1;
        } else {
            self.speed = 0;
        }
    }
}

/// Marker: systems in this set run only when simulation is not paused
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimulationSystems {
    AdvanceTime,
    ProcessActions,
    UpdateEconomy,
    UpdateUI,
}

pub fn plugin(app: &mut App) {
    app.insert_resource(SimulationState::new())
        .register_type::<SimulationState>()
        .configure_sets(
            FixedUpdate,
            (
                SimulationSystems::AdvanceTime,
                SimulationSystems::ProcessActions,
                SimulationSystems::UpdateEconomy,
                SimulationSystems::UpdateUI,
            )
                .chain()
                .run_if(simulation_not_paused),
        )
        .add_systems(FixedUpdate, advance_time.in_set(SimulationSystems::AdvanceTime));
}

fn simulation_not_paused(state: Res<SimulationState>) -> bool {
    !state.is_paused()
}

fn advance_time(mut state: ResMut<SimulationState>) {
    state.game_time += 1;
    if state.game_time % state.ticks_per_day as u64 == 0 {
        state.game_days += 1;
    }
}
```

**Step 2: Register simulation module in game/mod.rs**

In `src/game/mod.rs`, add:
- `pub mod simulation;` to module declarations
- `app.add_plugins(simulation::plugin);` in the `plugin` function
- Remove `game_time` and `game_days` fields from `GameState` in `resources.rs` (they move to `SimulationState`)

**Step 3: Replace old tick_passive_income system**

In `src/game/systems.rs`, remove the `tick_passive_income` system entirely. The new `advance_time` system in `simulation.rs` replaces it.

Remove the `tick_passive_income` system registration from `src/game/mod.rs`.

**Step 4: Update sidebar to read from SimulationState**

In `src/game/ui/sidebar.rs`, change the resources display section to read from `SimulationState` instead of `GameState`:
- `"Game Time: {sim_state.game_time}"`
- `"Days: {sim_state.game_days}"`
- Show speed: `"Speed: {sim_state.speed}x"` or `"Paused"` if speed == 0

**Step 5: Update save/load to use SimulationState**

In `src/game/save.rs`:
- Add `simulation_state: Option<SimulationState>` to `SaveData`
- Save `game_time` and `game_days` from `SimulationState`
- Restore them on load

**Step 6: Build and test**

Run: `cargo build`
Expected: Compiles. Game time now ticks via FixedUpdate. Days increment every 600 ticks.

**Step 7: Commit**

```bash
git add src/game/simulation.rs src/game/mod.rs src/game/systems.rs src/game/resources.rs src/game/ui/sidebar.rs src/game/save.rs
git commit -m "feat: add simulation tick system with game time and day cycle"
```

---

### Task 1.2: Speed Controls

**Files:**
- Modify: `src/game/simulation.rs`
- Modify: `src/game/ui/bottom_bar.rs`
- Modify: `src/screens/gameplay.rs`

**Step 1: Add speed control input system**

In `src/game/simulation.rs`, add a system that reads keyboard input for speed:

```rust
fn speed_controls(
    input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<SimulationState>,
) {
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
```

Register this system in the `plugin` function (runs in `Update`, not `FixedUpdate`, so it works while paused). Gate it to only run during `Screen::Gameplay`.

**Step 2: Adjust FixedUpdate timestep based on speed**

In the `advance_time` system (or a separate system before it), adjust `Time<Fixed>` period based on `SimulationState::speed`:

```rust
fn adjust_tick_rate(state: Res<SimulationState>, mut time: ResMut<Time<Fixed>>) {
    if state.is_changed() && !state.is_paused() {
        let period = Duration::from_secs_f64(1.0 / state.speed as f64);
        time.set_timestep(period);
    }
}
```

**Step 3: Add speed indicator to bottom bar or sidebar**

In the sidebar resources section, display current speed. Show buttons for Pause / 1x / 2x / 5x.

**Step 4: Update pause toggle in gameplay.rs**

In `src/screens/gameplay.rs`, change the P/Escape pause toggle to use `SimulationState::toggle_pause()` instead of `GameState::is_paused`. Remove `is_paused` from `GameState`.

**Step 5: Build and test**

Run: `cargo build`
Expected: Space pauses/unpauses. 1/2/3 keys set speed. Time ticks faster at higher speeds.

**Step 6: Commit**

```bash
git add src/game/simulation.rs src/game/ui/bottom_bar.rs src/screens/gameplay.rs src/game/resources.rs
git commit -m "feat: add speed controls (pause, 1x, 2x, 5x) with keyboard shortcuts"
```

---

### Task 1.3: Hunger Drain

**Files:**
- Modify: `src/game/character.rs`
- Modify: `src/game/simulation.rs`

**Step 1: Add hunger drain system**

Characters' hunger decreases over time. When hunger hits 0, health starts draining.

```rust
fn drain_hunger(
    mut characters: Query<(&mut Health, &CharacterInfo)>,
    sim: Res<SimulationState>,
) {
    // Drain hunger every 10 ticks (every 10 game-seconds)
    if sim.game_time % 10 != 0 {
        return;
    }
    for (mut health, _info) in &mut characters {
        if health.hunger > 0 {
            health.hunger = health.hunger.saturating_sub(1);
        }
    }
}
```

Register in `SimulationSystems::UpdateEconomy` set.

**Step 2: Change Health::hunger default from 0 to 100 (full)**

In `src/game/character.rs`, change the `Health` default to start hunger at 100:

```rust
impl Default for Health {
    fn default() -> Self {
        Self {
            head: 100, stomach: 100, chest: 100,
            left_arm: 100, right_arm: 100,
            left_leg: 100, right_leg: 100,
            hunger: 100,
        }
    }
}
```

**Step 3: Build and test**

Run: `cargo build`
Expected: Characters start at 100 hunger, it ticks down over time.

**Step 4: Commit**

```bash
git add src/game/character.rs src/game/simulation.rs
git commit -m "feat: add hunger drain system, characters start at 100 hunger"
```

---

## Phase 2: Action System

**Goal:** Characters can be assigned actions. Actions execute over time, producing effects. Job queues allow repeating tasks.

---

### Task 2.1: Action & ActionState Components

**Files:**
- Create: `src/game/action.rs`
- Modify: `src/game/mod.rs`
- Modify: `src/game/character.rs`

**Step 1: Define Action enum and ActionState component**

Create `src/game/action.rs`:

```rust
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// A single action a character can perform
#[derive(Clone, Debug, Serialize, Deserialize, Reflect, PartialEq)]
pub enum Action {
    Idle,
    Gather {
        location: String,      // location ID
        resource: String,      // item ID to gather
    },
    Collect {
        location: String,
        item: String,
    },
    Deposit {
        item: String,
    },
    Travel {
        destination: String,   // location ID
    },
    Research {
        tech_id: String,
    },
    Craft {
        recipe_id: String,
        workstation: String,   // entity ID
    },
    Build {
        building: String,      // entity ID
    },
    Explore,
}

/// A repeating job (loops back to start when done)
#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct Job {
    pub name: String,
    pub actions: Vec<Action>,
}

/// Progress tracker for the current action
#[derive(Clone, Debug, Default, Serialize, Deserialize, Reflect)]
pub struct ActionProgress {
    pub current: u32,
    pub required: u32,
}

impl ActionProgress {
    pub fn new(required: u32) -> Self {
        Self { current: 0, required }
    }

    pub fn tick(&mut self) -> bool {
        self.current += 1;
        self.current >= self.required
    }

    pub fn fraction(&self) -> f32 {
        if self.required == 0 { return 1.0; }
        self.current as f32 / self.required as f32
    }
}

/// Attached to each character entity
#[derive(Component, Clone, Debug, Default, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct ActionState {
    pub current_action: Option<Action>,
    pub progress: ActionProgress,
    pub action_queue: VecDeque<Action>,
    pub job_queue: Vec<Job>,
    pub current_job_index: usize,
}

impl ActionState {
    /// Push an action to the back of the queue
    pub fn queue_action(&mut self, action: Action) {
        self.action_queue.push_back(action);
    }

    /// Clear current action and queue
    pub fn clear(&mut self) {
        self.current_action = None;
        self.progress = ActionProgress::default();
        self.action_queue.clear();
    }

    /// Clear only the job queue
    pub fn clear_jobs(&mut self) {
        self.job_queue.clear();
        self.current_job_index = 0;
    }
}

pub fn plugin(app: &mut App) {
    app.register_type::<ActionState>();
}
```

**Step 2: Add ActionState to CharacterBundle**

In `src/game/character.rs`, add `ActionState` to `CharacterBundle`:

```rust
pub action_state: ActionState,
```

Initialize it as `ActionState::default()` in `CharacterBundle::new()`.

**Step 3: Register action module**

In `src/game/mod.rs`:
- `pub mod action;`
- `app.add_plugins(action::plugin);`

**Step 4: Update save/load**

In `src/game/save.rs`, add `action_state: ActionState` to `SerializedCharacter`. Load and save it alongside other components.

**Step 5: Build and test**

Run: `cargo build`
Expected: Characters now have an `ActionState` component. No actions execute yet.

**Step 6: Commit**

```bash
git add src/game/action.rs src/game/mod.rs src/game/character.rs src/game/save.rs
git commit -m "feat: add Action enum and ActionState component for character actions"
```

---

### Task 2.2: Action Dequeue & Idle System

**Files:**
- Modify: `src/game/action.rs`

**Step 1: Add dequeue system**

When a character has no current action, pop from the action queue. If the action queue is empty, check the job queue and cycle through its actions.

```rust
fn dequeue_actions(mut characters: Query<&mut ActionState>) {
    for mut state in &mut characters {
        if state.current_action.is_some() {
            continue;
        }

        // Try action queue first
        if let Some(action) = state.action_queue.pop_front() {
            state.current_action = Some(action);
            state.progress = ActionProgress::default();
            continue;
        }

        // Try job queue (loops)
        if !state.job_queue.is_empty() {
            let job = &state.job_queue[state.current_job_index % state.job_queue.len()];
            if !job.actions.is_empty() {
                // Queue all actions from this job
                for action in &job.actions {
                    state.action_queue.push_back(action.clone());
                }
                state.current_job_index += 1;
                // Pop the first one as current
                if let Some(action) = state.action_queue.pop_front() {
                    state.current_action = Some(action);
                    state.progress = ActionProgress::default();
                }
            }
            continue;
        }

        // Default to idle
        state.current_action = Some(Action::Idle);
        state.progress = ActionProgress::default();
    }
}
```

Register in `SimulationSystems::ProcessActions`.

**Step 2: Build and test**

Run: `cargo build`
Expected: All characters start with `Idle` as current action.

**Step 3: Commit**

```bash
git add src/game/action.rs
git commit -m "feat: add action dequeue system with job queue cycling"
```

---

### Task 2.3: Location Component & Travel Action

**Files:**
- Modify: `src/game/location.rs`
- Modify: `src/game/character.rs`
- Modify: `src/game/action.rs`

**Step 1: Expand Location system**

Rewrite `src/game/location.rs` to support resource nodes:

```rust
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, Reflect, PartialEq, Eq, Hash)]
pub enum LocationType {
    Base,
    Mine,
    Forest,
    Ruins,
    City,
    Wilderness,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct LocationInfo {
    pub id: String,
    pub name: String,
    pub loc_type: LocationType,
    pub distance: u32,  // travel time in ticks from base
}

#[derive(Component, Clone, Debug, Default, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct LocationResources {
    pub resource_type: String,       // item ID that can be gathered here
    pub capacity: u32,               // max amount
    pub yield_rate: u32,             // progress per gather tick
    pub current_amount: u32,         // how much is available
}

#[derive(Component, Clone, Debug, Default, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct LocationInventory {
    pub items: HashMap<String, u32>, // items sitting at this location
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct LocationRegistry {
    pub locations: HashMap<String, Entity>,
}
```

**Step 2: Add CharacterLocation component**

In `src/game/character.rs`, replace the `location: String` field in `CharacterInfo` with a proper component:

```rust
#[derive(Component, Clone, Debug, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct CharacterLocation {
    pub location_id: String,
}
```

Add this to `CharacterBundle`.

**Step 3: Add travel processor**

In `src/game/action.rs`, add:

```rust
fn process_travel(
    mut characters: Query<(&mut ActionState, &mut CharacterLocation)>,
) {
    for (mut state, mut location) in &mut characters {
        let Some(Action::Travel { ref destination }) = state.current_action else {
            continue;
        };

        // First tick: set required progress (we'll look up distance later)
        if state.progress.required == 0 {
            state.progress = ActionProgress::new(30); // default 30 ticks
        }

        if state.progress.tick() {
            location.location_id = destination.clone();
            state.current_action = None;
        }
    }
}
```

Register in `SimulationSystems::ProcessActions` after `dequeue_actions`.

**Step 4: Build and test**

Run: `cargo build`
Expected: Characters have a location, travel action moves them.

**Step 5: Commit**

```bash
git add src/game/location.rs src/game/character.rs src/game/action.rs
git commit -m "feat: add location system and travel action processor"
```

---

### Task 2.4: Gather, Collect, Deposit Actions

**Files:**
- Modify: `src/game/action.rs`
- Modify: `src/game/resources.rs`

**Step 1: Add BaseInventory resource**

In `src/game/resources.rs`, add:

```rust
#[derive(Resource, Clone, Debug, Default, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct BaseInventory {
    pub items: HashMap<String, u32>,
}

impl BaseInventory {
    pub fn add(&mut self, item: &str, amount: u32) {
        *self.items.entry(item.to_string()).or_insert(0) += amount;
    }

    pub fn remove(&mut self, item: &str, amount: u32) -> bool {
        if let Some(current) = self.items.get_mut(item) {
            if *current >= amount {
                *current -= amount;
                if *current == 0 {
                    self.items.remove(item);
                }
                return true;
            }
        }
        false
    }

    pub fn count(&self, item: &str) -> u32 {
        *self.items.get(item).unwrap_or(&0)
    }
}
```

Initialize this resource in `src/game/mod.rs`.

**Step 2: Add gather processor**

In `src/game/action.rs`:

```rust
fn process_gather(
    mut characters: Query<(&mut ActionState, &CharacterLocation)>,
    mut locations: Query<(&LocationInfo, &mut LocationResources, &mut LocationInventory)>,
) {
    for (mut state, char_loc) in &mut characters {
        let Some(Action::Gather { ref location, ref resource }) = state.current_action else {
            continue;
        };

        // Validate character is at the right location
        if char_loc.location_id != *location {
            // Need to travel first — clear action, will be handled by UI/AI
            state.current_action = None;
            continue;
        }

        // Find the location entity and check resources
        let mut found = false;
        for (loc_info, mut loc_res, mut loc_inv) in &mut locations {
            if loc_info.id == *location && loc_res.resource_type == *resource {
                if loc_res.current_amount > 0 {
                    // Progress gathering
                    if state.progress.required == 0 {
                        state.progress = ActionProgress::new(100); // 100 ticks to gather
                    }
                    if state.progress.tick() {
                        let amount = loc_res.yield_rate.min(loc_res.current_amount);
                        loc_res.current_amount = loc_res.current_amount.saturating_sub(amount);
                        *loc_inv.items.entry(resource.clone()).or_insert(0) += amount;
                        state.current_action = None; // done, dequeue will pick next
                    }
                    found = true;
                }
                break;
            }
        }
        if !found {
            state.current_action = None; // can't gather, skip
        }
    }
}
```

**Step 3: Add collect processor**

```rust
fn process_collect(
    mut characters: Query<(&mut ActionState, &CharacterLocation, &mut Inventory)>,
    mut locations: Query<(&LocationInfo, &mut LocationInventory)>,
) {
    for (mut state, char_loc, mut inventory) in &mut characters {
        let Some(Action::Collect { ref location, ref item }) = state.current_action else {
            continue;
        };

        if char_loc.location_id != *location {
            state.current_action = None;
            continue;
        }

        for (loc_info, mut loc_inv) in &mut locations {
            if loc_info.id == *location {
                if let Some(amount) = loc_inv.items.remove(item) {
                    *inventory.items
                        .entry(item.clone())
                        .or_insert("0".to_string()) = amount.to_string();
                }
                break;
            }
        }
        state.current_action = None;
    }
}
```

**Step 4: Add deposit processor**

```rust
fn process_deposit(
    mut characters: Query<(&mut ActionState, &CharacterLocation, &mut Inventory)>,
    mut base_inv: ResMut<BaseInventory>,
) {
    for (mut state, char_loc, mut inventory) in &mut characters {
        let Some(Action::Deposit { ref item }) = state.current_action else {
            continue;
        };

        // Must be at base
        if char_loc.location_id != "base" {
            state.current_action = None;
            continue;
        }

        if let Some(amount_str) = inventory.items.remove(item) {
            if let Ok(amount) = amount_str.parse::<u32>() {
                base_inv.add(item, amount);
            }
        }
        state.current_action = None;
    }
}
```

Register all three processors in `SimulationSystems::ProcessActions`.

**Step 5: Build and test**

Run: `cargo build`
Expected: Full gather → collect → deposit pipeline compiles.

**Step 6: Commit**

```bash
git add src/game/action.rs src/game/resources.rs src/game/mod.rs
git commit -m "feat: add gather, collect, deposit action processors and BaseInventory"
```

---

### Task 2.5: Skill XP Gain

**Files:**
- Modify: `src/game/action.rs`
- Modify: `src/game/character.rs`

**Step 1: Add XP gain system**

After actions process, grant XP based on what the character is doing:

```rust
fn apply_skill_xp(
    mut characters: Query<(&ActionState, &mut Skills)>,
) {
    for (state, mut skills) in &mut characters {
        let Some(ref action) = state.current_action else { continue };
        match action {
            Action::Gather { .. } => {
                skills.labouring = skills.labouring.saturating_add(1);
            }
            Action::Research { .. } => {
                skills.science = skills.science.saturating_add(1);
            }
            Action::Explore => {
                skills.scouting = skills.scouting.saturating_add(1);
                skills.athletics = skills.athletics.saturating_add(1);
            }
            Action::Travel { .. } => {
                skills.athletics = skills.athletics.saturating_add(1);
            }
            _ => {}
        }
    }
}
```

Register in `SimulationSystems::ProcessActions` after the action processors.

**Step 2: Update xp_to_level formula**

In `src/game/ui/inspector.rs`, change `xp_to_level` to match the web prototype:

```rust
fn xp_to_level(xp: u8) -> u32 {
    let xp = xp as f64;
    ((xp * 4.0 / 5.0).cbrt().floor() as u32 + 1).min(100)
}
```

Note: The `u8` XP type will cap at 255. Consider changing `Skills` fields to `u32` for long-term play. This is a redesign decision — the web prototype uses unbounded numbers.

**Step 3: Build and test**

Run: `cargo build`
Expected: Characters gain XP in relevant skills while performing actions.

**Step 4: Commit**

```bash
git add src/game/action.rs src/game/character.rs src/game/ui/inspector.rs
git commit -m "feat: add skill XP gain from actions, update level formula"
```

---

## Phase 3: Economy & Resources

**Goal:** Resources display in sidebar/dashboard. Base inventory tracks all items. Zeni can be earned and spent.

---

### Task 3.1: Resource Display in Sidebar

**Files:**
- Modify: `src/game/ui/sidebar.rs`

**Step 1: Show base resources in sidebar**

Read from `BaseInventory` and `BaseState` to display:
- Zeni (currency)
- Key resources: lumber, stone, iron, copper (counts from BaseInventory)
- Power: generation / consumption
- Research Level / Tech Level

Replace the current placeholder "Current Level" / "Game Time" / "Days" with meaningful resource info and simulation state.

**Step 2: Build and test**

Run: `cargo build`
Expected: Sidebar shows real resource values.

**Step 3: Commit**

```bash
git add src/game/ui/sidebar.rs
git commit -m "feat: display base resources, zeni, and power in sidebar"
```

---

### Task 3.2: Dashboard View Content

**Files:**
- Modify: `src/game/ui/content/dashboard.rs`

**Step 1: Populate dashboard**

Replace the empty stub with a useful overview:
- **Resources section**: All items in BaseInventory with counts
- **Workers section**: How many characters are idle vs. working (read ActionState)
- **Research section**: Currently researching tech (if any), progress bar
- **Recent notifications**: Last 5 notifications

**Step 2: Build and test**

Run: `cargo build`
Expected: Dashboard shows a meaningful overview of game state.

**Step 3: Commit**

```bash
git add src/game/ui/content/dashboard.rs
git commit -m "feat: populate dashboard with resources, workers, and research overview"
```

---

### Task 3.3: Inspector Tab Switching Bug Fix

**Files:**
- Modify: `src/game/ui/inspector.rs`

**Step 1: Render the tab bar**

The `tab_button` function exists (line 168) but is never called. Add a horizontal row of tab buttons above the tab content in the `CharacterInspector::construct` method:

```rust
// Tab bar row
ui.ch().row().gap(4).pad(8).build(|ui| {
    tab_button(ui, "Health", InspectorTab::Health, inspector_state.active_tab);
    tab_button(ui, "Equipment", InspectorTab::Equipment, inspector_state.active_tab);
    tab_button(ui, "Skills", InspectorTab::Skills, inspector_state.active_tab);
    tab_button(ui, "Inventory", InspectorTab::Inventory, inspector_state.active_tab);
});
```

Wire the button clicks to update `inspector_state.active_tab`.

**Step 2: Build and test**

Run: `cargo build`
Expected: Inspector shows clickable tab buttons. Clicking switches between Health/Equipment/Skills/Inventory.

**Step 3: Commit**

```bash
git add src/game/ui/inspector.rs
git commit -m "fix: render inspector tab bar buttons for tab switching"
```

---

## Phase 4: Data-Driven Content (RON)

**Goal:** Items, races, and scenarios load from RON files instead of hardcoded Rust.

---

### Task 4.1: RON Asset Loader Infrastructure

**Files:**
- Create: `src/game/data.rs`
- Modify: `src/game/mod.rs`
- Modify: `Cargo.toml`

**Step 1: Add RON dependency**

In `Cargo.toml`, add:
```toml
ron = "0.8"
```

**Step 2: Create data module with RON loading helpers**

Create `src/game/data.rs` with types for loading RON assets:

```rust
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Item definition loaded from items.ron
#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct ItemDef {
    pub id: String,
    pub name: String,
    pub price: u32,
    pub weight: u32,
    pub item_type: String,        // "trade", "food", "weapon", "armor", etc.
    pub nutrition: Option<u32>,    // for food items
}

/// Race/subrace XP multipliers loaded from races.ron
#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct SubraceDef {
    pub race: String,
    pub subrace: String,
    pub xp_multipliers: HashMap<String, f32>,  // skill_name → multiplier
}

/// All game data loaded from RON files
#[derive(Resource, Clone, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct GameData {
    pub items: HashMap<String, ItemDef>,
    pub races: Vec<SubraceDef>,
}

impl GameData {
    pub fn get_item(&self, id: &str) -> Option<&ItemDef> {
        self.items.get(id)
    }

    pub fn get_xp_multiplier(&self, race: &str, subrace: &str, skill: &str) -> f32 {
        self.races
            .iter()
            .find(|r| r.race == race && r.subrace == subrace)
            .and_then(|r| r.xp_multipliers.get(skill))
            .copied()
            .unwrap_or(1.0)
    }
}

pub fn plugin(app: &mut App) {
    app.insert_resource(GameData::default())
        .register_type::<GameData>();
}
```

Register in `src/game/mod.rs`.

**Step 3: Build and test**

Run: `cargo build`
Expected: GameData resource exists, empty for now.

**Step 4: Commit**

```bash
git add src/game/data.rs src/game/mod.rs Cargo.toml
git commit -m "feat: add RON data module with ItemDef, SubraceDef, GameData resource"
```

---

### Task 4.2: Create RON Data Files

**Files:**
- Create: `assets/data/items.ron`
- Create: `assets/data/races.ron`

**Step 1: Create items.ron**

Port a representative subset of items from the web prototype's `src/constants/items.ts` (~1,300 items). Start with the core resources and food, expand later:

```ron
// assets/data/items.ron
[
    (id: "lumber", name: "Lumber", price: 10, weight: 1, item_type: "trade", nutrition: None),
    (id: "stone", name: "Stone", price: 36, weight: 1, item_type: "trade", nutrition: None),
    (id: "iron_ore", name: "Iron Ore", price: 90, weight: 1, item_type: "trade", nutrition: None),
    (id: "copper_ore", name: "Copper Ore", price: 180, weight: 4, item_type: "trade", nutrition: None),
    (id: "raw_meat", name: "Raw Meat", price: 60, weight: 1, item_type: "food", nutrition: Some(30)),
    (id: "bread", name: "Bread", price: 488, weight: 1, item_type: "food", nutrition: Some(60)),
    // ... port remaining items from web prototype
]
```

**Step 2: Create races.ron**

Port all race data from the web prototype's `src/constants/races.ts`:

```ron
// assets/data/races.ron
[
    (race: "human", subrace: "greenlander", xp_multipliers: {
        "science": 1.2, "cooking": 1.2, "farming": 1.2,
        // all other skills default to 1.0
    }),
    (race: "human", subrace: "scorchlander", xp_multipliers: {
        "dexterity": 1.1, "strength": 0.9, "athletics": 1.2,
        "dodge": 1.2, "stealth": 1.2, "armour_smith": 1.2,
        "weapon_smith": 1.2, "cooking": 0.8, "labouring": 0.8, "farming": 0.8,
    }),
    // ... port remaining races
]
```

**Step 3: Load RON files at startup**

In `src/game/data.rs`, add a startup system that reads and parses the RON files:

```rust
fn load_game_data(mut game_data: ResMut<GameData>) {
    // Load items
    let items_str = include_str!("../../assets/data/items.ron");
    let items: Vec<ItemDef> = ron::from_str(items_str).expect("Failed to parse items.ron");
    for item in items {
        game_data.items.insert(item.id.clone(), item);
    }

    // Load races
    let races_str = include_str!("../../assets/data/races.ron");
    let races: Vec<SubraceDef> = ron::from_str(races_str).expect("Failed to parse races.ron");
    game_data.races = races;
}
```

Register as a `Startup` system.

**Step 4: Build and test**

Run: `cargo build`
Expected: GameData populated at startup with items and races.

**Step 5: Commit**

```bash
git add assets/data/items.ron assets/data/races.ron src/game/data.rs
git commit -m "feat: create RON data files for items and races, load at startup"
```

---

### Task 4.3: Use Race Multipliers in XP Gain

**Files:**
- Modify: `src/game/action.rs`

**Step 1: Update apply_skill_xp to use race multipliers**

Read `GameData` and `CharacterInfo` to apply race-specific XP multipliers:

```rust
fn apply_skill_xp(
    mut characters: Query<(&ActionState, &mut Skills, &CharacterInfo)>,
    game_data: Res<GameData>,
) {
    for (state, mut skills, info) in &mut characters {
        let Some(ref action) = state.current_action else { continue };
        let skill_name = match action {
            Action::Gather { .. } => "labouring",
            Action::Research { .. } => "science",
            Action::Explore => "scouting",
            Action::Travel { .. } => "athletics",
            _ => continue,
        };
        let multiplier = game_data.get_xp_multiplier(&info.race, &info.subrace, skill_name);
        let xp_gain = (1.0 * multiplier) as u8;
        // Apply to the matching skill field
        match skill_name {
            "labouring" => skills.labouring = skills.labouring.saturating_add(xp_gain),
            "science" => skills.science = skills.science.saturating_add(xp_gain),
            "scouting" => skills.scouting = skills.scouting.saturating_add(xp_gain),
            "athletics" => skills.athletics = skills.athletics.saturating_add(xp_gain),
            _ => {}
        }
    }
}
```

**Step 2: Build and test**

Run: `cargo build`
Expected: XP gain now scaled by race multipliers.

**Step 3: Commit**

```bash
git add src/game/action.rs
git commit -m "feat: apply race-based XP multipliers to skill gain"
```

---

## Phase 5: Research System

**Goal:** Tech tree loads from RON with ~200 nodes. Characters can research. Unlocks have downstream effects.

---

### Task 5.1: Port Research Data to RON

**Files:**
- Create: `assets/data/research.ron`
- Modify: `src/game/research.rs`
- Modify: `src/game/data.rs`

**Step 1: Define ResearchDef type**

In `src/game/data.rs`, add:

```rust
#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct ResearchDef {
    pub id: String,
    pub name: String,
    pub research_type: String,   // "basic", "core", "crafting", etc.
    pub tech_level: u32,
    pub cost: HashMap<String, u32>,   // item_id → amount
    pub time: u32,                     // ticks to complete
    pub prerequisites: Vec<String>,
    pub effects: Vec<ResearchEffect>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub enum ResearchEffect {
    UnlocksBuilding(String),
    UnlocksRecipe(String),
    SetsTechLevel(u32),
}
```

Add `pub research: HashMap<String, ResearchDef>` to `GameData`.

**Step 2: Create research.ron**

Port the ~200 research items from the web prototype's `src/constants/research.ts`:

```ron
// assets/data/research.ron
[
    (id: "gear_storage", name: "Gear Storage", research_type: "core", tech_level: 1,
     cost: {}, time: 14400, prerequisites: [], effects: []),
    (id: "item_storage", name: "Item Storage", research_type: "core", tech_level: 1,
     cost: {}, time: 14400, prerequisites: [], effects: []),
    (id: "small_house", name: "Small House", research_type: "core", tech_level: 1,
     cost: {"books": 2}, time: 21600, prerequisites: [], effects: []),
    (id: "technology_2", name: "Technology 2", research_type: "basic", tech_level: 1,
     cost: {"books": 6}, time: 21600, prerequisites: ["small_house"],
     effects: [SetsTechLevel(2)]),
    // ... port remaining ~200 entries
]
```

**Step 3: Rewrite research.rs to use data-driven approach**

Replace the hardcoded `TechId` enum and `TechTree` with a data-driven system that reads from `GameData::research`.

Rewrite `ResearchState` to use string IDs instead of the `TechId` enum:

```rust
#[derive(Resource, Clone, Debug, Default, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct ResearchState {
    pub unlocked: HashSet<String>,
    pub current_research: Option<String>,
    pub research_progress: u32,
}

impl ResearchState {
    pub fn is_unlocked(&self, id: &str) -> bool {
        self.unlocked.contains(id)
    }

    pub fn can_research(&self, id: &str, game_data: &GameData) -> bool {
        if self.unlocked.contains(id) { return false; }
        if let Some(def) = game_data.research.get(id) {
            def.prerequisites.iter().all(|dep| self.unlocked.contains(dep))
        } else {
            false
        }
    }
}
```

**Step 4: Load research data at startup**

Add to the `load_game_data` function.

**Step 5: Update research UI to use GameData**

In `src/game/ui/content/research.rs`, change from reading `TechTree` to reading `GameData::research`. Group by `tech_level` instead of `tier`. Show cost as item names + amounts.

**Step 6: Build and test**

Run: `cargo build`
Expected: Research view shows ~200 techs grouped by tech level.

**Step 7: Commit**

```bash
git add assets/data/research.ron src/game/research.rs src/game/data.rs src/game/ui/content/research.rs
git commit -m "feat: port research system to data-driven RON with ~200 tech nodes"
```

---

### Task 5.2: Research Action Processor

**Files:**
- Modify: `src/game/action.rs`

**Step 1: Add research processor**

```rust
fn process_research(
    mut characters: Query<(&mut ActionState, &CharacterLocation, &Skills)>,
    mut research: ResMut<ResearchState>,
    game_data: Res<GameData>,
    mut base_inv: ResMut<BaseInventory>,
) {
    for (mut state, char_loc, skills) in &mut characters {
        let Some(Action::Research { ref tech_id }) = state.current_action else {
            continue;
        };

        // Must be at base
        if char_loc.location_id != "base" {
            state.current_action = None;
            continue;
        }

        // Start research if not already in progress
        if research.current_research.as_ref() != Some(tech_id) {
            if !research.can_research(tech_id, &game_data) {
                state.current_action = None;
                continue;
            }
            // Deduct costs
            if let Some(def) = game_data.research.get(tech_id) {
                for (item, amount) in &def.cost {
                    if !base_inv.remove(item, *amount) {
                        state.current_action = None;
                        continue;
                    }
                }
                research.current_research = Some(tech_id.clone());
                research.research_progress = 0;
                state.progress = ActionProgress::new(def.time);
            }
        }

        // Progress research (science skill speeds it up)
        let science_level = (skills.science as f32).sqrt() as u32;
        let speed_bonus = 1 + science_level / 10;
        for _ in 0..speed_bonus {
            research.research_progress += 1;
        }

        if state.progress.tick() {
            // Research complete
            research.unlocked.insert(tech_id.clone());
            research.current_research = None;
            research.research_progress = 0;

            // Apply effects
            if let Some(def) = game_data.research.get(tech_id) {
                for effect in &def.effects {
                    match effect {
                        ResearchEffect::SetsTechLevel(level) => {
                            // Update base state tech level
                        }
                        _ => {}
                    }
                }
            }

            state.current_action = None;
        }
    }
}
```

Register in `SimulationSystems::ProcessActions`.

**Step 2: Build and test**

Run: `cargo build`
Expected: Characters at base can research techs, consuming items and time.

**Step 3: Commit**

```bash
git add src/game/action.rs
git commit -m "feat: add research action processor with science skill speed bonus"
```

---

## Phase 6: Crafting System

**Goal:** Recipes define input → output transformations. Workstations are entities. Characters can craft.

---

### Task 6.1: Recipe Definitions

**Files:**
- Create: `assets/data/recipes.ron`
- Modify: `src/game/data.rs`

**Step 1: Define RecipeDef type**

In `src/game/data.rs`:

```rust
#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct RecipeDef {
    pub id: String,
    pub name: String,
    pub inputs: HashMap<String, u32>,    // item_id → amount needed
    pub outputs: HashMap<String, u32>,   // item_id → amount produced
    pub workstation: String,             // required workstation type
    pub time: u32,                       // ticks to craft
    pub skill: String,                   // skill used (for XP)
    pub required_research: Option<String>, // research ID needed
}
```

Add `pub recipes: HashMap<String, RecipeDef>` to `GameData`.

**Step 2: Create recipes.ron**

Port recipes from the web prototype's entity crafting stats:

```ron
// assets/data/recipes.ron
[
    (id: "iron_plates", name: "Iron Plates", workstation: "basic_forge",
     inputs: {"iron_ore": 2}, outputs: {"iron_plates": 1},
     time: 60, skill: "labouring", required_research: None),
    (id: "copper_plates", name: "Copper Plates", workstation: "basic_forge",
     inputs: {"copper_ore": 2}, outputs: {"copper_plates": 1},
     time: 60, skill: "labouring", required_research: None),
    (id: "building_materials", name: "Building Materials", workstation: "basic_forge",
     inputs: {"iron_plates": 1, "lumber": 2}, outputs: {"building_materials": 1},
     time: 90, skill: "engineer", required_research: None),
    // ... port remaining recipes
]
```

**Step 3: Load recipes at startup**

Add to `load_game_data`.

**Step 4: Build and test**

Run: `cargo build`

**Step 5: Commit**

```bash
git add assets/data/recipes.ron src/game/data.rs
git commit -m "feat: add recipe definitions in RON with crafting data"
```

---

### Task 6.2: Craft Action Processor

**Files:**
- Modify: `src/game/action.rs`

**Step 1: Add craft processor**

```rust
fn process_craft(
    mut characters: Query<(&mut ActionState, &CharacterLocation, &mut Skills)>,
    game_data: Res<GameData>,
    mut base_inv: ResMut<BaseInventory>,
) {
    for (mut state, char_loc, mut skills) in &mut characters {
        let Some(Action::Craft { ref recipe_id, ref workstation }) = state.current_action else {
            continue;
        };

        // Must be at base
        if char_loc.location_id != "base" {
            state.current_action = None;
            continue;
        }

        let Some(recipe) = game_data.recipes.get(recipe_id) else {
            state.current_action = None;
            continue;
        };

        // First tick: consume inputs
        if state.progress.required == 0 {
            // Check and consume all inputs
            for (item, amount) in &recipe.inputs {
                if base_inv.count(item) < *amount {
                    state.current_action = None;
                    break;
                }
            }
            if state.current_action.is_none() { continue; }

            for (item, amount) in &recipe.inputs {
                base_inv.remove(item, *amount);
            }
            state.progress = ActionProgress::new(recipe.time);
        }

        if state.progress.tick() {
            // Produce outputs
            for (item, amount) in &recipe.outputs {
                base_inv.add(item, *amount);
            }
            state.current_action = None;
        }
    }
}
```

Register in `SimulationSystems::ProcessActions`.

**Step 2: Build and test**

Run: `cargo build`

**Step 3: Commit**

```bash
git add src/game/action.rs
git commit -m "feat: add craft action processor consuming inputs and producing outputs"
```

---

## Phase 7: Building System

**Goal:** Buildings can be constructed by workers. Completed buildings provide workstations, storage, power.

---

### Task 7.1: Building Definitions & Entities

**Files:**
- Create: `assets/data/buildings.ron`
- Create: `src/game/building.rs`
- Modify: `src/game/data.rs`
- Modify: `src/game/mod.rs`

**Step 1: Define BuildingDef type**

In `src/game/data.rs`:

```rust
#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct BuildingDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,         // "tech", "power", "storage", "farming", etc.
    pub cost: HashMap<String, u32>,
    pub build_time: u32,          // ticks
    pub tech_level: u32,
    pub required_research: Vec<String>,
    pub power_generation: i32,    // positive = generates, negative = consumes
    pub max_workers: u32,
    pub provides_workstation: Option<String>,  // workstation type ID
    pub provides_storage: u32,    // extra storage capacity
}
```

Add `pub buildings: HashMap<String, BuildingDef>` to `GameData`.

**Step 2: Create building components**

Create `src/game/building.rs`:

```rust
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Debug, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct Building {
    pub def_id: String,
    pub name: String,
    pub progress: u32,
    pub required: u32,
    pub complete: bool,
    pub workers: Vec<String>,  // character IDs currently building
}

pub fn plugin(app: &mut App) {
    app.register_type::<Building>();
}
```

**Step 3: Create buildings.ron**

Port building definitions from the web prototype's entities.ts.

**Step 4: Build and test**

Run: `cargo build`

**Step 5: Commit**

```bash
git add assets/data/buildings.ron src/game/building.rs src/game/data.rs src/game/mod.rs
git commit -m "feat: add building system with definitions, components, and RON data"
```

---

### Task 7.2: Build Action Processor

**Files:**
- Modify: `src/game/action.rs`

**Step 1: Add build processor**

Similar to craft but operates on Building entities:

```rust
fn process_build(
    mut characters: Query<(&mut ActionState, &CharacterLocation)>,
    mut buildings: Query<&mut Building>,
) {
    for (mut state, char_loc) in &mut characters {
        let Some(Action::Build { ref building }) = state.current_action else {
            continue;
        };

        if char_loc.location_id != "base" {
            state.current_action = None;
            continue;
        }

        // Find building entity and contribute progress
        for mut bldg in &mut buildings {
            if bldg.def_id == *building && !bldg.complete {
                bldg.progress += 1;
                if bldg.progress >= bldg.required {
                    bldg.complete = true;
                }
                break;
            }
        }

        // Check if building is done
        let done = buildings.iter().any(|b| b.def_id == *building && b.complete);
        if done {
            state.current_action = None;
        }
    }
}
```

**Step 2: Build and test**

Run: `cargo build`

**Step 3: Commit**

```bash
git add src/game/action.rs
git commit -m "feat: add build action processor for constructing buildings"
```

---

### Task 7.3: Power Grid System

**Files:**
- Modify: `src/game/simulation.rs`
- Modify: `src/game/building.rs`

**Step 1: Add power update system**

Sum power generation and consumption across all completed buildings:

```rust
fn update_power_grid(
    buildings: Query<&Building>,
    game_data: Res<GameData>,
    mut base_state: ResMut<BaseState>,
) {
    let mut generation = 0i32;
    let mut consumption = 0i32;

    for building in &buildings {
        if !building.complete { continue; }
        if let Some(def) = game_data.buildings.get(&building.def_id) {
            if def.power_generation > 0 {
                generation += def.power_generation;
            } else {
                consumption += def.power_generation.abs();
            }
        }
    }

    base_state.power.generation = generation as u32;
    base_state.power.consumption = consumption as u32;
    base_state.power.current = generation.saturating_sub(consumption).max(0) as u32;
}
```

Register in `SimulationSystems::UpdateEconomy`.

**Step 2: Build and test**

Run: `cargo build`

**Step 3: Commit**

```bash
git add src/game/simulation.rs src/game/building.rs
git commit -m "feat: add power grid system summing building generation/consumption"
```

---

## Phase 8: World & Exploration

**Goal:** Locations exist as entities. Characters can travel and explore to discover new locations.

---

### Task 8.1: Spawn Starting Locations

**Files:**
- Create: `assets/data/locations.ron`
- Modify: `src/game/location.rs`
- Modify: `src/game/scenarios.rs`

**Step 1: Define location data in RON**

```ron
// assets/data/locations.ron
[
    (id: "base", name: "Home Base", loc_type: Base, distance: 0,
     resource_type: "", capacity: 0, yield_rate: 0),
    (id: "copper_mine", name: "Copper Mine", loc_type: Mine, distance: 30,
     resource_type: "copper_ore", capacity: 500, yield_rate: 1),
    (id: "iron_mine", name: "Iron Mine", loc_type: Mine, distance: 45,
     resource_type: "iron_ore", capacity: 300, yield_rate: 1),
    (id: "stone_quarry", name: "Stone Quarry", loc_type: Mine, distance: 20,
     resource_type: "stone", capacity: 1000, yield_rate: 2),
    (id: "forest", name: "Forest", loc_type: Forest, distance: 15,
     resource_type: "lumber", capacity: 800, yield_rate: 2),
]
```

**Step 2: Spawn location entities in apply_scenario**

In `src/game/scenarios.rs`, after spawning characters, spawn location entities from the RON data and register them in `LocationRegistry`.

**Step 3: Build and test**

Run: `cargo build`
Expected: Locations exist as entities at game start.

**Step 4: Commit**

```bash
git add assets/data/locations.ron src/game/location.rs src/game/scenarios.rs
git commit -m "feat: spawn starting locations from RON data on new game"
```

---

### Task 8.2: Location View UI

**Files:**
- Create: `src/game/ui/content/locations.rs`
- Modify: `src/game/ui/content/mod.rs`
- Modify: `src/game/resources.rs`

**Step 1: Add Locations to GameView**

Add `Locations` variant to the `GameView` enum in `resources.rs`.

**Step 2: Create locations view**

Create a new content view showing all known locations:
- Location name, type, distance
- Resources available (type, current/capacity)
- Characters currently at this location

**Step 3: Add nav button in sidebar**

Add a "Locations" navigation button to the sidebar.

**Step 4: Build and test**

Run: `cargo build`

**Step 5: Commit**

```bash
git add src/game/ui/content/locations.rs src/game/ui/content/mod.rs src/game/resources.rs src/game/ui/sidebar.rs
git commit -m "feat: add locations view showing known locations and their resources"
```

---

### Task 8.3: Exploration Action Processor

**Files:**
- Modify: `src/game/action.rs`
- Modify: `src/game/location.rs`

**Step 1: Add discoverable locations**

Add a `discovered: bool` field to `LocationInfo`. Starting locations are discovered. Others start undiscovered and are revealed by exploration.

**Step 2: Add explore processor**

```rust
fn process_explore(
    mut characters: Query<(&mut ActionState, &Skills)>,
    mut locations: Query<&mut LocationInfo>,
    mut notifications: ResMut<NotificationState>,
) {
    for (mut state, skills) in &mut characters {
        let Some(Action::Explore) = state.current_action else { continue };

        if state.progress.required == 0 {
            let scouting_level = (skills.scouting as f32).sqrt() as u32;
            let time = 200u32.saturating_sub(scouting_level * 5).max(50);
            state.progress = ActionProgress::new(time);
        }

        if state.progress.tick() {
            // Try to discover an undiscovered location
            for mut loc in &mut locations {
                if !loc.discovered {
                    loc.discovered = true;
                    notifications.push(
                        format!("Discovered: {}!", loc.name),
                        NotificationLevel::Success,
                    );
                    break;
                }
            }
            state.current_action = None;
        }
    }
}
```

**Step 3: Build and test**

Run: `cargo build`

**Step 4: Commit**

```bash
git add src/game/action.rs src/game/location.rs
git commit -m "feat: add exploration action that discovers new locations"
```

---

## Phase 9: Action Assignment UI

**Goal:** Player can assign actions to characters through the UI.

---

### Task 9.1: Action Assignment from Context Menu

**Files:**
- Modify: `src/game/ui/context_menu.rs`
- Modify: `src/game/action.rs`

**Step 1: Wire context menu items to real actions**

Replace the no-op context menu handlers with actual action assignment:

- **"Gather"**: Opens a sub-menu of available locations/resources → queues `Action::Gather`
- **"Research"**: Opens a sub-menu of available techs → queues `Action::Research`
- **"Travel"**: Opens a sub-menu of known locations → queues `Action::Travel`
- **"Explore"**: Directly queues `Action::Explore`

For the initial implementation, use simple button lists rather than nested menus.

**Step 2: Show current action in character inspector**

In the inspector, add a section showing:
- Current action name and progress bar
- Action queue contents
- Job queue contents
- "Clear Actions" button
- "Clear Jobs" button

**Step 3: Build and test**

Run: `cargo build`
Expected: Right-click a character → assign actions → watch them execute over time.

**Step 4: Commit**

```bash
git add src/game/ui/context_menu.rs src/game/ui/inspector.rs
git commit -m "feat: wire context menu to assign real actions, show action status in inspector"
```

---

### Task 9.2: Job Assignment UI

**Files:**
- Modify: `src/game/ui/inspector.rs`

**Step 1: Add job queue management**

In the inspector, add a "Jobs" tab or section:
- "Add Job" button → select a preset job (e.g., "Gather Copper" = Travel to mine + Gather + Collect + Travel to base + Deposit)
- Display current job queue with reorder (up/down) and remove buttons
- Show which job is currently active

**Step 2: Create preset jobs**

Define common job presets as convenience helpers:

```rust
fn make_gather_job(location_id: &str, resource: &str) -> Job {
    Job {
        name: format!("Gather {}", resource),
        actions: vec![
            Action::Travel { destination: location_id.to_string() },
            Action::Gather { location: location_id.to_string(), resource: resource.to_string() },
            Action::Collect { location: location_id.to_string(), item: resource.to_string() },
            Action::Travel { destination: "base".to_string() },
            Action::Deposit { item: resource.to_string() },
        ],
    }
}
```

**Step 3: Build and test**

Run: `cargo build`
Expected: Player can assign repeating jobs. Characters autonomously cycle through gather loops.

**Step 4: Commit**

```bash
git add src/game/ui/inspector.rs src/game/action.rs
git commit -m "feat: add job queue management UI with preset gather jobs"
```

---

## Phase 10: Combat System

**Goal:** Characters can fight enemies. Combat uses melee/ranged skills. Damage applies to body parts.

---

### Task 10.1: Combat Data & Enemy Entities

**Files:**
- Create: `src/game/combat.rs`
- Modify: `src/game/mod.rs`

**Step 1: Define combat components**

```rust
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct CombatStats {
    pub melee_attack: u32,
    pub melee_defense: u32,
    pub dodge: u32,
    pub toughness: u32,
}

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct Enemy {
    pub name: String,
    pub level: u32,
}
```

**Step 2: Add combat processing system**

When two entities are in combat (same location, opposing factions):
- Each tick: attacker rolls attack vs. defender's defense
- On hit: damage applied to random body part
- Skills gain XP during combat

This is a large system — implement basic melee first, expand later.

**Step 3: Build and test**

Run: `cargo build`

**Step 4: Commit**

```bash
git add src/game/combat.rs src/game/mod.rs
git commit -m "feat: add basic combat system with melee attack/defense"
```

---

## Phase 11: Polish & Integration

**Goal:** All systems work together. Save/load handles all new state. UI is cohesive.

---

### Task 11.1: Update Save/Load for All New State

**Files:**
- Modify: `src/game/save.rs`

**Step 1: Update SaveData**

Add all new state to `SaveData`:
- `simulation_state: SimulationState`
- `base_inventory: BaseInventory`
- `research_state: ResearchState` (updated format with string IDs)
- `buildings: Vec<SerializedBuilding>`
- `locations: Vec<SerializedLocation>`

All new fields should be `Option<T>` for backwards compatibility with old saves.

**Step 2: Update save/load systems**

Save and restore all new resources and entities.

**Step 3: Build and test**

Run: `cargo build`
Test: Start game → play → save → load → verify all state restored.

**Step 4: Commit**

```bash
git add src/game/save.rs
git commit -m "feat: update save/load to handle all new game state"
```

---

### Task 11.2: Scenarios Update

**Files:**
- Modify: `src/game/scenarios.rs`
- Modify: `assets/data/scenarios.ron` (create)

**Step 1: Move scenarios to RON**

Port scenario definitions to `assets/data/scenarios.ron` with starting resources, characters, locations, and unlocked research.

**Step 2: Update apply_scenario to reset all new state**

Reset `SimulationState`, `BaseInventory`, `ResearchState`, despawn buildings and locations, then spawn fresh from scenario data.

**Step 3: Build and test**

Run: `cargo build`

**Step 4: Commit**

```bash
git add src/game/scenarios.rs assets/data/scenarios.ron
git commit -m "feat: move scenarios to RON, reset all game state on new game"
```

---

### Task 11.3: Final Integration & Balance Pass

**Files:**
- Various

**Step 1: Verify full game loop**

Test the complete loop:
1. Start new game → characters spawn at base with starting resources
2. Assign gather jobs → characters travel, gather, return, deposit
3. Accumulate resources → start research
4. Unlock techs → build workstations
5. Craft items → build buildings
6. Explore → discover new locations
7. Save/load → all state preserved

**Step 2: Balance pass**

Adjust timing constants:
- Gather time, travel time, craft time, research time
- Hunger drain rate
- XP gain rates
- Resource yields

**Step 3: Commit**

```bash
git add -A
git commit -m "feat: final integration and balance pass for core game loop"
```

---

## Summary

| Phase | Tasks | Key Deliverables |
|---|---|---|
| **1 — Simulation** | 1.1–1.3 | Game tick, speed controls, hunger drain |
| **2 — Actions** | 2.1–2.5 | ActionState, dequeue, travel, gather/collect/deposit, XP |
| **3 — Economy** | 3.1–3.3 | Resource display, dashboard content, tab fix |
| **4 — Data** | 4.1–4.3 | RON loader, items/races data, race multipliers |
| **5 — Research** | 5.1–5.2 | ~200 techs from RON, research processor |
| **6 — Crafting** | 6.1–6.2 | Recipes from RON, craft processor |
| **7 — Building** | 7.1–7.3 | Building defs, build processor, power grid |
| **8 — World** | 8.1–8.3 | Location spawning, locations UI, exploration |
| **9 — UI** | 9.1–9.2 | Action assignment, job queue management |
| **10 — Combat** | 10.1 | Basic melee combat |
| **11 — Polish** | 11.1–11.3 | Save/load update, scenarios, balance |

**Total: ~30 tasks across 11 phases.**
