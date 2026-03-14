use bevy::picking::events::{Click, Drag, DragEnd, DragStart, Out, Over, Pointer, Scroll};
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

/// Which axis a scrollbar controls.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScrollAxis {
    Horizontal,
    Vertical,
}

/// Marker on a scrollbar track node. Points to the ScrollableContent entity.
#[derive(Component)]
pub struct ScrollbarTrack {
    pub axis: ScrollAxis,
    pub content_entity: Entity,
}

/// Marker on a scrollbar thumb node. Points to the ScrollableContent entity.
#[derive(Component)]
pub struct ScrollbarThumb {
    pub axis: ScrollAxis,
    pub content_entity: Entity,
}

/// Drives scrollbar fade animation.
#[derive(Component)]
pub struct ScrollbarVisibility {
    pub opacity: f32,
    pub target_opacity: f32,
    pub last_activity: f32,
    pub fade_delay: f32,
}

impl Default for ScrollbarVisibility {
    fn default() -> Self {
        Self {
            opacity: 0.0,
            target_opacity: 0.0,
            last_activity: 0.0,
            fade_delay: 1.5,
        }
    }
}

/// Inserted on thumb during drag. Removed on drag end.
#[derive(Component)]
pub struct ScrollbarDragState {
    pub start_scroll: f32,
    pub start_mouse: f32,
}

// Scrollbar visual constants
const SCROLLBAR_SIZE: f32 = 6.0;
const SCROLLBAR_MIN_THUMB: f32 = 20.0;
const SCROLLBAR_IDLE_ALPHA: f32 = 0.5;
const SCROLLBAR_HOVER_ALPHA: f32 = 0.7;
const SCROLLBAR_DRAG_ALPHA: f32 = 0.9;
const SCROLLBAR_FADE_IN_SPEED: f32 = 6.0;
const SCROLLBAR_FADE_OUT_SPEED: f32 = 3.0;

// 2. PLUGIN
pub struct ScrollWidgetPlugin;

impl Plugin for ScrollWidgetPlugin {
    fn build(&self, app: &mut App) {
        // 1. Listen for new scroll areas to attach input handlers
        app.add_observer(attach_scroll_handlers);
        // 1b. Attach drag/hover handlers to thumbs, click handler to tracks
        app.add_observer(attach_scrollbar_drag_handlers);
        app.add_observer(attach_scrollbar_track_click);
        // 2. Update the visual layout based on scroll position every frame
        app.add_systems(Update, update_scroll_layout);
        // 3. Size and position scrollbar thumbs each frame
        app.add_systems(
            Update,
            update_scrollbar_layout.after(update_scroll_layout),
        );
        // 4. Fade animation: visibility → fade → apply opacity
        app.add_systems(
            Update,
            (
                update_scrollbar_visibility_on_scroll,
                tick_scrollbar_fade,
                apply_scrollbar_opacity,
            )
                .chain()
                .after(update_scrollbar_layout),
        );
    }
}

// 3. SYSTEMS & OBSERVERS

