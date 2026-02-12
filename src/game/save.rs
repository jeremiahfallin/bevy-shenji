use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, Task, block_on, poll_once};
use bevy_immediate::utils::Mutable;
use serde::{Deserialize, Serialize};

// IMPORTS
use crate::game::character::{
    CharacterBundle, CharacterInfo, Equipment, Health, Inventory, Skills, Squad,
};
use crate::game::resources::{BaseState, GameState, PlayerState, SquadState};
use crate::screens::Screen; // Added import

pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SaveGameMessage>();
        app.add_message::<LoadGameMessage>();
        app.add_systems(
            Update,
            (save_game, start_load_game, poll_load_game, autosave_system),
        );
        app.init_resource::<AutosaveTimer>();
    }
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
struct LoadGameTask(Task<Result<SaveData, String>>);

// --- DTOs ---

#[derive(Serialize, Deserialize)]
pub struct SerializedCharacter {
    pub info: CharacterInfo,
    pub health: Health,
    pub skills: Skills,
    pub equipment: Equipment,
    pub inventory: Inventory,
    pub squad: Squad,
}

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub game_state: GameState,
    pub player_state: PlayerState,
    pub squad_state: SquadState,
    pub base_state: Option<BaseState>,
    pub characters: Vec<SerializedCharacter>,
}

// --- SYSTEMS ---

fn autosave_system(
    time: Res<Time>,
    state: Res<State<Screen>>, // Check state
    mut timer: ResMut<AutosaveTimer>,
    mut save_events: ResMut<Messages<SaveGameMessage>>, // Updated to Messages
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        // Only autosave if we are in Gameplay
        if *state.get() == Screen::Gameplay {
            info!("Autosaving...");
            save_events.send(SaveGameMessage("autosave".to_string()));
        }
    }
}

fn save_game(
    mut events: ResMut<Messages<SaveGameMessage>>, // Updated to Messages
    game_state: Res<GameState>,
    player_state: Res<PlayerState>,
    squad_state: Res<SquadState>,
    base_state: Option<Res<BaseState>>,
    character_query: Query<(
        &CharacterInfo,
        &Health,
        &Skills,
        &Equipment,
        &Inventory,
        &Squad,
    )>,
) {
    for message in events.drain() {
        let mut serialized_characters = Vec::new();
        for (info, health, skills, equip, inv, squad) in character_query.iter() {
            serialized_characters.push(SerializedCharacter {
                info: info.clone(),
                health: *health,
                skills: *skills,
                equipment: equip.clone(),
                inventory: inv.clone(),
                squad: *squad,
            });
        }

        let save_data = SaveData {
            game_state: game_state.clone(),
            player_state: player_state.clone(),
            squad_state: squad_state.clone(),
            base_state: base_state.as_ref().map(|b| (**b).clone()),
            characters: serialized_characters,
        };

        let filename = format!("assets/saves/{}.json", message.0);
        let thread_pool = IoTaskPool::get();

        thread_pool
            .spawn(async move {
                #[cfg(not(target_family = "wasm"))]
                {
                    if let Some(parent) = std::path::Path::new(&filename).parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    match serde_json::to_string_pretty(&save_data) {
                        Ok(json) => {
                            if let Err(e) = std::fs::write(&filename, json) {
                                error!("Async Save Failed: {}", e);
                            } else {
                                info!("Async Save Complete: {}", filename);
                            }
                        }
                        Err(e) => error!("Serialization Failed: {}", e),
                    }
                }
            })
            .detach();
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
    old_characters: Query<Entity, With<CharacterInfo>>,
) {
    for (task_entity, mut task) in &mut tasks {
        if let Some(result) = block_on(poll_once(&mut task.0)) {
            match result {
                Ok(save_data) => {
                    info!("Loading game data...");
                    for entity in old_characters.iter() {
                        commands.entity(entity).despawn();
                    }
                    *game_state = save_data.game_state;
                    *player_state = save_data.player_state;
                    *squad_state = save_data.squad_state;
                    squad_state.characters.clear();
                    if let Some(base) = save_data.base_state {
                        commands.insert_resource(base);
                    }
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
                            })
                            .id();
                        squad_state.characters.insert(id, entity);
                    }
                    info!("Game loaded successfully!");
                }
                Err(e) => error!("Failed to load game: {}", e),
            }
            commands.entity(task_entity).despawn();
        }
    }
}
