use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, Task, block_on, poll_once};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::game::action::ActionState;
use crate::game::building::Building;
use crate::game::character::{
    CharacterBundle, CharacterInfo, CharacterLocation, Equipment, Health, Inventory, Skills, Squad,
};
use crate::game::location::{
    LocationInfo, LocationInventory, LocationRegistry, LocationResources, LocationType,
};
use crate::game::research::ResearchState;
use crate::game::resources::{
    BaseInventory, BaseState, ExplorationState, GameState, NotificationLevel, NotificationState,
    PlayerState, SquadState,
};
use crate::game::simulation::SimulationState;
use crate::screens::Screen;

pub fn plugin(app: &mut App) {
    app.add_message::<SaveGameMessage>();
    app.add_message::<LoadGameMessage>();
    app.add_systems(
        Update,
        (
            save_game,
            poll_save_game,
            start_load_game,
            poll_load_game,
            autosave_system,
        ),
    );
    app.init_resource::<AutosaveTimer>();
}

#[derive(Resource)]
pub struct AutosaveTimer(pub Timer);

impl Default for AutosaveTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(60.0, TimerMode::Repeating))
    }
}

#[derive(Message)]
pub struct SaveGameMessage(pub String);

#[derive(Message)]
pub struct LoadGameMessage(pub String);

#[derive(Component)]
struct SaveGameTask(Task<Result<String, String>>);

#[derive(Component)]
struct LoadGameTask(Task<Result<SaveData, String>>);

#[derive(Serialize, Deserialize)]
pub struct SerializedCharacter {
    pub info: CharacterInfo,
    pub health: Health,
    pub skills: Skills,
    pub equipment: Equipment,
    pub inventory: Inventory,
    pub squad: Squad,
    #[serde(default)]
    pub action_state: ActionState,
    #[serde(default)]
    pub character_location: CharacterLocation,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SerializedBuilding {
    pub def_id: String,
    pub name: String,
    pub progress: u32,
    pub required: u32,
    pub complete: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SerializedLocation {
    pub id: String,
    pub name: String,
    pub loc_type: LocationType,
    pub distance: u32,
    pub discovered: bool,
    pub resource_type: String,
    pub capacity: u32,
    pub yield_rate: u32,
    pub current_amount: u32,
    pub inventory: HashMap<String, u32>,
}

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub game_state: GameState,
    pub player_state: PlayerState,
    pub squad_state: SquadState,
    #[serde(default)]
    pub base_state: Option<BaseState>,
    #[serde(default)]
    pub research_state: Option<ResearchState>,
    #[serde(default)]
    pub simulation_state: Option<SimulationState>,
    #[serde(default)]
    pub base_inventory: Option<BaseInventory>,
    pub characters: Vec<SerializedCharacter>,
    #[serde(default)]
    pub buildings: Option<Vec<SerializedBuilding>>,
    #[serde(default)]
    pub locations: Option<Vec<SerializedLocation>>,
    #[serde(default)]
    pub exploration_state: Option<ExplorationState>,
}

fn autosave_system(
    time: Res<Time>,
    state: Res<State<Screen>>, // Check state
    mut timer: ResMut<AutosaveTimer>,
    mut save_events: MessageWriter<SaveGameMessage>, // Updated to Messages
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        // Only autosave if we are in Gameplay
        if *state.get() == Screen::Gameplay {
            info!("Autosaving...");
            save_events.write(SaveGameMessage("autosave".to_string()));
        }
    }
}

