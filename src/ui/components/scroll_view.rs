//! Scrollable container component built on `bevy_declarative`'s wheel-input
//! scroll handling.
//!
//! Phase A scope: builder API + overflow + ScrollPosition setup. No visual
//! scrollbar (track/thumb) — that's Phase B if/when needed by migrating screens.

use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_declarative::element::Element;
use bevy_declarative::style::styled::Styled;

/// Marker on the spawned ScrollView root.
#[derive(Component)]
pub struct ScrollViewMarker;

pub struct ScrollView {
    node: Node,
    bg: Option<Color>,
    children: Vec<Box<dyn Element>>,
    horizontal: bool,
    vertical: bool,
}

pub fn scroll_view() -> ScrollView {
    ScrollView {
        node: Node::default(),
        bg: None,
        children: Vec::new(),
        horizontal: false,
        vertical: true,
    }
}

impl ScrollView {
    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }

    pub fn vertical(mut self, on: bool) -> Self {
        self.vertical = on;
        self
    }

    pub fn child(mut self, c: impl Element + 'static) -> Self {
        self.children.push(Box::new(c));
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    fn finalize_node(&self, mut node: Node) -> Node {
        node.overflow.x = if self.horizontal {
            OverflowAxis::Scroll
        } else {
            OverflowAxis::Visible
        };
        node.overflow.y = if self.vertical {
            OverflowAxis::Scroll
        } else {
            OverflowAxis::Visible
        };
        node
    }

    pub fn spawn<'a>(self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let node = self.finalize_node(self.node.clone());
        let bg = self.bg;
        let children = self.children;

        let mut ec = commands.spawn((node, ScrollPosition::default(), ScrollViewMarker));
        if let Some(c) = bg {
            ec.insert(BackgroundColor(c));
        }
        if !children.is_empty() {
            ec.with_children(|inner| {
                for child in children {
                    child.spawn_with_parent(inner);
                }
            });
        }
        ec
    }
}

impl Styled for ScrollView {
    fn style_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl Element for ScrollView {
    fn spawn_with_parent(self: Box<Self>, parent: &mut ChildSpawnerCommands) {
        let node = self.finalize_node(self.node.clone());
        let bg = self.bg;
        let children = self.children;

        let mut ec = parent.spawn((node, ScrollPosition::default(), ScrollViewMarker));
        if let Some(c) = bg {
            ec.insert(BackgroundColor(c));
        }
        if !children.is_empty() {
            ec.with_children(|inner| {
                for child in children {
                    child.spawn_with_parent(inner);
                }
            });
        }
    }
}
