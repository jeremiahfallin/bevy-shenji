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
    pub hunger: u8,
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
pub struct CharacterInfo {
    pub id: String,
    pub name: String,
    pub race: String,
    pub subrace: String,
    pub location: String,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Inventory {
    pub items: HashMap<String, String>,
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
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, u8)> {
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
        }
    }
}
