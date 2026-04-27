use bevy::picking::events::{Drag, DragEnd, DragStart, Out, Over, Pointer, Scroll};
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAxis {
    Horizontal,
    Vertical,
}

/// Marker on a scrollbar track node. No entity references — uses hierarchy.
#[derive(Component)]
pub struct ScrollbarTrack {
    pub axis: ScrollAxis,
}

/// Marker on a scrollbar thumb node. No entity references — uses hierarchy.
#[derive(Component)]
pub struct ScrollbarThumb {
    pub axis: ScrollAxis,
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
            opacity: SCROLLBAR_IDLE_ALPHA,
            target_opacity: SCROLLBAR_IDLE_ALPHA,
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
        // Attach scroll input handler when content is spawned
        app.add_observer(attach_scroll_handlers);
        // Attach drag/hover handlers to thumbs
        app.add_observer(attach_scrollbar_drag_handlers);
        // Update the visual layout based on scroll position every frame
        app.add_systems(Update, update_scroll_layout);
        // Size and position scrollbar thumbs each frame.
        // Runs in Update so Node changes are applied BEFORE PostUpdate layout.
        // ComputedNode from the previous frame provides valid viewport/content sizes.
        app.add_systems(Update, update_scrollbar_layout.after(update_scroll_layout));
        // Fade animation
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

// 3. HELPER: find the ScrollableContent sibling of a track/thumb entity.
// Walks: entity → parent (viewport) → children → find ScrollableContent.
fn find_content_sibling(
    viewport_entity: Entity,
    children_query: &Query<&Children>,
    content_marker: &Query<Entity, With<ScrollableContent>>,
) -> Option<Entity> {
    let children = children_query.get(viewport_entity).ok()?;
    children
        .iter()
        .find(|&child| content_marker.contains(child))
}

// 4. SYSTEMS & OBSERVERS

/// Triggered when ScrollableContent is added. Only attaches the scroll observer.
fn attach_scroll_handlers(trigger: On<Add, ScrollableContent>, mut commands: Commands) {
    commands.entity(trigger.entity).observe(on_scroll_event);
}

/// Triggered when the mouse wheel is scrolled over the content entity.
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

        let max_scroll_x = (content_size.x - viewport_size.x).max(0.0);
        let max_scroll_y = (content_size.y - viewport_size.y).max(0.0);

        const SCROLL_SENSITIVITY: f32 = 40.0;

        let mut dx = trigger.event().x;
        let mut dy = trigger.event().y;

        // Shift+scroll redirects vertical to horizontal (Windows doesn't do this natively)
        let shift_held = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
        if shift_held && dx == 0.0 {
            dx = dy;
            dy = 0.0;
        }

        scroll.x = (scroll.x - dx * SCROLL_SENSITIVITY).clamp(0.0, max_scroll_x);
        scroll.y = (scroll.y - dy * SCROLL_SENSITIVITY).clamp(0.0, max_scroll_y);
    }
}

/// Syncs UiScrollPosition to Node left/top offsets.
fn update_scroll_layout(
    mut query: Query<
        (&mut UiScrollPosition, &mut Node, &ComputedNode, &ChildOf),
        Changed<UiScrollPosition>,
    >,
    parent_query: Query<&ComputedNode>,
) {
    for (mut pos, mut node, content_node, child_of) in query.iter_mut() {
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

        node.left = Val::Px(-pos.x);
        node.top = Val::Px(-pos.y);
    }
}

