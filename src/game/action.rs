use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::game::building::Building;
use crate::game::character::{CharacterInfo, CharacterLocation, Inventory, Skills};
use crate::game::data::{GameData, ResearchEffect};
use crate::game::location::{LocationInfo, LocationInventory, LocationRegistry, LocationResources};
use crate::game::research::ResearchState;
use crate::game::resources::{BaseInventory, BaseState, NotificationLevel, NotificationState};

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

fn process_travel(mut characters: Query<(&mut ActionState, &mut CharacterLocation)>) {
    for (mut state, mut location) in &mut characters {
        let destination = match &state.current_action {
            Some(Action::Travel { destination }) => destination.clone(),
            _ => continue,
        };
        if state.progress.required == 0 {
            state.progress = ActionProgress::new(30); // default 30 ticks
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
            *loc_inventory
                .items
                .entry(resource.clone())
                .or_insert(0) += yield_amount;

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
        for mut building in &mut buildings {
            if building.def_id == building_id && !building.complete {
                // Contribute 1 progress per tick
                building.progress += 1;
                if building.progress >= building.required {
                    building.complete = true;
                }
                found = true;
                break;
            }
        }

        if !found {
            // No matching incomplete building found; cancel action
            state.current_action = None;
            continue;
        }

        // Check if the building we just worked on is now complete
        let mut done = false;
        for building in &buildings {
            if building.def_id == building_id && building.complete {
                done = true;
                break;
            }
        }

        if done {
            state.current_action = None;
        }
    }
}

fn process_explore(
    mut characters: Query<(&mut ActionState, &Skills)>,
    mut locations: Query<&mut LocationInfo>,
    mut notifications: ResMut<NotificationState>,
) {
    for (mut state, skills) in &mut characters {
        if !matches!(&state.current_action, Some(Action::Explore)) {
            continue;
        }

        // Initialize progress based on scouting skill
        if state.progress.required == 0 {
            let required = (200u32).saturating_sub(skills.scouting * 5).max(50);
            state.progress = ActionProgress::new(required);
        }

        if state.progress.tick() {
            // Find the first undiscovered location and mark it as discovered
            let mut discovered_name = None;
            for mut loc_info in &mut locations {
                if !loc_info.discovered {
                    loc_info.discovered = true;
                    discovered_name = Some(loc_info.name.clone());
                    break;
                }
            }

            if let Some(name) = discovered_name {
                notifications.push(
                    format!("Discovered: {}", name),
                    NotificationLevel::Success,
                );
            } else {
                notifications.push(
                    "Exploration complete - no new locations found",
                    NotificationLevel::Info,
                );
            }

            state.current_action = None;
        }
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
        let xp_gain = (1.0 * multiplier) as u32;
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
            skills.athletics = skills.athletics.saturating_add((1.0 * ath_mult) as u32);
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
