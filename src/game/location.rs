use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum LocationType {
    Village,
    City,
    Ruins,
    Wilderness,
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub loc_type: LocationType,
}

impl Location {
    pub fn new(id: String, name: String, loc_type: LocationType) -> Self {
        Self { id, name, loc_type }
    }
}
