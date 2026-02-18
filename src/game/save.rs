use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, Task, block_on, poll_once};
use serde::{Deserialize, Serialize};

use crate::game::action::ActionState;
use crate::game::character::{
    CharacterBundle, CharacterInfo, Equipment, Health, Inventory, Skills, Squad,
};
use crate::game::research::ResearchState;
use crate::game::resources::{
    BaseState, GameState, NotificationLevel, NotificationState, PlayerState, SquadState,
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
}

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub game_state: GameState,
    pub player_state: PlayerState,
    pub squad_state: SquadState,
    pub base_state: Option<BaseState>,
    pub research_state: Option<ResearchState>,
    pub simulation_state: Option<SimulationState>,
    pub characters: Vec<SerializedCharacter>,
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
    character_query: Query<(
        &CharacterInfo,
        &Health,
        &Skills,
        &Equipment,
        &Inventory,
        &Squad,
        &ActionState,
    )>,
) {
    for message in events.drain() {
        let mut serialized_characters = Vec::new();
        for (info, health, skills, equip, inv, squad, action_state) in character_query.iter() {
            serialized_characters.push(SerializedCharacter {
                info: info.clone(),
                health: *health,
                skills: *skills,
                equipment: equip.clone(),
                inventory: inv.clone(),
                squad: *squad,
                action_state: action_state.clone(),
            });
        }

        let save_data = SaveData {
            game_state: game_state.clone(),
            player_state: player_state.clone(),
            squad_state: squad_state.clone(),
            base_state: base_state.as_ref().map(|b| (**b).clone()),
            research_state: research_state.as_ref().map(|r| (**r).clone()),
            simulation_state: Some(sim_state.clone()),
            characters: serialized_characters,
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
                    notifications.push(
                        format!("Save failed: {}", e),
                        NotificationLevel::Error,
                    );
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
    mut notifications: ResMut<NotificationState>,
    old_characters: Query<Entity, With<CharacterInfo>>,
) {
    for (task_entity, mut task) in &mut tasks {
        if let Some(result) = block_on(poll_once(&mut task.0)) {
            match result {
                Ok(save_data) => {
                    info!("Loading game data...");

                    // 1. Despawn all existing character entities to avoid orphans.
                    //    This uses deferred commands, but we rebuild the entity map
                    //    below so the old Entity IDs are never referenced again.
                    for entity in old_characters.iter() {
                        commands.entity(entity).despawn();
                    }

                    // 2. Restore top-level resource state from save data.
                    *game_state = save_data.game_state;
                    *player_state = save_data.player_state;

                    // 3. Restore squad state. The saved `characters` HashMap contains
                    //    Entity IDs from the old session which are now invalid, so we
                    //    must clear it and rebuild after spawning new entities.
                    *squad_state = save_data.squad_state;
                    squad_state.characters.clear();

                    // 4. Restore base, research, and simulation state, defaulting
                    //    if absent (for backwards compatibility with older saves).
                    *base_state = save_data.base_state.unwrap_or_default();
                    *research_state = save_data.research_state.unwrap_or_default();
                    *sim_state = save_data.simulation_state.unwrap_or_default();

                    // 5. Respawn character entities from save data and rebuild the
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
                            })
                            .id();
                        squad_state.characters.insert(id, entity);
                    }

                    info!("Game loaded successfully!");
                    notifications.push("Game loaded", NotificationLevel::Success);
                }
                Err(e) => {
                    error!("Failed to load game: {}", e);
                    notifications.push(
                        format!("Load failed: {}", e),
                        NotificationLevel::Error,
                    );
                }
            }
            commands.entity(task_entity).despawn();
        }
    }
}
