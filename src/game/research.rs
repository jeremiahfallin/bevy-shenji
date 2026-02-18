// src/game/research.rs
use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::data::GameData;

/// Dynamic state tracking the player's research progress.
#[derive(Resource, Default, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct ResearchState {
    /// Set of research IDs that have been completed.
    pub unlocked: HashSet<String>,
    /// The research ID currently being worked on (if any).
    pub current_research: Option<String>,
    /// Number of ticks of progress on the current research.
    pub research_progress: u32,
}

impl ResearchState {
    /// Returns `true` if the given research has already been completed.
    pub fn is_unlocked(&self, id: &str) -> bool {
        self.unlocked.contains(id)
    }

    /// Returns `true` if the given research can be started:
    /// - it is not already unlocked
    /// - all prerequisites are unlocked
    /// - the research ID exists in GameData
    pub fn can_research(&self, id: &str, game_data: &GameData) -> bool {
        if self.is_unlocked(id) {
            return false;
        }

        let Some(def) = game_data.get_research(id) else {
            return false;
        };

        // Check all prerequisites are unlocked
        for prereq in &def.prerequisites {
            if !self.is_unlocked(prereq) {
                return false;
            }
        }

        true
    }
}
