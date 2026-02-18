use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::game::location::LocationType;

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct ItemDef {
    pub id: String,
    pub name: String,
    pub price: u32,
    pub weight: u32,
    pub item_type: String,
    pub nutrition: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct SubraceDef {
    pub race: String,
    pub subrace: String,
    pub xp_multipliers: HashMap<String, f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect, PartialEq)]
pub enum ResearchEffect {
    UnlocksBuilding(String),
    UnlocksRecipe(String),
    SetsTechLevel(u32),
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct ResearchDef {
    pub id: String,
    pub name: String,
    pub research_type: String,
    pub tech_level: u32,
    pub cost: HashMap<String, u32>,
    pub time: u32,
    pub prerequisites: Vec<String>,
    pub effects: Vec<ResearchEffect>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct RecipeDef {
    pub id: String,
    pub name: String,
    pub inputs: HashMap<String, u32>,
    pub outputs: HashMap<String, u32>,
    pub workstation: String,
    pub time: u32,
    pub skill: String,
    pub required_research: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct BuildingDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub cost: HashMap<String, u32>,
    pub build_time: u32,
    pub tech_level: u32,
    pub required_research: Vec<String>,
    pub power_generation: i32,
    pub max_workers: u32,
    pub provides_workstation: Option<String>,
    pub provides_storage: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct LocationDef {
    pub id: String,
    pub name: String,
    pub loc_type: LocationType,
    pub distance: u32,
    pub resource_type: String,
    pub capacity: u32,
    pub yield_rate: u32,
    pub discovered: bool,
}

#[derive(Resource, Clone, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct GameData {
    pub items: HashMap<String, ItemDef>,
    pub races: Vec<SubraceDef>,
    pub research: HashMap<String, ResearchDef>,
    pub recipes: HashMap<String, RecipeDef>,
    pub buildings: HashMap<String, BuildingDef>,
    pub locations: Vec<LocationDef>,
}

impl GameData {
    pub fn get_item(&self, id: &str) -> Option<&ItemDef> {
        self.items.get(id)
    }

    pub fn get_xp_multiplier(&self, race: &str, subrace: &str, skill: &str) -> f32 {
        self.races
            .iter()
            .find(|r| r.race == race && r.subrace == subrace)
            .and_then(|r| r.xp_multipliers.get(skill))
            .copied()
            .unwrap_or(1.0)
    }

    pub fn get_research(&self, id: &str) -> Option<&ResearchDef> {
        self.research.get(id)
    }

    pub fn get_recipe(&self, id: &str) -> Option<&RecipeDef> {
        self.recipes.get(id)
    }

    pub fn get_building(&self, id: &str) -> Option<&BuildingDef> {
        self.buildings.get(id)
    }

    pub fn get_location(&self, id: &str) -> Option<&LocationDef> {
        self.locations.iter().find(|l| l.id == id)
    }
}

fn load_game_data(mut game_data: ResMut<GameData>) {
    // Load items
    let items_str = include_str!("../../assets/data/items.ron");
    let items: Vec<ItemDef> = ron::from_str(items_str).expect("Failed to parse items.ron");
    for item in items {
        game_data.items.insert(item.id.clone(), item);
    }

    // Load races
    let races_str = include_str!("../../assets/data/races.ron");
    let races: Vec<SubraceDef> = ron::from_str(races_str).expect("Failed to parse races.ron");
    game_data.races = races;

    // Load research
    let research_str = include_str!("../../assets/data/research.ron");
    let research_list: Vec<ResearchDef> =
        ron::from_str(research_str).expect("Failed to parse research.ron");
    for research in research_list {
        game_data.research.insert(research.id.clone(), research);
    }

    // Load recipes
    let recipes_str = include_str!("../../assets/data/recipes.ron");
    let recipe_list: Vec<RecipeDef> =
        ron::from_str(recipes_str).expect("Failed to parse recipes.ron");
    for recipe in recipe_list {
        game_data.recipes.insert(recipe.id.clone(), recipe);
    }

    // Load buildings
    let buildings_str = include_str!("../../assets/data/buildings.ron");
    let building_list: Vec<BuildingDef> =
        ron::from_str(buildings_str).expect("Failed to parse buildings.ron");
    for building in building_list {
        game_data.buildings.insert(building.id.clone(), building);
    }

    // Load locations
    let locations_str = include_str!("../../assets/data/locations.ron");
    let location_list: Vec<LocationDef> =
        ron::from_str(locations_str).expect("Failed to parse locations.ron");
    game_data.locations = location_list;
}

pub fn plugin(app: &mut App) {
    app.insert_resource(GameData::default())
        .register_type::<GameData>()
        .add_systems(Startup, load_game_data);
}
