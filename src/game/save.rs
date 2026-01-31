use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, Task, block_on, poll_once};
use serde::{Deserialize, Serialize};

use super::resources::{BaseState, GameState, PlayerState, SquadState};

pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SaveGameEvent>();
        app.add_message::<LoadGameEvent>();
        app.add_systems(
            Update,
            (save_game, start_load_game, poll_load_game, autosave_system),
        );
        // Register the Autosave Timer
        app.init_resource::<AutosaveTimer>();
    }
}

#[derive(Resource)]
pub struct AutosaveTimer(pub Timer);

impl Default for AutosaveTimer {
    fn default() -> Self {
        // Autosave every 60 seconds
        Self(Timer::from_seconds(60.0, TimerMode::Repeating))
    }
}

/// Trigger this event to save the game to a specific filename (e.g. "save1")
#[derive(Message)]
pub struct SaveGameEvent(pub String);

/// Trigger this event to load the game from a specific filename
#[derive(Message)]
pub struct LoadGameEvent(pub String);

/// A component that holds the running background task for loading.
#[derive(Component)]
struct LoadGameTask(Task<Result<SaveData, String>>);

/// The top-level struct that gets written to the JSON file
#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub game_state: GameState,
    pub player_state: PlayerState,
    pub squad_state: SquadState,
    pub base_state: Option<BaseState>,
}

fn autosave_system(
    time: Res<Time>,
    mut timer: ResMut<AutosaveTimer>,
    mut save_writer: MessageWriter<SaveGameEvent>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        info!("Autosaving...");
        save_writer.write(SaveGameEvent("autosave".to_string()));
    }
}

fn save_game(
    mut events: MessageReader<SaveGameEvent>,
    game_state: Res<GameState>,
    player_state: Res<PlayerState>,
    squad_state: Res<SquadState>,
    base_state: Option<Res<BaseState>>,
) {
    for event in events.read() {
        let save_data = SaveData {
            game_state: game_state.clone(),
            player_state: player_state.clone(),
            squad_state: squad_state.clone(),
            base_state: base_state.as_ref().map(|b| (**b).clone()),
        };

        let filename = format!("assets/saves/{}.json", event.0);

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

fn start_load_game(mut commands: Commands, mut events: MessageReader<LoadGameEvent>) {
    for event in events.read() {
        let filename = format!("assets/saves/{}.json", event.0);

        let thread_pool = IoTaskPool::get();

        // Spawn the task to read/parse the file
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

        // Spawn an entity to hold this task so we can poll it next frame
        commands.spawn(LoadGameTask(task));
    }
}

fn poll_load_game(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut LoadGameTask)>,
    mut game_state: ResMut<GameState>,
    mut player_state: ResMut<PlayerState>,
    mut squad_state: ResMut<SquadState>,
) {
    for (entity, mut task) in &mut tasks {
        // Poll the task once. If it's ready, we get Some(Result).
        if let Some(result) = block_on(poll_once(&mut task.0)) {
            match result {
                Ok(save_data) => {
                    *game_state = save_data.game_state;
                    *player_state = save_data.player_state;
                    *squad_state = save_data.squad_state;

                    if let Some(base) = save_data.base_state {
                        commands.insert_resource(base);
                    }
                    info!("Game loaded successfully!");
                }
                Err(e) => error!("Failed to load game: {}", e),
            }

            // Cleanup the task entity
            commands.entity(entity).despawn();
        }
    }
}
