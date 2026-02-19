# Exploration System Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace deterministic sequential exploration with a weighted random outcome system that discovers landmarks, generates resource nodes, and rewards items/XP.

**Architecture:** New `ExplorationState` resource tracks meta-state. The `process_explore` system in `action.rs` is rewritten to roll against a weighted outcome table on completion. Generated locations are spawned as regular `LocationInfo` + `LocationResources` entities. New items are added to `items.ron` for exploration rewards.

**Tech Stack:** Rust, Bevy 0.17, `rand` crate (already a transitive dependency via Bevy), RON data files, serde for save/load.

---

### Task 1: Add Exploration Reward Items to Data

**Files:**
- Modify: `assets/data/items.ron`

**Step 1: Add new item definitions**

Add these items to the end of the array in `assets/data/items.ron`, before the closing `]`:

```ron
    (id: "herbs", name: "Herbs", price: 20, weight: 1, item_type: "trade", nutrition: None),
    (id: "scrap_wood", name: "Scrap Wood", price: 5, weight: 1, item_type: "trade", nutrition: None),
    (id: "copper_nuggets", name: "Copper Nuggets", price: 50, weight: 1, item_type: "trade", nutrition: None),
    (id: "rare_gem", name: "Rare Gem", price: 2000, weight: 1, item_type: "trade", nutrition: None),
    (id: "ancient_artifact", name: "Ancient Artifact", price: 5000, weight: 2, item_type: "trade", nutrition: None),
```

**Step 2: Verify data loads**

Run: `cargo check`
Expected: Compiles with no errors (the `load_game_data` system in `data.rs` uses `include_str!` so it will pick up the new items at compile time).

**Step 3: Commit**

```
git add assets/data/items.ron
git commit -m "feat: add exploration reward items to data"
```

---

### Task 2: Add ExplorationState Resource

**Files:**
- Modify: `src/game/resources.rs` — add `ExplorationState` struct
- Modify: `src/game/mod.rs` — register the resource

**Step 1: Add ExplorationState to resources.rs**

Add after the `BaseInventory` impl block (after line 243 in `src/game/resources.rs`):

```rust
#[derive(Resource, Clone, Debug, Default, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct ExplorationState {
    pub total_explorations: u32,
    pub generated_nodes: StdHashMap<String, u32>,
}

impl ExplorationState {
    pub const MAX_GENERATED_PER_TYPE: u32 = 3;

    pub fn can_generate(&self, resource_type: &str) -> bool {
        self.generated_count(resource_type) < Self::MAX_GENERATED_PER_TYPE
    }

    pub fn record_generation(&mut self, resource_type: &str) {
        *self.generated_nodes.entry(resource_type.to_string()).or_insert(0) += 1;
    }

    pub fn generated_count(&self, resource_type: &str) -> u32 {
        *self.generated_nodes.get(resource_type).unwrap_or(&0)
    }
}
```

**Step 2: Register in mod.rs**

In `src/game/mod.rs`, add after line 21 (`app.init_resource::<resources::BaseInventory>();`):

```rust
    app.init_resource::<resources::ExplorationState>();
```

And after line 30 (`app.register_type::<resources::BaseInventory>();`):

```rust
    app.register_type::<resources::ExplorationState>();
```

**Step 3: Verify compilation**

Run: `cargo check`
Expected: Compiles with no errors.

**Step 4: Commit**

```
git add src/game/resources.rs src/game/mod.rs
git commit -m "feat: add ExplorationState resource with generation tracking"
```

---

### Task 3: Add ExplorationState to Save/Load

**Files:**
- Modify: `src/game/save.rs` — add to SaveData, save_game, poll_load_game
- Modify: `src/game/scenarios.rs` — reset on new game

**Step 1: Add to SaveData struct**

In `src/game/save.rs`, add the following import at the top (add `ExplorationState` to the existing import from `resources`):

```rust
use crate::game::resources::{
    BaseInventory, BaseState, ExplorationState, GameState, NotificationLevel, NotificationState,
    PlayerState, SquadState,
};
```

Add to the `SaveData` struct after the `locations` field (after line 113):

```rust
    #[serde(default)]
    pub exploration_state: Option<ExplorationState>,
```

**Step 2: Save ExplorationState in save_game**

In the `save_game` function, add `exploration_state: Res<ExplorationState>` to the system parameters. Then in the `SaveData` construction, add:

