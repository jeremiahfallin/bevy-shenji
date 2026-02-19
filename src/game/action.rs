use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::game::building::Building;
use crate::game::character::{CharacterInfo, CharacterLocation, Inventory, Skills};
use crate::game::data::{GameData, ResearchEffect};
use crate::game::location::{
    LocationInfo, LocationInventory, LocationRegistry, LocationResources, LocationType,
};
use crate::game::research::ResearchState;
use crate::game::resources::{
    BaseInventory, BaseState, ExplorationState, NotificationLevel, NotificationState,
};
use crate::game::simulation::SimulationState;

/// A single action a character can perform
#[derive(Clone, Debug, Serialize, Deserialize, Reflect, PartialEq)]
pub enum Action {
    Idle,
    Gather {
        location: String,
        resource: String,
    },
    Collect {
        location: String,
        item: String,
    },
    Deposit {
        item: String,
    },
    Travel {
        destination: String,
    },
    Research {
        tech_id: String,
    },
    Craft {
        recipe_id: String,
        workstation: String,
    },
    Build {
        building: String,
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
        Self {
            current: 0,
            required,
        }
    }

    pub fn tick(&mut self) -> bool {
        self.current += 1;
        self.current >= self.required
    }

    pub fn fraction(&self) -> f32 {
        if self.required == 0 {
            return 1.0;
        }
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
    pub fn queue_action(&mut self, action: Action) {
        self.action_queue.push_back(action);
    }

    pub fn clear(&mut self) {
        self.current_action = None;
        self.progress = ActionProgress::default();
        self.action_queue.clear();
    }

    pub fn clear_jobs(&mut self) {
        self.job_queue.clear();
        self.current_job_index = 0;
    }
}

/// Create a gather job that travels to a location, gathers a resource,
/// collects it, travels back to base, and deposits it.
pub fn make_gather_job(location_id: &str, resource: &str) -> Job {
    Job {
        name: format!("Gather {}", resource),
        actions: vec![
            Action::Travel {
                destination: location_id.to_string(),
            },
            Action::Gather {
                location: location_id.to_string(),
                resource: resource.to_string(),
            },
            Action::Collect {
                location: location_id.to_string(),
                item: resource.to_string(),
            },
            Action::Travel {
                destination: "base".to_string(),
            },
            Action::Deposit {
                item: resource.to_string(),
            },
        ],
    }
}

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
            let job_index = state.current_job_index % state.job_queue.len();
            let actions: Vec<Action> = state.job_queue[job_index].actions.clone();
            if !actions.is_empty() {
                for action in actions {
                    state.action_queue.push_back(action);
                }
                state.current_job_index += 1;
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

fn process_travel(
    mut characters: Query<(&mut ActionState, &mut CharacterLocation)>,
    location_registry: Res<LocationRegistry>,
    locations: Query<&LocationInfo>,
) {
    for (mut state, mut location) in &mut characters {
        let destination = match &state.current_action {
            Some(Action::Travel { destination }) => destination.clone(),
            _ => continue,
        };
        if state.progress.required == 0 {
            let travel_time = location_registry
                .locations
                .get(&destination)
                .and_then(|&e| locations.get(e).ok())
                .map(|info| info.distance)
                .unwrap_or(30);
            state.progress = ActionProgress::new(travel_time);
        }
        if state.progress.tick() {
            location.location_id = destination;
            state.current_action = None;
        }
    }
}

fn process_gather(
    mut characters: Query<(&mut ActionState, &CharacterLocation)>,
    mut locations: Query<(
        &LocationInfo,
        &mut LocationResources,
        &mut LocationInventory,
    )>,
    location_registry: Res<LocationRegistry>,
) {
    for (mut state, char_location) in &mut characters {
        let (location_id, resource) = match &state.current_action {
            Some(Action::Gather { location, resource }) => (location.clone(), resource.clone()),
            _ => continue,
        };

        // Verify character is at the correct location
        if char_location.location_id != location_id {
            state.current_action = None;
            continue;
        }

        // Find the location entity
        let Some(&location_entity) = location_registry.locations.get(&location_id) else {
            state.current_action = None;
            continue;
        };

        let Ok((_loc_info, mut loc_resources, mut loc_inventory)) =
            locations.get_mut(location_entity)
        else {
            state.current_action = None;
            continue;
        };

        // Check that the resource type matches and there is something to gather
        if loc_resources.resource_type != resource || loc_resources.current_amount == 0 {
            state.current_action = None;
            continue;
        }

        // Initialize progress on first tick
        if state.progress.required == 0 {
            state.progress = ActionProgress::new(100);
        }

        // Tick progress
        if state.progress.tick() {
            // Extract yield_rate amount from current_amount
            let yield_amount = loc_resources.yield_rate.min(loc_resources.current_amount);
            loc_resources.current_amount -= yield_amount;

            // Add to location inventory
            *loc_inventory.items.entry(resource.clone()).or_insert(0) += yield_amount;

            // Clear action
            state.current_action = None;
        }
    }
}

fn process_collect(
    mut characters: Query<(&mut ActionState, &CharacterLocation, &mut Inventory)>,
    mut locations: Query<&mut LocationInventory>,
    location_registry: Res<LocationRegistry>,
) {
    for (mut state, char_location, mut inventory) in &mut characters {
        let (location_id, item) = match &state.current_action {
            Some(Action::Collect { location, item }) => (location.clone(), item.clone()),
            _ => continue,
        };

        // Verify character is at the correct location
        if char_location.location_id != location_id {
            state.current_action = None;
            continue;
        }

        // Find the location entity
        let Some(&location_entity) = location_registry.locations.get(&location_id) else {
            state.current_action = None;
            continue;
        };

        let Ok(mut loc_inventory) = locations.get_mut(location_entity) else {
            state.current_action = None;
            continue;
        };

        // Move items from location inventory to character inventory
        if let Some(amount) = loc_inventory.items.remove(&item) {
            *inventory.items.entry(item.clone()).or_insert(0) += amount;
        }

        // Clear action (instant)
        state.current_action = None;
    }
}

fn process_deposit(
    mut characters: Query<(&mut ActionState, &CharacterLocation, &mut Inventory)>,
    mut base_inventory: ResMut<BaseInventory>,
) {
    for (mut state, char_location, mut inventory) in &mut characters {
        let item = match &state.current_action {
            Some(Action::Deposit { item }) => item.clone(),
            _ => continue,
        };

        // Verify character is at base
        if char_location.location_id != "base" {
            state.current_action = None;
            continue;
        }

        // Move items from character inventory to base inventory
        if let Some(amount) = inventory.items.remove(&item) {
            base_inventory.add(&item, amount);
        }

        // Clear action (instant)
        state.current_action = None;
    }
}

fn process_research(
    mut characters: Query<(&mut ActionState, &CharacterLocation, &Skills)>,
    mut research_state: ResMut<ResearchState>,
    game_data: Res<GameData>,
    mut base_inventory: ResMut<BaseInventory>,
    mut base_state: ResMut<BaseState>,
) {
    for (mut state, char_location, skills) in &mut characters {
        let tech_id = match &state.current_action {
            Some(Action::Research { tech_id }) => tech_id.clone(),
            _ => continue,
        };

        // Character must be at base to research
        if char_location.location_id != "base" {
            state.current_action = None;
            continue;
        }

        // Look up the research definition
        let Some(def) = game_data.get_research(&tech_id) else {
            state.current_action = None;
            continue;
        };

        // If the global current_research doesn't match, try to start new research
        if research_state.current_research.as_deref() != Some(&tech_id) {
            // Check prerequisites and that it isn't already unlocked
            if !research_state.can_research(&tech_id, &game_data) {
                state.current_action = None;
                continue;
            }

            // Check and deduct costs from BaseInventory
            let can_afford = def
                .cost
                .iter()
                .all(|(item_id, &amount)| base_inventory.count(item_id) >= amount);
            if !can_afford {
                state.current_action = None;
                continue;
            }
            for (item_id, &amount) in &def.cost {
                base_inventory.remove(item_id, amount);
            }

            // Start this research
            research_state.current_research = Some(tech_id.clone());
            research_state.research_progress = 0;
        }

        // Science skill provides a speed bonus: base 1 tick + 1 per 20 science skill
        let speed_bonus = 1 + (skills.science / 20);
        research_state.research_progress += speed_bonus;

        // Check if research is complete
        if research_state.research_progress >= def.time {
            research_state.unlocked.insert(tech_id.clone());

            // Apply effects
            for effect in &def.effects {
                match effect {
                    ResearchEffect::SetsTechLevel(level) => {
                        base_state.value.tech_level = *level;
                    }
                    ResearchEffect::UnlocksBuilding(_) | ResearchEffect::UnlocksRecipe(_) => {
                        // These are checked by other systems via ResearchState::unlocked
                    }
                }
            }

            // Clear current research
            research_state.current_research = None;
            research_state.research_progress = 0;
            state.current_action = None;
        }
    }
}

fn process_craft(
    mut characters: Query<(&mut ActionState, &CharacterLocation)>,
    game_data: Res<GameData>,
    mut base_inv: ResMut<BaseInventory>,
) {
    for (mut state, char_loc) in &mut characters {
        let recipe_id = match &state.current_action {
            Some(Action::Craft { recipe_id, .. }) => recipe_id.clone(),
            _ => continue,
        };

        // Character must be at base
        if char_loc.location_id != "base" {
            state.current_action = None;
            continue;
        }

        // Look up recipe definition
        let Some(recipe) = game_data.get_recipe(&recipe_id) else {
            state.current_action = None;
            continue;
        };

        // First tick: check and consume all inputs from BaseInventory
        if state.progress.required == 0 {
            let can_afford = recipe
                .inputs
                .iter()
                .all(|(item, &amount)| base_inv.count(item) >= amount);
            if !can_afford {
                state.current_action = None;
                continue;
            }
            for (item, &amount) in &recipe.inputs {
                base_inv.remove(item, amount);
            }
            state.progress = ActionProgress::new(recipe.time);
        }

        // Tick progress; on completion produce outputs into BaseInventory
        if state.progress.tick() {
            for (item, &amount) in &recipe.outputs {
                base_inv.add(item, amount);
            }
            state.current_action = None;
        }
    }
}

fn process_build(
    mut characters: Query<(&mut ActionState, &CharacterLocation)>,
    mut buildings: Query<&mut Building>,
) {
    for (mut state, char_loc) in &mut characters {
        let building_id = match &state.current_action {
            Some(Action::Build { building }) => building.clone(),
            _ => continue,
        };

        // Character must be at base
        if char_loc.location_id != "base" {
            state.current_action = None;
            continue;
        }

        // Find an incomplete Building entity with matching def_id
        let mut found = false;
        let mut completed = false;
        for mut building in &mut buildings {
            if building.def_id == building_id && !building.complete {
                building.progress += 1;
                if building.progress >= building.required {
                    building.complete = true;
                    completed = true;
                }
                found = true;
                break;
            }
        }

        if !found || completed {
            state.current_action = None;
        }
    }
}

/// Possible outcomes when an exploration completes.
enum ExplorationOutcome {
    DiscoverLandmark,
    DiscoverResourceNode,
    FindItems,
    BonusXp,
    Nothing,
}

const NODE_SIZES: &[&str] = &["Small", "Modest", "Rich", "Dense"];
const NODE_TERRAINS_ORE: &[&str] = &["Vein", "Deposit", "Outcrop"];
const NODE_TERRAINS_STONE: &[&str] = &["Quarry", "Outcrop"];
const NODE_TERRAINS_LUMBER: &[&str] = &["Grove", "Thicket"];
const GENERABLE_RESOURCES: &[&str] = &["lumber", "stone", "copper_ore", "iron_ore"];

fn pick_weighted_outcome(weights: &[(ExplorationOutcome, u32)], roll: u32) -> &ExplorationOutcome {
    let mut cumulative = 0;
    for (outcome, weight) in weights {
        cumulative += weight;
        if roll < cumulative {
            return outcome;
        }
    }
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
    let (min_dist, max_dist) = match existing_count {
        0 => (15, 25),
        1 => (30, 50),
        _ => (50, 80),
    };
    let range = max_dist - min_dist;
    min_dist + ((game_time as u32) % (range + 1))
}

fn generate_node_capacity(game_time: u64) -> u32 {
    100 + ((game_time as u32) % 201)
}

fn process_explore(
    mut commands: Commands,
    mut characters: Query<(&mut ActionState, &mut Skills, &CharacterInfo)>,
    mut locations: Query<(Entity, &mut LocationInfo)>,
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

        if state.progress.required == 0 {
            let scouting_level = xp_to_level(skills.scouting);
            let base_time = 100u32;
            let required = base_time.saturating_sub(scouting_level * 2).max(50);
            state.progress = ActionProgress::new(required);
        }

        if !state.progress.tick() {
            continue;
        }

        let scouting_level = xp_to_level(skills.scouting);
        let undiscovered_count = locations.iter().filter(|(_, l)| !l.discovered).count() as u32;
        let can_generate_any = GENERABLE_RESOURCES
            .iter()
            .any(|r| exploration_state.can_generate(r));

        let mut weights: Vec<(ExplorationOutcome, u32)> = Vec::new();

        if undiscovered_count > 0 {
            weights.push((ExplorationOutcome::DiscoverLandmark, undiscovered_count * 15));
        }
        if can_generate_any {
            weights.push((
                ExplorationOutcome::DiscoverResourceNode,
                scouting_level * 3 + 1,
            ));
        }
        weights.push((ExplorationOutcome::FindItems, 10));
        weights.push((ExplorationOutcome::BonusXp, 8));
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
                // Find the closest undiscovered location
                let mut best: Option<(Entity, String, u32)> = None;
                for (entity, loc) in locations.iter() {
                    if !loc.discovered {
                        if best.is_none() || loc.distance < best.as_ref().unwrap().2 {
                            best = Some((entity, loc.name.clone(), loc.distance));
                        }
                    }
                }
                if let Some((entity, name, _)) = best {
                    if let Ok((_, mut loc)) = locations.get_mut(entity) {
                        loc.discovered = true;
                    }
                    notifications.push(
                        format!("Discovered: {}!", name),
                        NotificationLevel::Success,
                    );
                }
            }

            ExplorationOutcome::DiscoverResourceNode => {
                let available: Vec<&&str> = GENERABLE_RESOURCES
                    .iter()
                    .filter(|r| exploration_state.can_generate(r))
                    .collect();
                if let Some(&&resource_type) =
                    available.get((sim.game_time as usize) % available.len())
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
                let item_roll = (sim.game_time as u32).wrapping_mul(7919) % 100;
                if scouting_level > 15 && item_roll < 15 {
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

fn apply_skill_xp(
    mut characters: Query<(&ActionState, &mut Skills, &CharacterInfo)>,
    game_data: Res<GameData>,
) {
    for (state, mut skills, info) in &mut characters {
        let Some(ref action) = state.current_action else {
            continue;
        };
        let skill_name = match action {
            Action::Gather { .. } => "labouring",
            Action::Research { .. } => "science",
            Action::Explore => "scouting",
            Action::Travel { .. } => "athletics",
            Action::Craft { .. } => "labouring",
            Action::Build { .. } => "engineer",
            _ => continue,
        };
        let multiplier = game_data.get_xp_multiplier(&info.race, &info.subrace, skill_name);
        let xp_gain = (1.0_f32 * multiplier).max(0.5).ceil() as u32;
        // Apply to matching skill field
        match skill_name {
            "labouring" => skills.labouring = skills.labouring.saturating_add(xp_gain),
            "science" => skills.science = skills.science.saturating_add(xp_gain),
            "scouting" => skills.scouting = skills.scouting.saturating_add(xp_gain),
            "athletics" => skills.athletics = skills.athletics.saturating_add(xp_gain),
            "engineer" => skills.engineer = skills.engineer.saturating_add(xp_gain),
            _ => {}
        }
        // Explore also gives athletics XP
        if matches!(action, Action::Explore) {
            let ath_mult = game_data.get_xp_multiplier(&info.race, &info.subrace, "athletics");
            skills.athletics = skills
                .athletics
                .saturating_add((1.0_f32 * ath_mult).max(0.5).ceil() as u32);
        }
    }
}

pub fn plugin(app: &mut App) {
    app.register_type::<ActionState>();

    app.add_systems(
        FixedUpdate,
        (
            dequeue_actions,
            process_travel,
            process_gather,
            process_collect,
            process_deposit,
            process_research,
            process_craft,
            process_build,
            process_explore,
            apply_skill_xp,
        )
            .chain()
            .in_set(crate::game::simulation::SimulationSystems::ProcessActions),
    );
}