fn save_game(
    mut commands: Commands,
    mut events: ResMut<Messages<SaveGameMessage>>,
    game_state: Res<GameState>,
    player_state: Res<PlayerState>,
    squad_state: Res<SquadState>,
    base_state: Option<Res<BaseState>>,
    research_state: Option<Res<ResearchState>>,
    sim_state: Res<SimulationState>,
    base_inventory: Res<BaseInventory>,
    exploration_state: Res<ExplorationState>,
    character_query: Query<(
        &CharacterInfo,
        &Health,
        &Skills,
        &Equipment,
        &Inventory,
        &Squad,
        &ActionState,
        &CharacterLocation,
    )>,
    building_query: Query<&Building>,
    location_query: Query<(&LocationInfo, &LocationResources, &LocationInventory)>,
) {
    for message in events.drain() {
        let mut serialized_characters = Vec::new();
        for (info, health, skills, equip, inv, squad, action_state, char_location) in
            character_query.iter()
        {
            serialized_characters.push(SerializedCharacter {
                info: info.clone(),
                health: *health,
                skills: *skills,
                equipment: equip.clone(),
                inventory: inv.clone(),
                squad: *squad,
                action_state: action_state.clone(),
                character_location: char_location.clone(),
            });
        }

        // Serialize all building entities
        let serialized_buildings: Vec<SerializedBuilding> = building_query
            .iter()
            .map(|b| SerializedBuilding {
                def_id: b.def_id.clone(),
                name: b.name.clone(),
                progress: b.progress,
                required: b.required,
                complete: b.complete,
            })
            .collect();

        // Serialize all location entities
        let serialized_locations: Vec<SerializedLocation> = location_query
            .iter()
            .map(|(info, resources, inventory)| SerializedLocation {
                id: info.id.clone(),
                name: info.name.clone(),
                loc_type: info.loc_type.clone(),
                distance: info.distance,
                discovered: info.discovered,
                resource_type: resources.resource_type.clone(),
                capacity: resources.capacity,
                yield_rate: resources.yield_rate,
                current_amount: resources.current_amount,
                inventory: inventory.items.clone(),
            })
            .collect();

        let save_data = SaveData {
            game_state: game_state.clone(),
            player_state: player_state.clone(),
            squad_state: squad_state.clone(),
            base_state: base_state.as_ref().map(|b| (**b).clone()),
            research_state: research_state.as_ref().map(|r| (**r).clone()),
            simulation_state: Some(sim_state.clone()),
            base_inventory: Some(base_inventory.clone()),
            characters: serialized_characters,
            buildings: Some(serialized_buildings),
            locations: Some(serialized_locations),
            exploration_state: Some(exploration_state.clone()),
        };

        let filename = format!("assets/saves/{}.json", message.0);
        let thread_pool = IoTaskPool::get();

        let task = thread_pool.spawn(async move {
            #[cfg(not(target_family = "wasm"))]
            {
                if let Some(parent) = std::path::Path::new(&filename).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                match serde_json::to_string_pretty(&save_data) {
                    Ok(json) => match std::fs::write(&filename, &json) {
                        Ok(()) => Ok(filename),
                        Err(e) => Err(format!("Write failed: {}", e)),
                    },
                    Err(e) => Err(format!("Serialization failed: {}", e)),
                }
            }
            #[cfg(target_family = "wasm")]
            {
                Err("WASM saving not implemented".to_string())
            }
        });

        commands.spawn(SaveGameTask(task));
    }
}

fn poll_save_game(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut SaveGameTask)>,
    mut notifications: ResMut<NotificationState>,
) {
    for (task_entity, mut task) in &mut tasks {
        if let Some(result) = block_on(poll_once(&mut task.0)) {
            match result {
                Ok(filename) => {
                    info!("Save complete: {}", filename);
                    notifications.push("Game saved", NotificationLevel::Success);
                }
                Err(e) => {
                    error!("Save failed: {}", e);
                    notifications.push(format!("Save failed: {}", e), NotificationLevel::Error);
                }
            }
            commands.entity(task_entity).despawn();
        }
    }
}

fn start_load_game(mut commands: Commands, mut events: ResMut<Messages<LoadGameMessage>>) {
    // WORKAROUND: Use events.drain()
    for message in events.drain() {
        let filename = format!("assets/saves/{}.json", message.0);
        let thread_pool = IoTaskPool::get();

        let task = thread_pool.spawn(async move {
            #[cfg(not(target_family = "wasm"))]
            {
                match std::fs::read_to_string(&filename) {
                    Ok(json) => match serde_json::from_str::<SaveData>(&json) {
                        Ok(data) => Ok(data),
                        Err(e) => Err(format!("Parse error: {}", e)),
                    },
                    Err(e) => Err(format!("File error: {}", e)),
                }
            }
            #[cfg(target_family = "wasm")]
            {
                Err("WASM loading not implemented".to_string())
            }
        });

        commands.spawn(LoadGameTask(task));
    }
}

