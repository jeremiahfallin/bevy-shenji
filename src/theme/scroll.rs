use bevy::picking::events::{Pointer, Scroll};
use bevy::prelude::*;
use bevy_immediate::{CapSet, Imm, ImmEntity, ImplCap};

use crate::theme::primitives::{
    style::{CapabilityUiLayout, ImmUiStyleExt},
    visuals::CapabilityUiVisuals,
};

// 1. COMPONENTS
#[derive(Component, Default)]
pub struct UiScrollPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct ScrollableContent;

// 2. PLUGIN
pub struct ScrollWidgetPlugin;

impl Plugin for ScrollWidgetPlugin {
    fn build(&self, app: &mut App) {
        // 1. Listen for new scroll areas to attach input handlers
        app.add_observer(attach_scroll_handlers);
        // 2. Update the visual layout based on scroll position every frame
        app.add_systems(Update, update_scroll_layout);
    }
}

// 3. SYSTEMS & OBSERVERS

// Triggered when 'ScrollableContent' is added to an entity
fn attach_scroll_handlers(trigger: On<Add, ScrollableContent>, mut commands: Commands) {
    commands.entity(trigger.entity).observe(on_scroll_event);
}

// Triggered when the mouse wheel is scrolled over the entity
fn on_scroll_event(
    trigger: On<Pointer<Scroll>>,
    mut query: Query<(&mut UiScrollPosition, &ComputedNode, &ChildOf)>,
    parent_query: Query<&ComputedNode>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if let Ok((mut scroll, content_node, child_of)) = query.get_mut(trigger.entity) {
        let Ok(viewport_node) = parent_query.get(child_of.parent()) else {
            return;
        };

        let content_size = content_node.size();
        let viewport_size = viewport_node.size();

        // Only allow scrolling when content overflows the viewport
        let max_scroll_x = (content_size.x - viewport_size.x).max(0.0);
        let max_scroll_y = (content_size.y - viewport_size.y).max(0.0);

        // Adjust sensitivity (pixels per scroll line)
        const SCROLL_SENSITIVITY: f32 = 40.0;

        let mut dx = trigger.event().x;
        let mut dy = trigger.event().y;

        // Shift+scroll redirects vertical scroll to horizontal (standard UX
        // pattern). On Windows, the OS does not convert Shift+wheel to
        // horizontal scroll, so we handle it here.
        let shift_held =
            keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
        if shift_held && dx == 0.0 {
            dx = dy;
            dy = 0.0;
        }

        scroll.x = (scroll.x - dx * SCROLL_SENSITIVITY).clamp(0.0, max_scroll_x);
        scroll.y = (scroll.y - dy * SCROLL_SENSITIVITY).clamp(0.0, max_scroll_y);
    }
}

// Syncs the abstract ScrollPosition to the actual UI Node style
fn update_scroll_layout(
    mut query: Query<
        (&mut UiScrollPosition, &mut Node, &ComputedNode, &ChildOf),
        Changed<UiScrollPosition>,
    >,
    parent_query: Query<&ComputedNode>,
) {
    for (mut pos, mut node, content_node, child_of) in query.iter_mut() {
        // Clamp scroll position to actual overflow bounds
        if let Ok(viewport_node) = parent_query.get(child_of.parent()) {
            let content_size = content_node.size();
            let viewport_size = viewport_node.size();

            let max_x = (content_size.x - viewport_size.x).max(0.0);
            let max_y = (content_size.y - viewport_size.y).max(0.0);

            let clamped_x = pos.x.clamp(0.0, max_x);
            let clamped_y = pos.y.clamp(0.0, max_y);

            if pos.x != clamped_x || pos.y != clamped_y {
                pos.x = clamped_x;
                pos.y = clamped_y;
            }
        }

        // Move the content up/left by the scroll amount
        node.left = Val::Px(-pos.x);
        node.top = Val::Px(-pos.y);
    }
}

// 4. THE FLUENT API

pub trait ImmUiScrollExt<Cap> {
    fn scrollarea(
        self,
        inner_style_fn: impl FnOnce(&mut Node),
        content: impl FnOnce(&mut Imm<'_, '_, Cap>),
    ) -> Self;

    fn scroll_view(self, content: impl FnOnce(&mut Imm<'_, '_, Cap>)) -> Self;
}

impl<'w, 's, 'a, Cap> ImmUiScrollExt<Cap> for ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiLayout> + ImplCap<CapabilityUiVisuals>,
{
    fn scrollarea(
        self,
        inner_style_fn: impl FnOnce(&mut Node),
        content: impl FnOnce(&mut Imm<'_, '_, Cap>),
    ) -> Self {
        self
            // Outer Container (The Window)
            .style(|n| {
                n.display = Display::Grid;
                // Single cell grid ensures content overlaps/fills correctly
                n.grid_template_columns = vec![GridTrack::flex(1.0)];
                n.grid_template_rows = vec![GridTrack::flex(1.0)];
                // Critical: Clip content that moves outside
                n.overflow = Overflow::clip();
            })
            .add(|ui| {
                // Inner Container (The Moving Content)
                ui.ch()
                    .style(inner_style_fn)
                    .on_spawn_insert(|| (UiScrollPosition::default(), ScrollableContent))
                    .add(content);
            })
    }

    fn scroll_view(self, content: impl FnOnce(&mut Imm<'_, '_, Cap>)) -> Self {
        self.scrollarea(
            |n| {
                n.display = Display::Flex;
                n.flex_direction = FlexDirection::Column;
            },
            content,
        )
    }
}
