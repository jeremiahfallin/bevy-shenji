use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, Reflect, PartialEq, Eq, Hash, Default)]
pub enum LocationType {
    #[default]
    Base,
    Mine,
    Forest,
    Ruins,
    City,
    Wilderness,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct LocationInfo {
    pub id: String,
    pub name: String,
    pub loc_type: LocationType,
    pub distance: u32,
    #[serde(default = "default_discovered")]
    pub discovered: bool,
}

fn default_discovered() -> bool {
    true
}

#[derive(Component, Clone, Debug, Default, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct LocationResources {
    pub resource_type: String,
    pub capacity: u32,
    pub yield_rate: u32,
    pub current_amount: u32,
}

#[derive(Component, Clone, Debug, Default, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct LocationInventory {
    pub items: HashMap<String, u32>,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct LocationRegistry {
    pub locations: HashMap<String, Entity>,
}

pub fn plugin(app: &mut App) {
    app.register_type::<LocationInfo>()
        .register_type::<LocationResources>()
        .register_type::<LocationInventory>()
        .insert_resource(LocationRegistry::default())
        .register_type::<LocationRegistry>();
}
