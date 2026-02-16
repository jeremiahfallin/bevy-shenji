// src/game/research.rs
use crate::game::resources::BaseValues;
use bevy::{
    platform::collections::{HashMap, HashSet},
    prelude::*,
};
use serde::{Deserialize, Serialize};

// --- STATIC DATA (The Rules) ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum TechId {
    BasicTools,
    Agriculture,
    Mining,
    SteelSmelting,
    // Add more...
}

#[derive(Debug, Clone)]
pub struct TechDefinition {
    pub id: TechId,
    pub name: String,
    pub description: String,
    pub cost: u32, // Cost in 'Research Points' or 'Zeni'
    pub tier: u32, // For UI grouping (Column 1, 2, 3...)
    pub dependencies: Vec<TechId>,
}

// A central repository for all tech definitions
#[derive(Resource)]
pub struct TechTree {
    pub defs: HashMap<TechId, TechDefinition>,
}

impl FromWorld for TechTree {
    fn from_world(_world: &mut World) -> Self {
        let mut defs = HashMap::new();

        // Helper to insert
        let mut add =
            |id: TechId, name: &str, desc: &str, cost: u32, tier: u32, deps: Vec<TechId>| {
                defs.insert(
                    id.clone(),
                    TechDefinition {
                        id,
                        name: name.to_string(),
                        description: desc.to_string(),
                        cost,
                        tier,
                        dependencies: deps,
                    },
                );
            };

        add(
            TechId::BasicTools,
            "Basic Tools",
            "Unlocks simple gathering.",
            50,
            0,
            vec![],
        );
        add(
            TechId::Agriculture,
            "Agriculture",
            "Grow your own food.",
            100,
            1,
            vec![TechId::BasicTools],
        );
        add(
            TechId::Mining,
            "Mining",
            "Extract ores from the earth.",
            150,
            1,
            vec![TechId::BasicTools],
        );
        add(
            TechId::SteelSmelting,
            "Steel Smelting",
            "Better alloys.",
            300,
            2,
            vec![TechId::Mining],
        );

        Self { defs }
    }
}

// --- DYNAMIC STATE (The Player's Progress) ---

#[derive(Resource, Default, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct ResearchState {
    pub unlocked: HashSet<TechId>,
}

impl ResearchState {
    pub fn is_unlocked(&self, id: &TechId) -> bool {
        self.unlocked.contains(id)
    }

    pub fn can_research(&self, id: &TechId, tree: &TechTree) -> bool {
        if self.is_unlocked(id) {
            return false;
        }

        let Some(def) = tree.defs.get(id) else {
            return false;
        };

        // Check dependencies
        for dep in &def.dependencies {
            if !self.is_unlocked(dep) {
                return false;
            }
        }

        true
    }
}
