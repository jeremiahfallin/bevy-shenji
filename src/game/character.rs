use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};

use crate::game::action::ActionState;

#[derive(
    Component, Debug, Clone, Copy, PartialEq, Eq, Reflect, Serialize, Deserialize,
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
    pub hunger: u8,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            head: 100,
            stomach: 100,
            chest: 100,
            left_arm: 100,
            right_arm: 100,
            left_leg: 100,
            right_leg: 100,
            hunger: 100,
        }
    }
}

#[derive(
    Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct Skills {
    pub armor_smith: u32,
    pub assassination: u32,
    pub athletics: u32,
    pub blunt: u32,
    pub cooking: u32,
    pub crossbows: u32,
    pub crossbow_smith: u32,
    pub dexterity: u32,
    pub dodge: u32,
    pub engineer: u32,
    pub farming: u32,
    pub hackers: u32,
    pub heavy_weapons: u32,
    pub katanas: u32,
    pub labouring: u32,
    pub lockpicking: u32,
    pub martial_arts: u32,
    pub medic: u32,
    pub melee_attack: u32,
    pub melee_defense: u32,
    pub perception: u32,
    pub polearms: u32,
    pub robotics: u32,
    pub sabres: u32,
    pub science: u32,
    pub scouting: u32,
    pub stealth: u32,
    pub strength: u32,
    pub thievery: u32,
    pub toughness: u32,
    pub turrets: u32,
    pub weapon_smith: u32,
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
pub struct CharacterInfo {
    pub id: String,
    pub name: String,
    pub race: String,
    pub subrace: String,
    pub location: String,
}

#[derive(Component, Clone, Debug, Default, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct CharacterLocation {
    pub location_id: String,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Inventory {
    pub items: HashMap<String, u32>,
}

#[derive(
    Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct Squad(pub u16);

#[derive(Bundle)]
pub struct CharacterBundle {
    pub info: CharacterInfo,
    pub health: Health,
    pub skills: Skills,
    pub equipment: Equipment,
    pub inventory: Inventory,
    pub squad: Squad,
    pub action_state: ActionState,
    pub character_location: CharacterLocation,
}

impl Health {
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, u8)> {
        [
            ("Head", self.head),
            ("Stomach", self.stomach),
            ("Chest", self.chest),
            ("Left Arm", self.left_arm),
            ("Right Arm", self.right_arm),
            ("Left Leg", self.left_leg),
            ("Right Leg", self.right_leg),
        ]
        .into_iter()
    }
}

impl Skills {
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, u32)> {
        [
            ("Armor Smith", self.armor_smith),
            ("Assassination", self.assassination),
            ("Athletics", self.athletics),
            ("Blunt", self.blunt),
            ("Cooking", self.cooking),
            ("Crossbows", self.crossbows),
            ("Crossbow Smith", self.crossbow_smith),
            ("Dexterity", self.dexterity),
            ("Dodge", self.dodge),
            ("Engineer", self.engineer),
            ("Farming", self.farming),
            ("Hackers", self.hackers),
            ("Heavy Weapons", self.heavy_weapons),
            ("Katanas", self.katanas),
            ("Labouring", self.labouring),
            ("Lockpicking", self.lockpicking),
            ("Martial Arts", self.martial_arts),
            ("Medic", self.medic),
            ("Melee Attack", self.melee_attack),
            ("Melee Defense", self.melee_defense),
            ("Perception", self.perception),
            ("Polearms", self.polearms),
            ("Robotics", self.robotics),
            ("Sabres", self.sabres),
            ("Science", self.science),
            ("Scouting", self.scouting),
            ("Stealth", self.stealth),
            ("Strength", self.strength),
            ("Thievery", self.thievery),
            ("Toughness", self.toughness),
            ("Turrets", self.turrets),
            ("Weapon Smith", self.weapon_smith),
        ]
        .into_iter()
    }
}

impl CharacterBundle {
    pub fn new(id: String, name: String, race: String, subrace: String, location: String) -> Self {
        let location_id = location.clone();
        Self {
            info: CharacterInfo {
                id,
                name,
                race,
                subrace,
                location,
            },
            health: Health::default(),
            skills: Skills::default(),
            equipment: Equipment::default(),
            inventory: Inventory {
                items: HashMap::new(),
            },
            squad: Squad(0),
            action_state: ActionState::default(),
            character_location: CharacterLocation {
                location_id,
            },
        }
    }
}
