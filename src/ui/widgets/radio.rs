//! Radio group widget — exclusive selection within a shared `group_id`.

use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::ecs::system::EntityCommands;
use bevy::picking::events::{Click, Pointer};
use bevy::prelude::*;
use bevy_declarative::element::Element;
use bevy_declarative::style::styled::Styled;

use crate::ui::tokens::palette::*;

/// Emitted when a radio in a group becomes newly selected.
#[derive(Message, Debug, Clone)]
pub struct RadioChanged {
    pub entity: Entity,
    pub group_id: String,
    pub value: String,
}

/// State component attached to each radio option.
#[derive(Component, Debug, Clone)]
pub struct RadioState {
    pub group_id: String,
    pub value: String,
    pub selected: bool,
    pub disabled: bool,
}

pub struct Radio {
    node: Node,
    group_id: String,
    value: String,
    selected: bool,
    disabled: bool,
    label: Option<String>,
}

pub fn radio(group_id: impl Into<String>, value: impl Into<String>) -> Radio {
    Radio {
        node: Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            ..default()
        },
        group_id: group_id.into(),
        value: value.into(),
        selected: false,
        disabled: false,
        label: None,
    }
}

impl Radio {
    pub fn selected(mut self, sel: bool) -> Self {
        self.selected = sel;
        self
    }

    pub fn label(mut self, s: impl Into<String>) -> Self {
        self.label = Some(s.into());
        self
    }

    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    fn build_state(&self) -> RadioState {
        RadioState {
            group_id: self.group_id.clone(),
            value: self.value.clone(),
            selected: self.selected,
            disabled: self.disabled,
        }
    }

    pub fn spawn<'a>(self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let state = self.build_state();
        let label = self.label;
        let selected = self.selected;

        let mut ec = commands.spawn((self.node, state));
        ec.with_children(|inner| {
            let indicator_color = if selected { PRIMARY_500 } else { SURFACE_INSET };
            inner.spawn((
                Node {
                    width: Val::Px(16.0),
                    height: Val::Px(16.0),
                    margin: UiRect::right(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(indicator_color),
            ));
            if let Some(label_text) = label {
                inner.spawn((
                    Text::new(label_text),
                    TextFont::default(),
                    TextColor(TEXT_PRIMARY),
                ));
            }
        });
        ec
    }
}

impl Styled for Radio {
    fn style_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl Element for Radio {
    fn spawn_with_parent(self: Box<Self>, parent: &mut ChildSpawnerCommands) {
        let state = self.build_state();
        let label = self.label;
        let selected = self.selected;

        parent.spawn((self.node, state)).with_children(|inner| {
            let indicator_color = if selected { PRIMARY_500 } else { SURFACE_INSET };
            inner.spawn((
                Node {
                    width: Val::Px(16.0),
                    height: Val::Px(16.0),
                    margin: UiRect::right(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(indicator_color),
            ));
            if let Some(label_text) = label {
                inner.spawn((
                    Text::new(label_text),
                    TextFont::default(),
                    TextColor(TEXT_PRIMARY),
                ));
            }
        });
    }
}

pub fn plugin(app: &mut App) {
    app.add_message::<RadioChanged>();
    app.add_observer(on_radio_click);
}

fn on_radio_click(
    click: On<Pointer<Click>>,
    mut q: Query<(Entity, &mut RadioState)>,
    mut events: MessageWriter<RadioChanged>,
) {
    let target = click.entity;

    // Read target's state without holding the borrow during the loop.
    let Ok((_, target_state)) = q.get(target) else {
        return;
    };
    if target_state.disabled {
        return;
    }
    if target_state.selected {
        // Already selected — no event, no state change.
        return;
    }
    let group_id = target_state.group_id.clone();
    let value = target_state.value.clone();

    // Iterate all radios in the same group and update selection.
    for (entity, mut state) in q.iter_mut() {
        if state.group_id != group_id {
            continue;
        }
        let want_selected = entity == target;
        if state.selected != want_selected {
            state.selected = want_selected;
        }
    }

    events.write(RadioChanged {
        entity: target,
        group_id,
        value,
    });
}