// Triggered when 'ScrollableContent' is added to an entity.
// Attaches scroll input handler and spawns scrollbar track+thumb pairs.
fn attach_scroll_handlers(
    trigger: On<Add, ScrollableContent>,
    mut commands: Commands,
    child_of_query: Query<&ChildOf>,
) {
    let content_entity = trigger.entity;
    commands.entity(content_entity).observe(on_scroll_event);

    // Look up the viewport (parent) entity so scrollbars become siblings of content.
    let Ok(child_of) = child_of_query.get(content_entity) else {
        return;
    };
    let viewport_entity = child_of.parent();

    // Spawn vertical scrollbar: track with thumb child
    let vert_thumb = commands
        .spawn((
            ScrollbarThumb {
                axis: ScrollAxis::Vertical,
                content_entity,
            },
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(SCROLLBAR_SIZE),
                min_height: Val::Px(SCROLLBAR_MIN_THUMB),
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                ..default()
            },
            BorderRadius::all(Val::Px(SCROLLBAR_SIZE / 2.0)),
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
        ))
        .id();

    commands
        .spawn((
            ScrollbarTrack {
                axis: ScrollAxis::Vertical,
                content_entity,
            },
            ScrollbarVisibility::default(),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(SCROLLBAR_SIZE),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..default()
            },
            ChildOf(viewport_entity),
        ))
        .add_child(vert_thumb);

    // Spawn horizontal scrollbar: track with thumb child
    let horiz_thumb = commands
        .spawn((
            ScrollbarThumb {
                axis: ScrollAxis::Horizontal,
                content_entity,
            },
            Node {
                position_type: PositionType::Absolute,
                height: Val::Px(SCROLLBAR_SIZE),
                min_width: Val::Px(SCROLLBAR_MIN_THUMB),
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                ..default()
            },
            BorderRadius::all(Val::Px(SCROLLBAR_SIZE / 2.0)),
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
        ))
        .id();

    commands
        .spawn((
            ScrollbarTrack {
                axis: ScrollAxis::Horizontal,
                content_entity,
            },
            ScrollbarVisibility::default(),
            Node {
                position_type: PositionType::Absolute,
                height: Val::Px(SCROLLBAR_SIZE),
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                ..default()
            },
            ChildOf(viewport_entity),
        ))
        .add_child(horiz_thumb);
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

// Sizes and positions scrollbar thumbs based on content vs viewport sizes.
// Hides tracks when there is no overflow.
fn update_scrollbar_layout(
    content_query: Query<(&UiScrollPosition, &ComputedNode, &ChildOf)>,
    viewport_query: Query<&ComputedNode>,
    mut track_query: Query<(&ScrollbarTrack, &mut Node, &ComputedNode)>,
    mut thumb_query: Query<(&ScrollbarThumb, &mut Node), Without<ScrollbarTrack>>,
) {
    for (track, mut track_node, track_computed) in track_query.iter_mut() {
        let Ok((scroll_pos, content_node, child_of)) = content_query.get(track.content_entity)
        else {
            continue;
        };
        let Ok(viewport_node) = viewport_query.get(child_of.parent()) else {
            continue;
        };

        let content_size = content_node.size();
        let viewport_size = viewport_node.size();

        let (content_len, viewport_len, scroll_val) = match track.axis {
            ScrollAxis::Vertical => (content_size.y, viewport_size.y, scroll_pos.y),
            ScrollAxis::Horizontal => (content_size.x, viewport_size.x, scroll_pos.x),
        };

        let max_scroll = (content_len - viewport_len).max(0.0);

        // Hide track if no overflow
        if max_scroll <= 0.0 {
            track_node.display = Display::None;
            continue;
        }
        track_node.display = Display::Flex;

        // Compute thumb size and position
        let track_len = match track.axis {
            ScrollAxis::Vertical => track_computed.size().y,
            ScrollAxis::Horizontal => track_computed.size().x,
        };

        let thumb_len =
            ((viewport_len / content_len) * track_len).clamp(SCROLLBAR_MIN_THUMB, track_len);
        let thumb_pos = if max_scroll > 0.0 {
            (scroll_val / max_scroll) * (track_len - thumb_len)
        } else {
            0.0
        };

        // Update matching thumb
        for (thumb, mut thumb_node) in thumb_query.iter_mut() {
            if thumb.content_entity == track.content_entity && thumb.axis == track.axis {
                match thumb.axis {
                    ScrollAxis::Vertical => {
                        thumb_node.height = Val::Px(thumb_len);
                        thumb_node.top = Val::Px(thumb_pos);
                    }
                    ScrollAxis::Horizontal => {
                        thumb_node.width = Val::Px(thumb_len);
                        thumb_node.left = Val::Px(thumb_pos);
                    }
                }
                break;
            }
        }
    }
}

// — Scrollbar interaction observers —

