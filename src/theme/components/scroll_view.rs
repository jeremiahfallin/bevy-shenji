use bevy::prelude::*;

#[derive(Component, Default, Debug, Clone, Reflect)]
pub struct ScrollPosition {
    pub offset_x: f32,
    pub offset_y: f32,
}

#[derive(Component, Default, Debug, Clone, Reflect)]
pub struct ScrollableContent;
