//! Checkbox widget — typed builder, state component, change message.

use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::ecs::system::EntityCommands;
use bevy::picking::events::{Click, Pointer};
use bevy::prelude::*;
use bevy_declarative::element::Element;
use bevy_declarative::style::styled::Styled;

use crate::ui::tokens::palette::*;

/// Emitted when a non-disabled checkbox is clicked.
#[derive(Message, Debug, Clone, Copy)]
pub struct CheckboxChanged {
    pub entity: Entity,
    pub checked: bool,
}

/// State component attached to the checkbox root.
#[derive(Component, Debug, Clone, Copy)]
pub struct CheckboxState {
    pub checked: bool,
    pub disabled: bool,
}

pub struct Checkbox {
    node: Node,
    checked: bool,
    disabled: bool,
    label: Option<String>,
}

pub fn checkbox(initial: bool) -> Checkbox {
    Checkbox {
        node: Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            ..default()
        },
        checked: initial,
        disabled: false,
        label: None,
    }
}

impl Checkbox {
    pub fn label(mut self, s: impl Into<String>) -> Self {
        self.label = Some(s.into());
        self
    }

    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    pub fn spawn<'a>(self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let state = CheckboxState {
            checked: self.checked,
            disabled: self.disabled,
        };
        let label = self.label;
        let checked = self.checked;

        let mut ec = commands.spawn((self.node, state));
        ec.with_children(|inner| {
            let indicator_color = if checked { PRIMARY_500 } else { SURFACE_INSET };
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

impl Styled for Checkbox {
    fn style_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl Element for Checkbox {
    fn spawn_with_parent(self: Box<Self>, parent: &mut ChildSpawnerCommands) {
        let state = CheckboxState {
            checked: self.checked,
            disabled: self.disabled,
        };
        let label = self.label;
        let checked = self.checked;

        parent.spawn((self.node, state)).with_children(|inner| {
            let indicator_color = if checked { PRIMARY_500 } else { SURFACE_INSET };
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
    app.add_message::<CheckboxChanged>();
    app.add_observer(on_checkbox_click);
}

fn on_checkbox_click(
    click: On<Pointer<Click>>,
    mut q: Query<&mut CheckboxState>,
    mut events: MessageWriter<CheckboxChanged>,
) {
    let target = click.entity;
    let Ok(mut state) = q.get_mut(target) else {
        return;
    };
    if state.disabled {
        return;
    }
    state.checked = !state.checked;
    events.write(CheckboxChanged {
        entity: target,
        checked: state.checked,
    });
}