```rust
            exploration_state: Some(exploration_state.clone()),
```

**Step 3: Load ExplorationState in poll_load_game**

In the `poll_load_game` function, add `mut exploration_state: ResMut<ExplorationState>` to the system parameters. Then after the line that restores `base_inventory` (line 346: `*base_inventory = save_data.base_inventory.unwrap_or_default();`), add:

```rust
                    *exploration_state = save_data.exploration_state.unwrap_or_default();
```

**Step 4: Reset in scenarios.rs**

In `src/game/scenarios.rs`, add `ExplorationState` to the import from resources:

```rust
use super::resources::{BaseInventory, BaseState, ExplorationState, GameState, PlayerState, SquadState};
```

Add `exploration_state: &mut ExplorationState` parameter to the `apply_scenario` function signature.

After the line `*base_inventory = BaseInventory::default();` (line 115), add:

```rust
    *exploration_state = ExplorationState::default();
```

Then update the call site in `src/screens/new_game.rs` (or wherever `apply_scenario` is called) to pass the new parameter.

**Step 5: Verify compilation**

Run: `cargo check`
Expected: Compiles with no errors.

**Step 6: Commit**

```
git add src/game/save.rs src/game/scenarios.rs src/game/mod.rs src/screens/new_game.rs
git commit -m "feat: persist ExplorationState in save/load, reset on new game"
```

---

### Task 4: Rewrite process_explore with Weighted Outcomes

**Files:**
- Modify: `src/game/action.rs` — rewrite `process_explore`, add outcome types and helper functions

**Step 1: Add exploration outcome types and helpers**

Add above the `process_explore` function in `src/game/action.rs`:

```rust
/// Possible outcomes when an exploration completes.
enum ExplorationOutcome {
    DiscoverLandmark,
    DiscoverResourceNode,
    FindItems,
    BonusXp,
    Nothing,
}

/// Name parts for generated resource nodes.
const NODE_SIZES: &[&str] = &["Small", "Modest", "Rich", "Dense"];
const NODE_TERRAINS_ORE: &[&str] = &["Vein", "Deposit", "Outcrop"];
const NODE_TERRAINS_STONE: &[&str] = &["Quarry", "Outcrop"];
const NODE_TERRAINS_LUMBER: &[&str] = &["Grove", "Thicket"];

/// Resources that can be generated as dynamic nodes.
const GENERABLE_RESOURCES: &[&str] = &["lumber", "stone", "copper_ore", "iron_ore"];

fn pick_weighted_outcome(weights: &[(ExplorationOutcome, u32)], roll: u32) -> &ExplorationOutcome {
    let mut cumulative = 0;
    for (outcome, weight) in weights {
        cumulative += weight;
        if roll < cumulative {
            return outcome;
        }
    }
    // Fallback (should not happen if roll < total_weight)
    &weights.last().unwrap().0
}

fn xp_to_level(xp: u32) -> u32 {
    (xp as f64).cbrt().floor() as u32
}

fn generate_node_name(resource_type: &str, game_time: u64) -> String {
    let size = NODE_SIZES[(game_time as usize) % NODE_SIZES.len()];
    let resource_display = match resource_type {
        "copper_ore" => "Copper",
        "iron_ore" => "Iron",
        "stone" => "Stone",
        "lumber" => "Lumber",
        _ => resource_type,
    };
    let terrains: &[&str] = match resource_type {
        "lumber" => NODE_TERRAINS_LUMBER,
        "stone" => NODE_TERRAINS_STONE,
        _ => NODE_TERRAINS_ORE,
    };
    let terrain = terrains[((game_time / 7) as usize) % terrains.len()];
    format!("{} {} {}", size, resource_display, terrain)
}

fn generate_node_distance(existing_count: u32, game_time: u64) -> u32 {
    // Distance range increases with existing node count
    let (min_dist, max_dist) = match existing_count {
        0 => (15, 25),
        1 => (30, 50),
        _ => (50, 80),
    };
    let range = max_dist - min_dist;
    min_dist + ((game_time as u32) % (range + 1))
}

fn generate_node_capacity(game_time: u64) -> u32 {
    // 100-300 capacity for generated nodes
    100 + ((game_time as u32) % 201)
}
```

**Step 2: Rewrite process_explore**

Replace the entire `process_explore` function with:

```rust
fn process_explore(
    mut commands: Commands,
    mut characters: Query<(&mut ActionState, &mut Skills, &CharacterInfo)>,
    mut locations: Query<&mut LocationInfo>,
    mut notifications: ResMut<NotificationState>,
    mut exploration_state: ResMut<ExplorationState>,
    mut base_inventory: ResMut<BaseInventory>,
    mut location_registry: ResMut<LocationRegistry>,
    game_data: Res<GameData>,
    sim: Res<SimulationState>,
) {
    for (mut state, mut skills, info) in &mut characters {
        if !matches!(&state.current_action, Some(Action::Explore)) {
            continue;
        }

        // Initialize progress based on scouting skill
        if state.progress.required == 0 {
            let scouting_level = xp_to_level(skills.scouting);
            let base_time = 100u32;
            let required = base_time.saturating_sub(scouting_level * 2).max(50);
            state.progress = ActionProgress::new(required);
        }

        if !state.progress.tick() {
            continue;
        }

        // Exploration complete — roll outcome
        let scouting_level = xp_to_level(skills.scouting);

        // Count undiscovered landmarks
        let undiscovered_count = locations.iter().filter(|l| !l.discovered).count() as u32;

        // Check which resource types can still generate nodes
        let can_generate_any = GENERABLE_RESOURCES
            .iter()
            .any(|r| exploration_state.can_generate(r));

        // Build weight table
        let mut weights: Vec<(ExplorationOutcome, u32)> = Vec::new();

        // Landmark discovery
        if undiscovered_count > 0 {
            weights.push((ExplorationOutcome::DiscoverLandmark, undiscovered_count * 15));
        }

        // Resource node generation
        if can_generate_any {
            weights.push((ExplorationOutcome::DiscoverResourceNode, scouting_level * 3 + 1));
        }

        // Find items (common)
        weights.push((ExplorationOutcome::FindItems, 10));

        // Bonus XP
        weights.push((ExplorationOutcome::BonusXp, 8));

        // Nothing (decreases with skill)
        let nothing_weight = 50 / scouting_level.max(1);
        if nothing_weight > 0 {
            weights.push((ExplorationOutcome::Nothing, nothing_weight));
        }

        let total_weight: u32 = weights.iter().map(|(_, w)| *w).sum();
        let roll = (sim.game_time as u32).wrapping_mul(31337) % total_weight;

        let outcome = pick_weighted_outcome(&weights, roll);
        exploration_state.total_explorations += 1;

        match outcome {
            ExplorationOutcome::DiscoverLandmark => {
                // Find the closest undiscovered landmark and reveal it
                let mut best: Option<(Entity, String, u32)> = None;
                for (entity, loc) in locations.iter().enumerate_entities() {
                    if !loc.discovered {
                        if best.is_none() || loc.distance < best.as_ref().unwrap().2 {
                            best = Some((entity, loc.name.clone(), loc.distance));
                        }
                    }
                }
                if let Some((entity, name, _)) = best {
                    if let Ok(mut loc) = locations.get_mut(entity) {
                        loc.discovered = true;
                    }
                    notifications.push(
                        format!("Discovered: {}!", name),
                        NotificationLevel::Success,
                    );
                }
            }

            ExplorationOutcome::DiscoverResourceNode => {
                // Pick a resource type that hasn't hit the cap
                let available: Vec<&&str> = GENERABLE_RESOURCES
                    .iter()
                    .filter(|r| exploration_state.can_generate(r))
                    .collect();

                if let Some(&&resource_type) = available
                    .get((sim.game_time as usize) % available.len())
                {
                    let existing = exploration_state.generated_count(resource_type);
                    let distance = generate_node_distance(existing, sim.game_time);
                    let capacity = generate_node_capacity(sim.game_time);
                    let name = generate_node_name(resource_type, sim.game_time);
                    let id = format!("gen_{}_{}", resource_type, sim.game_time);

                    let loc_type = match resource_type {
                        "lumber" => LocationType::Forest,
                        _ => LocationType::Mine,
                    };

                    let entity = commands
                        .spawn((
                            LocationInfo {
                                id: id.clone(),
                                name: name.clone(),
                                loc_type,
                                distance,
                                discovered: true,
                            },
                            LocationResources {
                                resource_type: resource_type.to_string(),
                                capacity,
                                yield_rate: 1,
                                current_amount: capacity,
                            },
                            LocationInventory::default(),
                        ))
                        .id();

                    location_registry.locations.insert(id, entity);
                    exploration_state.record_generation(resource_type);

                    notifications.push(
                        format!("Discovered new location: {}!", name),
                        NotificationLevel::Success,
                    );
                }
            }

            ExplorationOutcome::FindItems => {
                // Roll for item rewards
                let scouting = scouting_level;
                let item_roll = (sim.game_time as u32).wrapping_mul(7919) % 100;

                if scouting > 15 && item_roll < 15 {
                    // Rare item
                    let rare_items = ["rare_gem", "ancient_artifact"];
                    let item = rare_items[(sim.game_time as usize) % rare_items.len()];
                    base_inventory.add(item, 1);
                    let item_name = game_data
                        .get_item(item)
                        .map(|i| i.name.as_str())
                        .unwrap_or(item);
                    notifications.push(
                        format!("Found rare item: {}!", item_name),
                        NotificationLevel::Success,
                    );
                } else {
                    // Common item
                    let common_items = ["herbs", "scrap_wood", "copper_nuggets"];
                    let item = common_items[(sim.game_time as usize) % common_items.len()];
                    let quantity = 1 + (sim.game_time as u32 % 3);
                    base_inventory.add(item, quantity);
                    let item_name = game_data
                        .get_item(item)
                        .map(|i| i.name.as_str())
                        .unwrap_or(item);
                    notifications.push(
                        format!("Found {} x{}!", item_name, quantity),
                        NotificationLevel::Info,
                    );
                }
            }

            ExplorationOutcome::BonusXp => {
                let race = &info.race;
                let subrace = &info.subrace;
                let scout_mult = game_data.get_xp_multiplier(race, subrace, "scouting");
                let ath_mult = game_data.get_xp_multiplier(race, subrace, "athletics");
                let scout_bonus = (10.0 * scout_mult).max(1.0).ceil() as u32;
                let ath_bonus = (5.0 * ath_mult).max(1.0).ceil() as u32;
                skills.scouting = skills.scouting.saturating_add(scout_bonus);
                skills.athletics = skills.athletics.saturating_add(ath_bonus);
                notifications.push("Gained exploration experience!", NotificationLevel::Info);
            }

            ExplorationOutcome::Nothing => {
                notifications.push(
                    "Exploration yielded nothing of interest.",
                    NotificationLevel::Info,
                );
            }
        }

        state.current_action = None;
    }
}
```