/// When a ScrollbarThumb is added, attach drag and hover observers.
fn attach_scrollbar_drag_handlers(trigger: On<Add, ScrollbarThumb>, mut commands: Commands) {
    let thumb_entity = trigger.entity;
    commands
        .entity(thumb_entity)
        .observe(on_thumb_drag_start)
        .observe(on_thumb_drag)
        .observe(on_thumb_drag_end)
        .observe(on_thumb_hover_in)
        .observe(on_thumb_hover_out);
}

/// When a ScrollbarTrack is added, attach click observer.
fn attach_scrollbar_track_click(trigger: On<Add, ScrollbarTrack>, mut commands: Commands) {
    let track_entity = trigger.entity;
    commands.entity(track_entity).observe(on_track_click);
}

fn on_thumb_drag_start(
    trigger: On<Pointer<DragStart>>,
    thumb_query: Query<&ScrollbarThumb>,
    scroll_query: Query<&UiScrollPosition>,
    mut track_query: Query<(&ScrollbarTrack, &mut ScrollbarVisibility)>,
    mut commands: Commands,
) {
    let thumb_entity = trigger.entity;
    let Ok(thumb) = thumb_query.get(thumb_entity) else {
        return;
    };
    let Ok(scroll_pos) = scroll_query.get(thumb.content_entity) else {
        return;
    };

    let pointer_pos = trigger.event().pointer_location.position;
    let (start_scroll, start_mouse) = match thumb.axis {
        ScrollAxis::Vertical => (scroll_pos.y, pointer_pos.y),
        ScrollAxis::Horizontal => (scroll_pos.x, pointer_pos.x),
    };

    commands
        .entity(thumb_entity)
        .insert(ScrollbarDragState {
            start_scroll,
            start_mouse,
        });

    // Set track to drag opacity
    for (track, mut vis) in track_query.iter_mut() {
        if track.content_entity == thumb.content_entity && track.axis == thumb.axis {
            vis.target_opacity = SCROLLBAR_DRAG_ALPHA;
            break;
        }
    }
}

fn on_thumb_drag(
    trigger: On<Pointer<Drag>>,
    thumb_query: Query<(&ScrollbarThumb, &ScrollbarDragState)>,
    mut scroll_query: Query<(&mut UiScrollPosition, &ComputedNode, &ChildOf)>,
    viewport_query: Query<&ComputedNode, Without<UiScrollPosition>>,
    track_query: Query<(&ScrollbarTrack, &ComputedNode), Without<UiScrollPosition>>,
) {
    let thumb_entity = trigger.entity;
    let Ok((thumb, drag_state)) = thumb_query.get(thumb_entity) else {
        return;
    };
    let Ok((mut scroll_pos, content_node, child_of)) = scroll_query.get_mut(thumb.content_entity)
    else {
        return;
    };
    let Ok(viewport_node) = viewport_query.get(child_of.parent()) else {
        return;
    };

    let content_size = content_node.size();
    let viewport_size = viewport_node.size();

    let (content_len, viewport_len) = match thumb.axis {
        ScrollAxis::Vertical => (content_size.y, viewport_size.y),
        ScrollAxis::Horizontal => (content_size.x, viewport_size.x),
    };

    let max_scroll = (content_len - viewport_len).max(0.0);
    if max_scroll <= 0.0 {
        return;
    }

    // Find matching track to get track length
    let mut track_len = 0.0;
    for (track, track_computed) in track_query.iter() {
        if track.content_entity == thumb.content_entity && track.axis == thumb.axis {
            track_len = match thumb.axis {
                ScrollAxis::Vertical => track_computed.size().y,
                ScrollAxis::Horizontal => track_computed.size().x,
            };
            break;
        }
    }

    if track_len <= 0.0 {
        return;
    }

    let thumb_len =
        ((viewport_len / content_len) * track_len).clamp(SCROLLBAR_MIN_THUMB, track_len);
    let scrollable_track = track_len - thumb_len;

    if scrollable_track <= 0.0 {
        return;
    }

    let pointer_pos = trigger.event().pointer_location.position;
    let current_mouse = match thumb.axis {
        ScrollAxis::Vertical => pointer_pos.y,
        ScrollAxis::Horizontal => pointer_pos.x,
    };

    let mouse_delta = current_mouse - drag_state.start_mouse;
    let scroll_delta = mouse_delta * (max_scroll / scrollable_track);
    let new_scroll = (drag_state.start_scroll + scroll_delta).clamp(0.0, max_scroll);

    match thumb.axis {
        ScrollAxis::Vertical => scroll_pos.y = new_scroll,
        ScrollAxis::Horizontal => scroll_pos.x = new_scroll,
    }
}

