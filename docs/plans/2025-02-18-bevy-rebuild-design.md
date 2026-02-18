# Shenji Bevy Rebuild — Design Document

## Context

Shenji is a Kenshi-inspired management/strategy sim. A web prototype (Next.js/React) exists with full game systems: ~1,300 items, ~200 research nodes, 32-skill character system, action/job queues, crafting, building, exploration. The Bevy version has a solid UI shell (screen flow, save/load, layout, inspector) but no simulation. This document describes the design for porting the core game into Bevy 0.17, keeping the same game pillars while restructuring for idiomatic ECS.

## Guiding Principles

- **Port core, redesign details** — same game design pillars, improved/simplified where it makes sense in Bevy's ECS model
- **ECS-native actions** — actions as components on character entities, processed by Bevy systems with system sets for ordering
- **Data-driven** — items, research, recipes, races defined in RON asset files, not hardcoded in Rust
- **Incremental milestones** — each phase delivers a testable, visible improvement

---

## 1. Architecture Overview

Five pillars, each a Bevy plugin module:

| Pillar | Module | Purpose |
|---|---|---|
| Simulation | `game/simulation/` | Game tick, time, day/night cycle |
| Characters | `game/character/` | Actions, skills, health, equipment |
| Economy | `game/economy/` | Resources, crafting, building, trading |
| World | `game/world/` | Locations, travel, exploration |
| Progression | `game/progression/` | Research tree, tech levels, unlocks |

Data files live in `assets/data/*.ron` and are loaded as Bevy assets.

---

## 2. Simulation Core

### Game Tick

Uses Bevy's `Time<Fixed>` with a configurable tick rate. Default: 1 game-second per real-second.

**System ordering per tick:**

```
SimulationSystems (ordered sets):
  1. AdvanceTime        — increment game_time, game_days
  2. ProcessActions     — run the action pipeline for all characters
  3. UpdateEconomy      — power grid, passive income, hunger drain
  4. CheckEvents        — trigger events based on state
  5. UpdateUI           — sync UI state with game state
```

### Speed Controls

Pause, 1x, 2x, 5x — multiplies the fixed timestep rate.

### Day Cycle

Every N ticks = 1 game day. Days drive hunger drain, passive events, autosave triggers.

---

## 3. Character System

### Entity Components

Each character is an entity with:

- `CharacterInfo` — id, name, race, subrace
- `Health` — 7 body parts (head, chest, stomach, left/right arm, left/right leg) + hunger + oil (robots)
- `Skills` — 32 skills stored as XP values; race multiplier applied on gain
- `Equipment` — 8 slots (head, shirt, chest, backpack, legs, feet, leftHand, rightHand) referencing item IDs
- `CharacterInventory` — HashMap<ItemId, u32>
- `ActionState` — current action + queue + job queue
- `CharacterLocation` — which location entity they're at
- `Squad` — squad membership

### Action System (ECS-Native)

```rust
// On each character entity
ActionState {
    current_action: Option<Action>,
    action_queue: VecDeque<Action>,
    job_queue: Vec<Job>,          // repeating tasks that loop
}

enum Action {
    Idle,
    Gather { location: Entity, resource: ItemId },
    Collect { location: Entity, item: ItemId },
    Deposit { item: ItemId },
    Craft { recipe: RecipeId, workstation: Entity },
    Build { building: Entity },
    Travel { destination: Entity },
    Explore,
    Research { tech_id: TechId },
}
```

**Processing systems (ordered):**

1. `dequeue_actions` — if no current action, pop from action_queue; if empty, loop from job_queue
2. `validate_actions` — check preconditions (correct location, inventory space, resources available)
3. `process_gather` / `process_craft` / `process_travel` / etc. — one system per action type
4. `apply_skill_xp` — grant XP based on action performed, scaled by race multiplier
5. `check_action_complete` — mark done, advance to next

### Skill Growth

- XP earned per tick while performing relevant actions
- Race multipliers loaded from `races.ron`
- Level formula: `min(1 + cbrt(xp * 4 / 5), 100)`

### Races & Subraces

5 races, each with subraces that define XP multipliers for all 32 skills:

- **Human**: Greenlander (science/cooking/farming bonus), Scorchlander (dexterity/stealth/athletics)
- **Shek**: Melee combat focus (strength, melee attack bonuses)
- **Hive**: Prince (perception/stealth/science), Soldier (melee), Drone (engineering/robotics)
- **Skeleton**: P4 (engineering/science), base Skeleton (robotics), Soldierbot (perception/ranged)

---

## 4. Economy & Crafting

### Base Resources

Components on the base entity:

- `BaseInventory` — HashMap<ItemId, u32> for all stored items
- `BasePower` — generation, consumption, capacity, current
- `BaseWealth` — zeni (currency), land, tatami