/// Sizes and positions scrollbar tracks and thumbs based on content vs viewport.
/// Finds content entity dynamically via parent-child hierarchy (no stored references).
fn update_scrollbar_layout(
    track_query: Query<(Entity, &ScrollbarTrack, &ChildOf)>,
    children_query: Query<&Children>,
    content_marker: Query<Entity, With<ScrollableContent>>,
    content_data: Query<(&UiScrollPosition, &ComputedNode)>,
    viewport_computed: Query<&ComputedNode>,
    mut node_query: Query<&mut Node>,
    thumb_query: Query<(Entity, &ScrollbarThumb)>,
) {
    for (track_entity, track, track_child_of) in track_query.iter() {
        let viewport_entity = track_child_of.parent();

        // Find the ScrollableContent sibling dynamically
        let Some(content_entity) =
            find_content_sibling(viewport_entity, &children_query, &content_marker)
        else {
            // No content sibling found — hide track
            if let Ok(mut track_node) = node_query.get_mut(track_entity) {
                track_node.display = Display::None;
            }
            continue;
        };

        let Ok((scroll_pos, content_computed)) = content_data.get(content_entity) else {
            continue;
        };
        let Ok(vp_computed) = viewport_computed.get(viewport_entity) else {
            continue;
        };

        let content_size = content_computed.size();
        let viewport_size = vp_computed.size();

        let (content_len, viewport_len, scroll_val) = match track.axis {
            ScrollAxis::Vertical => (content_size.y, viewport_size.y, scroll_pos.y),
            ScrollAxis::Horizontal => (content_size.x, viewport_size.x, scroll_pos.x),
        };

        let max_scroll = (content_len - viewport_len).max(0.0);

        // Hide track if no overflow
        if let Ok(mut track_node) = node_query.get_mut(track_entity) {
            if max_scroll <= 0.0 {
                track_node.display = Display::None;
                continue;
            }
            track_node.display = Display::Flex;

            // Explicitly size track to viewport dimensions
            match track.axis {
                ScrollAxis::Vertical => {
                    track_node.height = Val::Px(viewport_size.y);
                }
                ScrollAxis::Horizontal => {
                    track_node.width = Val::Px(viewport_size.x);
                }
            }
        }

        let track_len = viewport_len;
        if track_len < SCROLLBAR_MIN_THUMB {
            continue;
        }

        let thumb_len =
            ((viewport_len / content_len) * track_len).clamp(SCROLLBAR_MIN_THUMB, track_len);
        let thumb_pos = if max_scroll > 0.0 {
            (scroll_val / max_scroll) * (track_len - thumb_len)
        } else {
            0.0
        };

        // Find the thumb child of this track
        if let Ok(track_children) = children_query.get(track_entity) {
            for child in track_children.iter() {
                if let Ok((_, thumb)) = thumb_query.get(child) {
                    if thumb.axis == track.axis {
                        if let Ok(mut thumb_node) = node_query.get_mut(child) {
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
                        }
                        break;
                    }
                }
            }
        }
    }
}

// — Scrollbar interaction observers —

/// When a ScrollbarThumb is added, attach drag and hover observers.
fn attach_scrollbar_drag_handlers(trigger: On<Add, ScrollbarThumb>, mut commands: Commands) {
    commands
        .entity(trigger.entity)
        .observe(on_thumb_drag_start)
        .observe(on_thumb_drag)
        .observe(on_thumb_drag_end)
        .observe(on_thumb_hover_in)
        .observe(on_thumb_hover_out);
}

/// Helper: from a thumb entity, walk up to find the content entity.
/// thumb → parent (track) → parent (viewport) → find ScrollableContent child.
fn find_content_from_thumb(
    thumb_entity: Entity,
    child_of_query: &Query<&ChildOf>,
    children_query: &Query<&Children>,
    content_marker: &Query<Entity, With<ScrollableContent>>,
) -> Option<Entity> {
    let track_entity = child_of_query.get(thumb_entity).ok()?.parent();
    let viewport_entity = child_of_query.get(track_entity).ok()?.parent();
    find_content_sibling(viewport_entity, children_query, content_marker)
}

fn on_thumb_drag_start(
    trigger: On<Pointer<DragStart>>,
    thumb_query: Query<&ScrollbarThumb>,
    child_of_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    content_marker: Query<Entity, With<ScrollableContent>>,
    scroll_query: Query<&UiScrollPosition>,
    mut track_query: Query<(&ScrollbarTrack, &mut ScrollbarVisibility)>,
    mut commands: Commands,
) {
    let thumb_entity = trigger.entity;
    let Ok(thumb) = thumb_query.get(thumb_entity) else {
        return;
    };
    let Some(content_entity) = find_content_from_thumb(
        thumb_entity,
        &child_of_query,
        &children_query,
        &content_marker,
    ) else {
        return;
    };
    let Ok(scroll_pos) = scroll_query.get(content_entity) else {
        return;
    };

    let pointer_pos = trigger.event().pointer_location.position;
    let (start_scroll, start_mouse) = match thumb.axis {
        ScrollAxis::Vertical => (scroll_pos.y, pointer_pos.y),
        ScrollAxis::Horizontal => (scroll_pos.x, pointer_pos.x),
    };

    commands.entity(thumb_entity).insert(ScrollbarDragState {
        start_scroll,
        start_mouse,
    });

    // Set track to drag opacity
    let track_entity = child_of_query.get(thumb_entity).map(|c| c.parent()).ok();
    if let Some(track_entity) = track_entity {
        if let Ok((_, mut vis)) = track_query.get_mut(track_entity) {
            vis.target_opacity = SCROLLBAR_DRAG_ALPHA;
        }
    }
}

