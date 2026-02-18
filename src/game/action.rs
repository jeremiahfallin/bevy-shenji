use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// A single action a character can perform
#[derive(Clone, Debug, Serialize, Deserialize, Reflect, PartialEq)]
pub enum Action {
    Idle,
    Gather {
        location: String,
        resource: String,
    },
    Collect {
        location: String,
        item: String,
    },
    Deposit {
        item: String,
    },
    Travel {
        destination: String,
    },
    Research {
        tech_id: String,
    },
    Craft {
        recipe_id: String,
        workstation: String,
    },
    Build {
        building: String,
    },
    Explore,
}

/// A repeating job (loops back to start when done)
#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct Job {
    pub name: String,
    pub actions: Vec<Action>,
}

/// Progress tracker for the current action
#[derive(Clone, Debug, Default, Serialize, Deserialize, Reflect)]
pub struct ActionProgress {
    pub current: u32,
    pub required: u32,
}

impl ActionProgress {
    pub fn new(required: u32) -> Self {
        Self {
            current: 0,
            required,
        }
    }

    pub fn tick(&mut self) -> bool {
        self.current += 1;
        self.current >= self.required
    }

    pub fn fraction(&self) -> f32 {
        if self.required == 0 {
            return 1.0;
        }
        self.current as f32 / self.required as f32
    }
}

/// Attached to each character entity
#[derive(Component, Clone, Debug, Default, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct ActionState {
    pub current_action: Option<Action>,
    pub progress: ActionProgress,
    pub action_queue: VecDeque<Action>,
    pub job_queue: Vec<Job>,
    pub current_job_index: usize,
}

impl ActionState {
    pub fn queue_action(&mut self, action: Action) {
        self.action_queue.push_back(action);
    }

    pub fn clear(&mut self) {
        self.current_action = None;
        self.progress = ActionProgress::default();
        self.action_queue.clear();
    }

    pub fn clear_jobs(&mut self) {
        self.job_queue.clear();
        self.current_job_index = 0;
    }
}

pub fn plugin(app: &mut App) {
    app.register_type::<ActionState>();
}
