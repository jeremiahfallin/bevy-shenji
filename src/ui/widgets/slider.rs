//! Slider widget — track + thumb, drag updates value within range.

use std::ops::RangeInclusive;

use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::ecs::system::EntityCommands;
use bevy::picking::events::{Drag, Pointer};
use bevy::prelude::*;
use bevy_declarative::element::Element;
use bevy_declarative::style::styled::Styled;

use crate::ui::tokens::palette::*;

const SLIDER_WIDTH: f32 = 160.0;
const SLIDER_HEIGHT: f32 = 20.0;
const THUMB_SIZE: f32 = 16.0;
const TRACK_HEIGHT: f32 = 4.0;

/// Emitted when a slider's value changes due to a drag.
#[derive(Message, Debug, Clone, Copy)]
pub struct SliderChanged {
    pub entity: Entity,
    pub value: f32,
}

/// State component attached to the slider root.
#[derive(Component, Debug, Clone, Copy)]
pub struct SliderState {
    pub value: f32,
    pub min: f32,
    pub max: f32,
}

pub struct Slider {
    node: Node,
    value: f32,
    min: f32,
    max: f32,
}

pub fn slider(value: f32, range: RangeInclusive<f32>) -> Slider {
    let min = *range.start();
    let max = *range.end();
    let clamped = value.clamp(min, max);
    Slider {
        node: Node {
            width: Val::Px(SLIDER_WIDTH),
            height: Val::Px(SLIDER_HEIGHT),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            ..default()
        },
        value: clamped,
        min,
        max,
    }
}

impl Slider {
    fn build_state(&self) -> SliderState {
        SliderState {
            value: self.value,
            min: self.min,
            max: self.max,
        }
    }

    pub fn spawn<'a>(self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let state = self.build_state();
        let value = self.value;
        let min = self.min;
        let max = self.max;

        let mut ec = commands.spawn((self.node, state));
        ec.with_children(|inner| {
            spawn_track_and_thumb(inner, value, min, max);
        });
        ec
    }
}

fn spawn_track_and_thumb(parent: &mut ChildSpawnerCommands, value: f32, min: f32, max: f32) {
    let frac = if (max - min).abs() < f32::EPSILON {
        0.0
    } else {
        ((value - min) / (max - min)).clamp(0.0, 1.0)
    };
    // Track
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(TRACK_HEIGHT),
            ..default()
        },
        BackgroundColor(SURFACE_INSET),
    ));
    // Thumb (positioned absolutely)
    parent.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(frac * (SLIDER_WIDTH - THUMB_SIZE)),
            width: Val::Px(THUMB_SIZE),
            height: Val::Px(THUMB_SIZE),
            ..default()
        },
        BackgroundColor(PRIMARY_500),
    ));
}

impl Styled for Slider {
    fn style_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl Element for Slider {
    fn spawn_with_parent(self: Box<Self>, parent: &mut ChildSpawnerCommands) {
        let state = self.build_state();
        let value = self.value;
        let min = self.min;
        let max = self.max;

        parent.spawn((self.node, state)).with_children(|inner| {
            spawn_track_and_thumb(inner, value, min, max);
        });
    }
}

pub fn plugin(app: &mut App) {
    app.add_message::<SliderChanged>();
    app.add_observer(on_slider_drag);
}

fn on_slider_drag(
    drag: On<Pointer<Drag>>,
    mut q: Query<&mut SliderState>,
    mut events: MessageWriter<SliderChanged>,
) {
    let target = drag.entity;
    let Ok(mut state) = q.get_mut(target) else {
        return;
    };
    let dx = drag.delta.x;
    if dx.abs() < f32::EPSILON {
        return;
    }
    let range = state.max - state.min;
    if range.abs() < f32::EPSILON {
        return;
    }
    let value_delta = (dx / SLIDER_WIDTH) * range;
    let new_value = (state.value + value_delta).clamp(state.min, state.max);
    if (new_value - state.value).abs() < f32::EPSILON {
        return;
    }
    state.value = new_value;
    events.write(SliderChanged {
        entity: target,
        value: new_value,
    });
}
