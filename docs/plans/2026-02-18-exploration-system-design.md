# Exploration System Design

## Overview

Replace the current deterministic sequential exploration with a weighted random outcome system. Exploration discovers both hand-crafted landmark locations and dynamically generated resource nodes, with item rewards and skill progression as secondary outcomes.

## Core Loop

1. Player assigns `Action::Explore` to a character
2. Scouting skill determines exploration duration (distance-weighted)
3. On completion, roll against a weighted outcome table
4. Apply the outcome (discover location, grant items, bonus XP, or nothing)
5. Send notification describing the result

## Outcome Table

Five outcome types, each with a weight formula that depends on game state and character skill:

| Outcome | Weight | Effect |
|---------|--------|--------|
| Discover Landmark | `undiscovered_count * 15` (0 when all found) | Reveals a pre-defined location from `locations.ron`, prioritizing closer ones |
| Discover Resource Node | `scouting_level * 3` (0 when at cap for resource type) | Spawns a generated location entity with scaled-down capacity/yield |
| Find Items | `10` (common), `+5` if scouting > 15 (rare) | Adds items directly to BaseInventory |
| Bonus XP | `8` | Burst of extra scouting + athletics XP |
| Nothing | `50 / max(1, scouting_level)` | Notification only, becomes rarer with skill |

Weight of 0 removes that outcome from the pool entirely.

## Generated Resource Nodes

Dynamic locations created by exploration, supplementing hand-crafted landmarks.

**Resource types:** lumber, stone, copper_ore, iron_ore (matching existing items.ron entries).

**Naming template:** `"{Size} {Resource} {Terrain}"` drawn from word pools:
- Size: Small, Modest, Rich, Dense
- Resource: Copper, Iron, Stone, Lumber
- Terrain: Vein, Deposit, Quarry, Grove, Outcrop

Examples: "Small Copper Vein", "Rich Iron Deposit", "Dense Lumber Grove"

**Scaling vs. landmarks:**
- Capacity: 100-300 (vs. landmark 500-1000)
- Yield rate: 1 (same as landmarks)
- LocationType: Mine for ore/stone, Forest for lumber

**Constraints:**
- Max 3 generated nodes per resource type
- Distance increases with existing node count:
  - 1st node: 15-25
  - 2nd node: 30-50
  - 3rd node: 50-80

## Difficulty Model: Distance-Weighted

No global difficulty counter. Exploration time is tied to what's being discovered.

**Base exploration time:**
- Landmarks: `landmark.distance * 3` ticks
- Generated nodes: `rolled_distance * 3` ticks
- Non-location outcomes (items, XP, nothing): `100` ticks base

**Scouting skill modifier:** `base_time - (scouting * 2)`, minimum 50 ticks.

Since closer landmarks are prioritized for discovery and generated nodes get further away as more exist, difficulty naturally escalates without an artificial counter.

## Item Rewards

When the "Find Items" outcome is rolled:

**Common items (scouting any level):**
- herbs, scrap_wood, copper_nuggets (1-3 quantity)

**Rare items (scouting > 15):**
- rare_gem, ancient_artifact (1 quantity)
- Added to the item pool with weight 5 (vs. 10 for common)

Items are added directly to `BaseInventory`. Items must exist in `items.ron`.

## Bonus XP Reward

When "Bonus XP" is rolled:
- Scouting: +10 XP (modified by race multiplier)
- Athletics: +5 XP (modified by race multiplier)

Applied on top of the normal per-tick XP gain from the Explore action.

## ExplorationState Resource

New resource tracking exploration meta-state:

```rust
#[derive(Resource, Clone, Debug, Default, Serialize, Deserialize, Reflect)]
pub struct ExplorationState {
    pub total_explorations: u32,
    pub generated_nodes: HashMap<String, u32>, // resource_type -> count
}
```

Methods:
- `can_generate(resource_type: &str) -> bool` — checks against max cap (3)
- `record_generation(resource_type: &str)` — increments counter
- `generated_count(resource_type: &str) -> u32`

Persisted in save data as `Option<ExplorationState>` with `#[serde(default)]`.

## RNG

Use `rand` crate (already a Bevy dependency). Seed from `SimulationState.game_time` at the moment of outcome resolution for reproducibility if replay support is added later.

Weighted selection: compute cumulative weights, roll `0..total_weight`, find the outcome whose cumulative range contains the roll.

## Files Affected

- `src/game/action.rs` — rewrite `process_explore`, add outcome resolution logic
- `src/game/resources.rs` — add `ExplorationState` resource
- `src/game/save.rs` — add `ExplorationState` to `SaveData`
- `src/game/scenarios.rs` — reset `ExplorationState` on new game
- `src/game/mod.rs` — register `ExplorationState`
- `assets/data/items.ron` — add exploration reward items if missing (herbs, rare_gem, etc.)
