use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(
    Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct Health {
    pub head: u8,
    pub stomach: u8,
    pub chest: u8,
    pub left_arm: u8,
    pub right_arm: u8,
    pub left_leg: u8,
    pub right_leg: u8,
}

#[derive(
    Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct Skills {
    pub armor_smith: u8,
    pub assassination: u8,
    pub athletics: u8,
    pub blunt: u8,
    pub cooking: u8,
    pub crossbows: u8,
    pub crossbow_smith: u8,
    pub dexterity: u8,
    pub dodge: u8,
    pub engineer: u8,
    pub farming: u8,
    pub hackers: u8,
    pub heavy_weapons: u8,
    pub katanas: u8,
    pub labouring: u8,
    pub lockpicking: u8,
    pub martial_arts: u8,
    pub medic: u8,
    pub melee_attack: u8,
    pub melee_defense: u8,
    pub perception: u8,
    pub polearms: u8,
    pub robotics: u8,
    pub sabres: u8,
    pub science: u8,
    pub scouting: u8,
    pub stealth: u8,
    pub strength: u8,
    pub thievery: u8,
    pub toughness: u8,
    pub turrets: u8,
    pub weapon_smith: u8,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Equipment {
    pub head: Option<String>,
    pub chest: Option<String>,
    pub legs: Option<String>,
    pub feet: Option<String>,
    pub hands: Option<String>,
    pub main_hand: Option<String>,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub race: String,
    pub subrace: String,
    pub location: String,
    pub health: Health,
    pub skills: Skills,
    pub equipment: Equipment,
    pub squad: u16,
    pub inventory: HashMap<String, String>,
}

impl Character {
    pub fn new(id: String, name: String, race: String, subrace: String, location: String) -> Self {
        Self {
            id,
            name,
            race,
            subrace,
            location,
            health: Health::default(),
            skills: Skills::default(),
            equipment: Equipment::default(),
            squad: 0,
            inventory: HashMap::new(),
        }
    }

    pub fn add_to_squad(&mut self, squad: u16) {
        self.squad = squad;
    }

    pub fn remove_from_squad(&mut self) {
        self.squad = 0;
    }
}
