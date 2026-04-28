//! Tabs widget — label row + active content panel.
//!
//! TODO: Active-tab content swapping in response to `TabsState` changes is not
//! yet implemented; the initial spawn renders only the active tab's content.

use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::ecs::system::EntityCommands;
use bevy::picking::events::{Click, Pointer};
use bevy::prelude::*;
use bevy_declarative::element::Element;
use bevy_declarative::style::styled::Styled;

use crate::ui::tokens::palette::*;

/// Emitted when the active tab changes via a label click.
#[derive(Message, Debug, Clone, Copy)]
pub struct TabsChanged {
    pub entity: Entity,
    pub active: usize,
}

/// State component attached to the tabs root.
#[derive(Component, Debug, Clone, Copy)]
pub struct TabsState {
    pub active: usize,
    pub count: usize,
}

/// Marker component placed on each tab label entity.
#[derive(Component, Debug, Clone, Copy)]
pub struct TabLabelButton {
    pub tabs_root: Entity,
    pub index: usize,
}

struct TabEntry {
    label: String,
    content: Box<dyn Element>,
}

pub struct Tabs {
    node: Node,
    initial_active: usize,
    entries: Vec<TabEntry>,
}

pub fn tabs(initial_active: usize) -> Tabs {
    Tabs {
        node: Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        initial_active,
        entries: Vec::new(),
    }
}

impl Tabs {
    pub fn tab(mut self, label: impl Into<String>, content: impl Element + 'static) -> Self {
        self.entries.push(TabEntry {
            label: label.into(),
            content: Box::new(content),
        });
        self
    }

    fn resolved_active(&self) -> usize {
        if self.entries.is_empty() {
            0
        } else {
            self.initial_active.min(self.entries.len() - 1)
        }
    }

    fn build_state(&self) -> TabsState {
        TabsState {
            active: self.resolved_active(),
            count: self.entries.len(),
        }
    }

    pub fn spawn<'a>(self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let state = self.build_state();
        let active = state.active;
        let entries = self.entries;

        let mut ec = commands.spawn((self.node, state));
        let root_id = ec.id();
        ec.with_children(|inner| {
            spawn_tabs_children(inner, root_id, active, entries);
        });
        ec
    }
}

fn spawn_tabs_children(
    parent: &mut ChildSpawnerCommands,
    root: Entity,
    active: usize,
    entries: Vec<TabEntry>,
) {
    // Label row.
    parent
        .spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            ..default()
        })
        .with_children(|row| {
            for (index, entry) in entries.iter().enumerate() {
                let bg = if index == active {
                    PRIMARY_500
                } else {
                    SURFACE_INSET
                };
                row.spawn((
                    Node {
                        padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                        margin: UiRect::right(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(bg),
                    TabLabelButton {
                        tabs_root: root,
                        index,
                    },
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(entry.label.clone()),
                        TextFont::default(),
                        TextColor(TEXT_PRIMARY),
                    ));
                });
            }
        });

    // Active content.
    let mut entries = entries;
    if active < entries.len() {
        let content = entries.remove(active).content;
        content.spawn_with_parent(parent);
    }
}

impl Styled for Tabs {
    fn style_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl Element for Tabs {
    fn spawn_with_parent(self: Box<Self>, parent: &mut ChildSpawnerCommands) {
        let state = self.build_state();
        let active = state.active;
        let entries = self.entries;

        let mut ec = parent.spawn((self.node, state));
        let root_id = ec.id();
        ec.with_children(|inner| {
            spawn_tabs_children(inner, root_id, active, entries);
        });
    }
}

pub fn plugin(app: &mut App) {
    app.add_message::<TabsChanged>();
    app.add_observer(on_tab_label_click);
}

fn on_tab_label_click(
    click: On<Pointer<Click>>,
    labels: Query<&TabLabelButton>,
    mut roots: Query<&mut TabsState>,
    mut events: MessageWriter<TabsChanged>,
) {
    let target = click.entity;
    let Ok(label) = labels.get(target) else {
        return;
    };
    let Ok(mut state) = roots.get_mut(label.tabs_root) else {
        return;
    };
    if label.index >= state.count {
        return;
    }
    if state.active == label.index {
        return;
    }
    state.active = label.index;
    events.write(TabsChanged {
        entity: label.tabs_root,
        active: label.index,
    });
}