fn on_thumb_drag_end(
    trigger: On<Pointer<DragEnd>>,
    thumb_query: Query<&ScrollbarThumb>,
    mut track_query: Query<(&ScrollbarTrack, &mut ScrollbarVisibility)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let thumb_entity = trigger.entity;
    commands
        .entity(thumb_entity)
        .remove::<ScrollbarDragState>();

    let Ok(thumb) = thumb_query.get(thumb_entity) else {
        return;
    };

    for (track, mut vis) in track_query.iter_mut() {
        if track.content_entity == thumb.content_entity && track.axis == thumb.axis {
            vis.target_opacity = SCROLLBAR_IDLE_ALPHA;
            vis.last_activity = time.elapsed_secs();
            break;
        }
    }
}

fn on_track_click(
    trigger: On<Pointer<Click>>,
    track_query: Query<(&ScrollbarTrack, &ComputedNode, &GlobalTransform)>,
    mut scroll_query: Query<(&mut UiScrollPosition, &ComputedNode, &ChildOf)>,
    viewport_query: Query<&ComputedNode, Without<UiScrollPosition>>,
) {
    let track_entity = trigger.entity;
    let Ok((track, track_computed, track_transform)) = track_query.get(track_entity) else {
        return;
    };
    let Ok((mut scroll_pos, content_node, child_of)) = scroll_query.get_mut(track.content_entity)
    else {
        return;
    };
    let Ok(viewport_node) = viewport_query.get(child_of.parent()) else {
        return;
    };

    let content_size = content_node.size();
    let viewport_size = viewport_node.size();

    let (content_len, viewport_len) = match track.axis {
        ScrollAxis::Vertical => (content_size.y, viewport_size.y),
        ScrollAxis::Horizontal => (content_size.x, viewport_size.x),
    };

    let max_scroll = (content_len - viewport_len).max(0.0);
    if max_scroll <= 0.0 {
        return;
    }

    let track_size = track_computed.size();
    let track_len = match track.axis {
        ScrollAxis::Vertical => track_size.y,
        ScrollAxis::Horizontal => track_size.x,
    };

    if track_len <= 0.0 {
        return;
    }

    // Get click position relative to track
    let click_pos = trigger.event().pointer_location.position;
    let track_global_pos = track_transform.translation().truncate();
    // The track's global transform gives us the center; we need top-left.
    // ComputedNode size is in logical pixels; transform is also in logical pixels for UI.
    let track_top_left = track_global_pos - track_size * 0.5;

    let relative_pos = match track.axis {
        ScrollAxis::Vertical => click_pos.y - track_top_left.y,
        ScrollAxis::Horizontal => click_pos.x - track_top_left.x,
    };

    // Scale by inverse_scale_factor to convert from screen to logical pixels
    let isf = track_computed.inverse_scale_factor();
    let relative_logical = relative_pos * isf;

    let ratio = (relative_logical / track_len).clamp(0.0, 1.0);
    let new_scroll = (ratio * max_scroll).clamp(0.0, max_scroll);

    match track.axis {
        ScrollAxis::Vertical => scroll_pos.y = new_scroll,
        ScrollAxis::Horizontal => scroll_pos.x = new_scroll,
    }
}

fn on_thumb_hover_in(
    trigger: On<Pointer<Over>>,
    thumb_query: Query<&ScrollbarThumb>,
    mut track_query: Query<(&ScrollbarTrack, &mut ScrollbarVisibility)>,
) {
    let thumb_entity = trigger.entity;
    let Ok(thumb) = thumb_query.get(thumb_entity) else {
        return;
    };

    for (track, mut vis) in track_query.iter_mut() {
        if track.content_entity == thumb.content_entity && track.axis == thumb.axis {
            vis.target_opacity = SCROLLBAR_HOVER_ALPHA;
            break;
        }
    }
}

