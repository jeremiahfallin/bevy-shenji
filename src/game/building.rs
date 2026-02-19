use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Debug, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct Building {
    pub def_id: String,
    pub name: String,
    pub progress: u32,
    pub required: u32,
    pub complete: bool,
}

pub fn plugin(app: &mut App) {
    app.register_type::<Building>();
}