### Resource Flow

```
Gather (at location) → Collect (pickup) → Travel (to base) → Deposit (into base inventory)
                                                                    ↓
                                                              Craft / Build
```

### Crafting

- Recipes in `recipes.ron`: inputs (item × qty), output (item × qty), required workstation, time, relevant skill
- `process_craft` system: validates inputs in base inventory, decrements, tracks progress, produces output
- Multiple workers can craft simultaneously at a workstation (each on their own recipe)

### Building

- Building definitions in `buildings.ron`: cost (items), build time, worker slots, what it provides (workstation, storage, power, etc.)
- `process_build` system: workers contribute progress each tick, building entity status changes to Complete when done
- Completed buildings unlock workstations, storage capacity, power generation

### Power Grid

- Buildings have power_generation and power_consumption values
- `update_power` system sums generation/consumption across all buildings
- Some buildings/workstations require power to operate

---

## 5. World & Locations

### Location Entities

Each location is an entity with:

- `LocationInfo` — id, name, type (Base, Mine, Forest, Ruins, City, Wilderness)
- `LocationResources` — resource type, capacity, yield rate, current amount
- `LocationInventory` — items sitting at the location (gathered but not yet collected)
- `DistanceFromBase` — multiplier for travel time

### Travel

- `process_travel` system tracks progress toward destination
- Travel time = base_time × distance_multiplier / athletics_modifier
- Character is unavailable (no other actions) during travel

### Exploration

- Characters assigned Explore action discover new locations
- Scouting skill affects discovery speed and quality
- Discovered locations are spawned as new entities and become available for gathering/travel

---

## 6. Research & Progression

### Tech Tree

Loaded from `research.ron`: ~200 nodes with id, name, description, type, tier (1-6), cost (items), research_time, prerequisites, and effects.

### Research Types

basic, core, crafting, defense, electric, farming, industry, smithing, training

### Processing

- Characters with Research action contribute progress per tick, scaled by science skill
- Research costs consumed from base inventory when research starts
- When complete, tech unlocked and effects applied

### Tech Effects

Each research node can have one or more effects:

- `UnlocksBuilding(BuildingId)`
- `UnlocksRecipe(RecipeId)`
- `UnlocksLocation(LocationType)`
- `ModifiesRate(Resource, Multiplier)`
- `UnlocksTechLevel(u8)` — gates access to higher-tier research

---

## 7. Data Files (RON)

```
assets/data/
├── races.ron          # Race/subrace definitions + XP multipliers per skill
├── items.ron          # ~1,300 item definitions (name, weight, value, category)
├── research.ron       # ~200 tech tree nodes with prerequisites and effects
├── recipes.ron        # Crafting recipes (inputs, outputs, workstation, time, skill)
├── buildings.ron      # Building definitions (cost, time, provides)
├── locations.ron      # Starting + discoverable locations with resources
└── scenarios.ron      # Starting conditions (characters, items, resources)
```

---

## 8. UI Enhancements

Building on the existing Bevy UI shell:

- **Dashboard**: Resource overview, active workers summary, current research, recent notifications
- **Inspector tab switching**: Fix existing bug — render the tab bar buttons
- **Action assignment UI**: Select character → assign action from context menu
- **Job queue panel**: View, reorder, remove repeating jobs per character
- **Speed controls**: Pause / 1x / 2x / 5x buttons in bottom bar
- **Location view**: List of known locations with resource info, distance, workers present
- **Build menu**: Available buildings filtered by tech level, showing costs and requirements

---

## 9. Phased Milestones

| Phase | Goal | Key Deliverables |
|---|---|---|
| **1 — Tick** | Time passes, days count | Simulation core, speed controls, day cycle, hunger drain |
| **2 — Actions** | Characters do things | ActionState component, dequeue/validate systems, gather/collect/deposit/travel processors |
| **3 — Economy** | Resources flow | BaseInventory, resource gathering loop, action assignment UI |
| **4 — Data** | Port game content | RON asset loader, items, races, scenarios from RON files |
| **5 — Research** | Progression works | Tech tree from RON, research action processor, unlock effects |
| **6 — Crafting** | Production chains | Recipes from RON, workstation entities, craft action processor |
| **7 — Building** | Base construction | Building definitions, build action, power grid system |
| **8 — World** | Exploration & travel | Location discovery, exploration action, scouting |
| **9 — Combat** | Fighting works | Damage calculation, combat skills, enemy entities |
| **10 — Polish** | Full game loop | Events system, game balance, UI polish, settings |

---

## 10. What's NOT In Scope

- Multiplayer / networking
- 3D rendering (staying 2D)
- Procedural world generation (locations are authored in data files)
- Mod support (future consideration)
- Mobile / console ports