fn on_thumb_drag(
    trigger: On<Pointer<Drag>>,
    thumb_query: Query<(&ScrollbarThumb, &ScrollbarDragState)>,
    child_of_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    content_marker: Query<Entity, With<ScrollableContent>>,
    mut scroll_query: Query<
        (&mut UiScrollPosition, &ComputedNode, &ChildOf),
        Without<ScrollbarThumb>,
    >,
    viewport_query: Query<&ComputedNode, Without<UiScrollPosition>>,
) {
    let thumb_entity = trigger.entity;
    let Ok((thumb, drag_state)) = thumb_query.get(thumb_entity) else {
        return;
    };
    let Some(content_entity) = find_content_from_thumb(
        thumb_entity,
        &child_of_query,
        &children_query,
        &content_marker,
    ) else {
        return;
    };
    let Ok((mut scroll_pos, content_node, content_child_of)) = scroll_query.get_mut(content_entity)
    else {
        return;
    };
    let Ok(viewport_node) = viewport_query.get(content_child_of.parent()) else {
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

    let track_len = viewport_len;
    if track_len < SCROLLBAR_MIN_THUMB {
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
    child_of_query: Query<&ChildOf>,
    mut track_query: Query<&mut ScrollbarVisibility, With<ScrollbarTrack>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let thumb_entity = trigger.entity;
    commands.entity(thumb_entity).remove::<ScrollbarDragState>();

    // Walk to parent track and update visibility
    if let Ok(child_of) = child_of_query.get(thumb_entity) {
        let track_entity = child_of.parent();
        if let Ok(mut vis) = track_query.get_mut(track_entity) {
            vis.target_opacity = SCROLLBAR_IDLE_ALPHA;
            vis.last_activity = time.elapsed_secs();
        }
    }
}

fn on_thumb_hover_in(
    trigger: On<Pointer<Over>>,
    child_of_query: Query<&ChildOf>,
    mut track_query: Query<&mut ScrollbarVisibility, With<ScrollbarTrack>>,
) {
    if let Ok(child_of) = child_of_query.get(trigger.entity) {
        if let Ok(mut vis) = track_query.get_mut(child_of.parent()) {
            vis.target_opacity = SCROLLBAR_HOVER_ALPHA;
        }
    }
}

fn on_thumb_hover_out(
    trigger: On<Pointer<Out>>,
    child_of_query: Query<&ChildOf>,
    drag_query: Query<&ScrollbarDragState>,
    mut track_query: Query<&mut ScrollbarVisibility, With<ScrollbarTrack>>,
    time: Res<Time>,
) {
    // Don't reduce opacity if currently dragging
    if drag_query.get(trigger.entity).is_ok() {
        return;
    }

    if let Ok(child_of) = child_of_query.get(trigger.entity) {
        if let Ok(mut vis) = track_query.get_mut(child_of.parent()) {
            vis.target_opacity = SCROLLBAR_IDLE_ALPHA;
            vis.last_activity = time.elapsed_secs();
        }
    }
}

// — Fade animation systems —

/// When scroll position changes, mark sibling tracks as active.
fn update_scrollbar_visibility_on_scroll(
    changed_scroll: Query<(Entity, &ChildOf), (Changed<UiScrollPosition>, With<ScrollableContent>)>,
    children_query: Query<&Children>,
    mut track_query: Query<&mut ScrollbarVisibility, With<ScrollbarTrack>>,
    time: Res<Time>,
) {
    for (_, child_of) in changed_scroll.iter() {
        let viewport_entity = child_of.parent();
        if let Ok(children) = children_query.get(viewport_entity) {
            for child in children.iter() {
                if let Ok(mut vis) = track_query.get_mut(child) {
                    vis.last_activity = time.elapsed_secs();
                    vis.target_opacity = SCROLLBAR_IDLE_ALPHA;
                }
            }
        }
    }
}

/// Lerps opacity toward target; triggers fade-out after delay.
fn tick_scrollbar_fade(
    time: Res<Time>,
    mut query: Query<(Entity, &mut ScrollbarVisibility), With<ScrollbarTrack>>,
    children_query: Query<&Children>,
    drag_query: Query<&ScrollbarDragState>,
) {
    let dt = time.delta_secs();
    let now = time.elapsed_secs();

    for (track_entity, mut vis) in query.iter_mut() {
        // Check if any thumb child is being dragged
        let is_dragging = children_query
            .get(track_entity)
            .map(|children| children.iter().any(|child| drag_query.get(child).is_ok()))
            .unwrap_or(false);

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

/// Applies the track's visibility opacity to child thumb BackgroundColor.
fn apply_scrollbar_opacity(
    track_query: Query<(Entity, &ScrollbarVisibility), With<ScrollbarTrack>>,
    children_query: Query<&Children>,
    mut bg_query: Query<&mut BackgroundColor, With<ScrollbarThumb>>,
) {
    for (track_entity, vis) in track_query.iter() {
        if let Ok(children) = children_query.get(track_entity) {
            for child in children.iter() {
                if let Ok(mut bg) = bg_query.get_mut(child) {
                    bg.0 = Color::srgba(1.0, 1.0, 1.0, vis.opacity);
                }
            }
        }
    }
}

// 5. THE FLUENT API

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
            // Outer Container (The Viewport)
            .style(|n| {
                n.display = Display::Flex;
                n.overflow = Overflow::clip();
            })
            .add(|ui| {
                // Inner Container (The Moving Content)
                ui.ch()
                    .style(|n| {
                        n.flex_shrink = 0.0;
                    })
                    .style(inner_style_fn)
                    .on_spawn_insert(|| (UiScrollPosition::default(), ScrollableContent))
                    .add(content);

                // Vertical scrollbar track + thumb (immediate-mode, recreated each frame)
                ui.ch()
                    .style(|n| {
                        n.position_type = PositionType::Absolute;
                        n.width = Val::Px(SCROLLBAR_SIZE);
                        n.right = Val::Px(0.0);
                        n.top = Val::Px(0.0);
                    })
                    .on_spawn_insert(|| {
                        (
                            ScrollbarTrack {
                                axis: ScrollAxis::Vertical,
                            },
                            ScrollbarVisibility::default(),
                        )
                    })
                    .add(|ui| {
                        ui.ch()
                            .style(|n| {
                                n.position_type = PositionType::Absolute;
                                n.width = Val::Px(SCROLLBAR_SIZE);
                                n.min_height = Val::Px(SCROLLBAR_MIN_THUMB);
                                n.top = Val::Px(0.0);
                                n.left = Val::Px(0.0);
                                n.border_radius = BorderRadius::all(Val::Px(SCROLLBAR_SIZE / 2.0));
                            })
                            .on_spawn_insert(|| {
                                (
                                    ScrollbarThumb {
                                        axis: ScrollAxis::Vertical,
                                    },
                                    BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
                                )
                            });
                    });

                // Horizontal scrollbar track + thumb
                ui.ch()
                    .style(|n| {
                        n.position_type = PositionType::Absolute;
                        n.height = Val::Px(SCROLLBAR_SIZE);
                        n.bottom = Val::Px(0.0);
                        n.left = Val::Px(0.0);
                    })
                    .on_spawn_insert(|| {
                        (
                            ScrollbarTrack {
                                axis: ScrollAxis::Horizontal,
                            },
                            ScrollbarVisibility::default(),
                        )
                    })
                    .add(|ui| {
                        ui.ch()
                            .style(|n| {
                                n.position_type = PositionType::Absolute;
                                n.height = Val::Px(SCROLLBAR_SIZE);
                                n.min_width = Val::Px(SCROLLBAR_MIN_THUMB);
                                n.top = Val::Px(0.0);
                                n.left = Val::Px(0.0);
                                n.border_radius = BorderRadius::all(Val::Px(SCROLLBAR_SIZE / 2.0));
                            })
                            .on_spawn_insert(|| {
                                (
                                    ScrollbarThumb {
                                        axis: ScrollAxis::Horizontal,
                                    },
                                    BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
                                )
                            });
                    });
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
