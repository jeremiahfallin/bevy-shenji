use super::character::Character;
use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Resource, Default, Debug, Reflect, Clone, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct GameState {
    pub current_level: u32,
    pub game_time: f32,
    pub is_paused: bool,
    pub game_days: u32,
}

impl GameState {
    pub fn reset(&mut self) {
        self.current_level = 0;
        self.game_time = 0.0;
        self.is_paused = false;
        self.game_days = 0;
    }
}

#[derive(Resource, Default, Debug, Reflect, Clone, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct PlayerState {
    pub gold: u32,
    pub experience: u32,
    pub level: u32,
}

#[derive(Debug, Reflect, Clone, Serialize, Deserialize)]
pub struct Squad {
    pub name: String,
    pub members: Vec<String>,
}

#[derive(Resource, Debug, Reflect, Clone, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct SquadState {
    /// All characters in the game, keyed by their ID
    pub characters: HashMap<String, Character>,
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
    pub fn add_character(&mut self, mut character: Character) -> String {
        let id = format!("char_{}", self.next_id);
        self.next_id += 1;
        character.id = id.clone();
        self.characters.insert(id.clone(), character);
        id
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
                },
            );
            self.squad_order.push(squad_id);
        }
    }

    pub fn assign_to_squad(&mut self, character_id: &str, squad_id: u16) {
        if let Some(character) = self.characters.get_mut(character_id) {
            character.add_to_squad(squad_id);

            self.get_or_create_squad(squad_id);

            if let Some(squad) = self.squads.get_mut(&squad_id) {
                if !squad.members.iter().any(|m| m == character_id) {
                    squad.members.push(character_id.to_string());
                }
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

#[derive(Resource, Debug, Reflect, Clone, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct BaseValues {
    pub zeni: u32,
    pub land: u32,
    pub tatami: u32,
    pub research_level: u32,
    pub tech_level: u32,
}

#[derive(Resource, Debug, Reflect, Clone, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct BasePower {
    pub generation: u32,
    pub consumption: u32,
    pub capacity: u32,
    pub current: u32,
}