**Step 3: Add missing imports**

At the top of `src/game/action.rs`, ensure these are imported:

```rust
use crate::game::location::LocationType;
use crate::game::resources::ExplorationState;
```

Also add `use crate::game::simulation::SimulationState;` if not already present.

The function signature for `process_explore` now takes `Commands`, `ExplorationState`, `BaseInventory`, `LocationRegistry` (as `ResMut`), `GameData`, and `SimulationState` — verify all are imported.

**Step 4: Verify compilation**

Run: `cargo check`
Expected: Compiles with no errors. There may be warnings about unused `enumerate_entities` — if so, use `.iter()` with manual entity tracking instead.

**Step 5: Commit**

```
git add src/game/action.rs
git commit -m "feat: rewrite exploration with weighted random outcomes"
```

---

### Task 5: Update Scenarios Call Site

**Files:**
- Modify: `src/screens/new_game.rs` (or wherever `apply_scenario` is called)

**Step 1: Find and update all call sites**

Search for `apply_scenario(` across the codebase. Each call site needs the new `&mut ExplorationState` parameter added. The system calling `apply_scenario` needs to add `exploration_state: ResMut<ExplorationState>` to its parameters.

Pass `&mut exploration_state` at the call site.

**Step 2: Verify compilation**

Run: `cargo check`
Expected: Compiles with no errors.

**Step 3: Commit**

```
git add src/screens/new_game.rs
git commit -m "feat: pass ExplorationState to scenario setup"
```

---

### Task 6: Verify Full Integration

**Step 1: Run tests**

Run: `cargo test`
Expected: All tests pass (0 failures).

**Step 2: Run clippy**

Run: `cargo clippy`
Expected: No errors (warnings are acceptable per CLAUDE.md conventions).

**Step 3: Format**

Run: `cargo fmt --all`

**Step 4: Final compilation check**

Run: `cargo check`
Expected: Clean compilation.

**Step 5: Commit any formatting changes**

```
git add -A
git commit -m "chore: format and lint cleanup"
```

(Only if there are changes to commit.)

**Step 6: Push to remote**

```
git push
```
