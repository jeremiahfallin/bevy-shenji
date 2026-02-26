use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap as StdHashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect, Serialize, Deserialize)]
pub enum GameView {
    #[default]
    Dashboard,
    Research,
    Squads,
    Characters,
    Locations,
    Buildings,
}

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct UiState {
    pub active_view: GameView,
}

#[derive(Resource, Default, Debug, Reflect, Clone, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct GameState {
    pub current_level: u32,
}

impl GameState {
    pub fn reset(&mut self) {
        self.current_level = 0;
    }
}

#[derive(Resource, Default, Debug, Reflect, Clone, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct PlayerState {
    pub gold: u32,
    pub experience: u32,
    pub level: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect, Serialize, Deserialize)]
pub enum SquadStatus {
    #[default]
    Idle,
    Active,
    Traveling,
}
#[derive(Debug, Reflect, Clone, Serialize, Deserialize)]
pub struct Squad {
    pub name: String,
    pub members: Vec<String>,
    #[serde(default)]
    pub status: SquadStatus,
}

#[derive(Resource, Debug, Reflect, Clone, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct SquadState {
    /// All characters in the game, keyed by their ID
    pub characters: HashMap<String, Entity>,
    /// Squads keyed by their ID
    pub squads: HashMap<u16, Squad>,
    /// Order of squads to display
    pub squad_order: Vec<u16>,
    /// Currently selected character ID (for UI)
    pub selected_character: Option<String>,
    /// Next available character ID counter
    pub next_id: u32,
    /// Next available squad ID counter
    pub next_squad_id: u16,
}

impl Default for SquadState {
    fn default() -> Self {
        Self {
            characters: HashMap::new(),
            squads: HashMap::new(),
            squad_order: Vec::new(),
            selected_character: None,
            next_id: 1,
            next_squad_id: 1,
        }
    }
}

impl SquadState {
    pub fn add_character(&mut self, id: String, entity: Entity) {
        self.characters.insert(id, entity);
    }

    pub fn select_character(&mut self, character_id: String) {
        self.selected_character = Some(character_id);
    }

    pub fn get_or_create_squad(&mut self, squad_id: u16) {
        if !self.squads.contains_key(&squad_id) {
            let name = format!("Squad {}", squad_id);
            self.squads.insert(
                squad_id,
                Squad {
                    name,
                    members: Vec::new(),
                    status: SquadStatus::default(),
                },
            );
            self.squad_order.push(squad_id);
        }
    }

    pub fn add_member_to_squad(&mut self, character_id: &str, squad_id: u16) {
        self.get_or_create_squad(squad_id);

        if let Some(squad) = self.squads.get_mut(&squad_id) {
            if !squad.members.iter().any(|m| m == character_id) {
                squad.members.push(character_id.to_string());
            }
        }
    }

    pub fn reorder_squads(&mut self, new_order: Vec<u16>) {
        self.squad_order = new_order;
    }
}

#[derive(Resource, Debug, Reflect, Clone, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct BaseState {
    pub value: BaseValues,
    pub power: BasePower,
}

impl Default for BaseState {
    fn default() -> Self {
        Self {
            value: BaseValues::default(),
            power: BasePower::default(),
        }
    }
}

#[derive(Resource, Debug, Reflect, Clone, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct BaseValues {
    pub zeni: u32,
    pub land: u32,
    pub tatami: u32,
    pub research_level: u32,
    pub tech_level: u32,
}

impl Default for BaseValues {
    fn default() -> Self {
        Self {
            zeni: 1000,
            land: 10,
            tatami: 10,
            research_level: 0,
            tech_level: 0,
        }
    }
}

#[derive(Resource, Debug, Reflect, Clone, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct BasePower {
    pub generation: u32,
    pub consumption: u32,
    pub capacity: u32,
    pub current: u32,
}

impl Default for BasePower {
    fn default() -> Self {
        Self {
            generation: 1,
            consumption: 0,
            capacity: 100,
            current: 1,
        }
    }
}

/// Severity level for UI notifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum NotificationLevel {
    Info,
    Success,
    Error,
}

/// A single notification to display to the player.
#[derive(Debug, Clone, Reflect)]
pub struct Notification {
    pub message: String,
    pub level: NotificationLevel,
    /// Remaining seconds before this notification is dismissed.
    pub ttl: f32,
}

/// Resource that holds a queue of active notifications.
#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct NotificationState {
    pub notifications: VecDeque<Notification>,
}

impl NotificationState {
    /// Push a new notification with a default 4-second display time.
    pub fn push(&mut self, message: impl Into<String>, level: NotificationLevel) {
        self.notifications.push_back(Notification {
            message: message.into(),
            level,
            ttl: 4.0,
        });
    }

    /// Tick all notification timers and remove expired ones.
    pub fn tick(&mut self, dt: f32) {
        for n in self.notifications.iter_mut() {
            n.ttl -= dt;
        }
        self.notifications.retain(|n| n.ttl > 0.0);
    }
}

/// A persistent log entry for the event log sidebar.
#[derive(Debug, Clone, Reflect)]
pub struct EventLogEntry {
    pub message: String,
    pub level: NotificationLevel,
    pub game_tick: u64,
}

/// Persistent event log that keeps a history of game events.
#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct EventLog {
    pub entries: VecDeque<EventLogEntry>,
}

impl EventLog {
    pub const MAX_ENTRIES: usize = 100;

    pub fn push(&mut self, message: impl Into<String>, level: NotificationLevel, game_tick: u64) {
        self.entries.push_front(EventLogEntry {
            message: message.into(),
            level,
            game_tick,
        });
        if self.entries.len() > Self::MAX_ENTRIES {
            self.entries.pop_back();
        }
    }
}
#[derive(Resource, Clone, Debug, Default, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct BaseInventory {
    pub items: StdHashMap<String, u32>,
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
        *self
            .generated_nodes
            .entry(resource_type.to_string())
            .or_insert(0) += 1;
    }

    pub fn generated_count(&self, resource_type: &str) -> u32 {
        *self.generated_nodes.get(resource_type).unwrap_or(&0)
    }
}
