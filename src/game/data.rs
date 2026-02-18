use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Resource, Clone, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct GameData {
    pub items: HashMap<String, ItemDef>,
    pub races: Vec<SubraceDef>,
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
}

pub fn plugin(app: &mut App) {
    app.insert_resource(GameData::default())
        .register_type::<GameData>()
        .add_systems(Startup, load_game_data);
}