fn on_thumb_hover_out(
    trigger: On<Pointer<Out>>,
    thumb_query: Query<&ScrollbarThumb>,
    drag_query: Query<&ScrollbarDragState>,
    mut track_query: Query<(&ScrollbarTrack, &mut ScrollbarVisibility)>,
    time: Res<Time>,
) {
    let thumb_entity = trigger.entity;
    let Ok(thumb) = thumb_query.get(thumb_entity) else {
        return;
    };

    // Don't reduce opacity if currently dragging
    if drag_query.get(thumb_entity).is_ok() {
        return;
    }

    for (track, mut vis) in track_query.iter_mut() {
        if track.content_entity == thumb.content_entity && track.axis == thumb.axis {
            vis.target_opacity = SCROLLBAR_IDLE_ALPHA;
            vis.last_activity = time.elapsed_secs();
            break;
        }
    }
}

// — Fade animation systems —

/// When scroll position changes, mark matching tracks as active.
fn update_scrollbar_visibility_on_scroll(
    changed_scroll: Query<Entity, Changed<UiScrollPosition>>,
    mut track_query: Query<(&ScrollbarTrack, &mut ScrollbarVisibility)>,
    time: Res<Time>,
) {
    for content_entity in changed_scroll.iter() {
        for (track, mut vis) in track_query.iter_mut() {
            if track.content_entity == content_entity {
                vis.last_activity = time.elapsed_secs();
                vis.target_opacity = SCROLLBAR_IDLE_ALPHA;
            }
        }
    }
}

/// Lerps opacity toward target; triggers fade-out after delay (skips if dragging).
fn tick_scrollbar_fade(
    time: Res<Time>,
    mut track_query: Query<(&ScrollbarTrack, &mut ScrollbarVisibility)>,
    drag_query: Query<(Entity, &ScrollbarThumb), With<ScrollbarDragState>>,
) {
    let dt = time.delta_secs();
    let now = time.elapsed_secs();

    for (track, mut vis) in track_query.iter_mut() {
        // Check if any thumb for this track is being dragged
        let is_dragging = drag_query.iter().any(|(_, thumb)| {
            thumb.content_entity == track.content_entity && thumb.axis == track.axis
        });

        if !is_dragging && now - vis.last_activity > vis.fade_delay {
            vis.target_opacity = 0.0;
        }

        let speed = if vis.target_opacity > vis.opacity {
            SCROLLBAR_FADE_IN_SPEED
        } else {
            SCROLLBAR_FADE_OUT_SPEED
        };
        vis.opacity = lerp_toward(vis.opacity, vis.target_opacity, speed * dt);
    }
}

fn lerp_toward(current: f32, target: f32, max_delta: f32) -> f32 {
    if (target - current).abs() <= max_delta {
        target
    } else if target > current {
        current + max_delta
    } else {
        current - max_delta
    }
}

/// Applies the track's visibility opacity to the thumb's BackgroundColor.
fn apply_scrollbar_opacity(
    track_query: Query<(&ScrollbarTrack, &ScrollbarVisibility)>,
    mut thumb_query: Query<(&ScrollbarThumb, &mut BackgroundColor)>,
) {
    for (thumb, mut bg) in thumb_query.iter_mut() {
        for (track, vis) in track_query.iter() {
            if track.content_entity == thumb.content_entity && track.axis == thumb.axis {
                bg.0 = Color::srgba(1.0, 1.0, 1.0, vis.opacity);
                break;
            }
        }
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
                n.display = Display::Flex;
                // Critical: Clip content that moves outside
                n.overflow = Overflow::clip();
            })
            .add(|ui| {
                // Inner Container (The Moving Content)
                // flex_shrink: 0 ensures content keeps its natural size
                // and can overflow the parent (which clips it)
                ui.ch()
                    .style(|n| {
                        n.flex_shrink = 0.0;
                    })
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