fn poll_load_game(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut LoadGameTask)>,
    mut game_state: ResMut<GameState>,
    mut player_state: ResMut<PlayerState>,
    mut squad_state: ResMut<SquadState>,
    mut base_state: ResMut<BaseState>,
    mut research_state: ResMut<ResearchState>,
    mut sim_state: ResMut<SimulationState>,
    mut base_inventory: ResMut<BaseInventory>,
    mut exploration_state: ResMut<ExplorationState>,
    mut notifications: ResMut<NotificationState>,
    mut location_registry: ResMut<LocationRegistry>,
    old_characters: Query<Entity, With<CharacterInfo>>,
    old_buildings: Query<Entity, With<Building>>,
    old_locations: Query<Entity, With<LocationInfo>>,
) {
    for (task_entity, mut task) in &mut tasks {
        if let Some(result) = block_on(poll_once(&mut task.0)) {
            match result {
                Ok(save_data) => {
                    info!("Loading game data...");

                    // 1. Despawn all existing character entities to avoid orphans.
                    for entity in old_characters.iter() {
                        commands.entity(entity).despawn();
                    }

                    // 2. Despawn all existing building entities.
                    for entity in old_buildings.iter() {
                        commands.entity(entity).despawn();
                    }

                    // 3. Despawn all existing location entities and clear the registry.
                    for entity in old_locations.iter() {
                        commands.entity(entity).despawn();
                    }
                    location_registry.locations.clear();

                    // 4. Restore top-level resource state from save data.
                    *game_state = save_data.game_state;
                    *player_state = save_data.player_state;

                    // 5. Restore squad state. The saved `characters` HashMap contains
                    //    Entity IDs from the old session which are now invalid, so we
                    //    must clear it and rebuild after spawning new entities.
                    *squad_state = save_data.squad_state;
                    squad_state.characters.clear();

                    // 6. Restore base, research, and simulation state, defaulting
                    //    if absent (for backwards compatibility with older saves).
                    *base_state = save_data.base_state.unwrap_or_default();
                    *research_state = save_data.research_state.unwrap_or_default();
                    *sim_state = save_data.simulation_state.unwrap_or_default();
                    *base_inventory = save_data.base_inventory.unwrap_or_default();
                    *exploration_state = save_data.exploration_state.unwrap_or_default();

                    // 7. Respawn character entities from save data and rebuild the
                    //    entity map so SquadState.characters has valid Entity IDs.
                    for char_data in save_data.characters {
                        let id = char_data.info.id.clone();
                        let entity = commands
                            .spawn(CharacterBundle {
                                info: char_data.info,
                                health: char_data.health,
                                skills: char_data.skills,
                                equipment: char_data.equipment,
                                inventory: char_data.inventory,
                                squad: char_data.squad,
                                action_state: char_data.action_state,
                                character_location: char_data.character_location,
                            })
                            .id();
                        squad_state.characters.insert(id, entity);
                    }

                    // 8. Respawn building entities from save data.
                    if let Some(buildings) = save_data.buildings {
                        for b in buildings {
                            commands.spawn(Building {
                                def_id: b.def_id,
                                name: b.name,
                                progress: b.progress,
                                required: b.required,
                                complete: b.complete,
                            });
                        }
                    }

                    // 9. Respawn location entities from save data and rebuild
                    //    the LocationRegistry.
                    if let Some(locations) = save_data.locations {
                        for loc in locations {
                            let entity = commands
                                .spawn((
                                    LocationInfo {
                                        id: loc.id.clone(),
                                        name: loc.name,
                                        loc_type: loc.loc_type,
                                        distance: loc.distance,
                                        discovered: loc.discovered,
                                    },
                                    LocationResources {
                                        resource_type: loc.resource_type,
                                        capacity: loc.capacity,
                                        yield_rate: loc.yield_rate,
                                        current_amount: loc.current_amount,
                                    },
                                    LocationInventory {
                                        items: loc.inventory,
                                    },
                                ))
                                .id();
                            location_registry.locations.insert(loc.id, entity);
                        }
                    }

                    info!("Game loaded successfully!");
                    notifications.push("Game loaded", NotificationLevel::Success);
                }
                Err(e) => {
                    error!("Failed to load game: {}", e);
                    notifications.push(format!("Load failed: {}", e), NotificationLevel::Error);
                }
            }
            commands.entity(task_entity).despawn();
        }
    }
}
