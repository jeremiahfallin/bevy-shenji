# Scrollbar Overlay Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add thin, semi-transparent overlay scrollbars with fade animation and drag interaction to all scrollareas.

**Architecture:** Scrollbar tracks and thumbs are absolute-positioned Bevy UI nodes spawned as siblings of the scroll content inside each scrollarea. Systems compute thumb size/position from `ComputedNode` sizes each frame. Drag uses `Pointer<Drag*>` observers, fade uses a custom `ScrollbarVisibility` component with lerp.

**Tech Stack:** Bevy 0.17 UI nodes, `bevy::picking::events` for Pointer observers, bevy_immediate fluent API.

---

### Task 1: Add Scrollbar Components

**Files:**
- Modify: `src/theme/scroll.rs`

**Step 1: Add the new types at the top of scroll.rs (after existing components)**

```rust
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
const SCROLLBAR_FADE_IN_SPEED: f32 = 6.0;   // ~150ms at 0→1
const SCROLLBAR_FADE_OUT_SPEED: f32 = 3.0;  // ~300ms at 1→0
```

**Step 2: Verify it compiles**

Run: `cargo check`
Expected: compiles with only pre-existing warnings

**Step 3: Commit**

```bash
git add src/theme/scroll.rs
git commit -m "feat: add scrollbar component types and constants"
```

---

### Task 2: Spawn Scrollbar Nodes in scrollarea()

**Files:**
- Modify: `src/theme/scroll.rs` — the `scrollarea()` method

The scrollarea method needs to spawn track + thumb nodes after the inner content. The challenge: we need the inner content's `Entity` to store in the scrollbar components. Use `on_spawn_insert` with a closure that captures the entity.

**Step 1: Modify the `scrollarea()` method to spawn scrollbar tracks and thumbs**

Replace the current `scrollarea` method body. The outer container needs to be relative-positioned so absolute children work. After spawning the inner content, spawn two tracks (vertical, horizontal), each containing a thumb child.

```rust
fn scrollarea(
    self,
    inner_style_fn: impl FnOnce(&mut Node),
    content: impl FnOnce(&mut Imm<'_, '_, Cap>),
) -> Self {
    self
        // Outer Container (The Window)
        .style(|n| {
            n.display = Display::Flex;
            n.position_type = PositionType::Relative;
            n.overflow = Overflow::clip();
        })
        .add(|ui| {
            // Inner Container (The Moving Content)
            let content_entity = ui.ch()
                .style(|n| {
                    n.flex_shrink = 0.0;
                })
                .style(inner_style_fn)
                .on_spawn_insert(|| (UiScrollPosition::default(), ScrollableContent))
                .add(content)
                .id();

            // Vertical scrollbar track
            ui.ch_id("scrollbar_v")
                .style(|n| {
                    n.position_type = PositionType::Absolute;
                    n.right = Val::Px(0.0);
                    n.top = Val::Px(0.0);
                    n.bottom = Val::Px(0.0);
                    n.width = Val::Px(SCROLLBAR_SIZE);
                })
                .on_spawn_insert(move || {
                    (
                        ScrollbarTrack { axis: ScrollAxis::Vertical, content_entity },
                        ScrollbarVisibility::default(),
                        BackgroundColor(Color::NONE),
                    )
                })
                .add(|ui| {
                    // Vertical thumb
                    ui.ch_id("thumb_v")
                        .style(|n| {
                            n.position_type = PositionType::Absolute;
                            n.width = Val::Px(SCROLLBAR_SIZE);
                            n.border_radius = BorderRadius::all(Val::Px(SCROLLBAR_SIZE / 2.0));
                            n.left = Val::Px(0.0);
                            n.top = Val::Px(0.0);
                            n.height = Val::Px(SCROLLBAR_MIN_THUMB);
                        })
                        .on_spawn_insert(move || {
                            (
                                ScrollbarThumb { axis: ScrollAxis::Vertical, content_entity },
                                BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.0)),
                            )
                        });
                });

            // Horizontal scrollbar track
            ui.ch_id("scrollbar_h")
                .style(|n| {
                    n.position_type = PositionType::Absolute;
                    n.bottom = Val::Px(0.0);
                    n.left = Val::Px(0.0);
                    n.right = Val::Px(0.0);
                    n.height = Val::Px(SCROLLBAR_SIZE);
                })
                .on_spawn_insert(move || {
                    (
                        ScrollbarTrack { axis: ScrollAxis::Horizontal, content_entity },
                        ScrollbarVisibility::default(),
                        BackgroundColor(Color::NONE),
                    )
                })
                .add(|ui| {
                    // Horizontal thumb
                    ui.ch_id("thumb_h")
                        .style(|n| {
                            n.position_type = PositionType::Absolute;
                            n.height = Val::Px(SCROLLBAR_SIZE);
                            n.border_radius = BorderRadius::all(Val::Px(SCROLLBAR_SIZE / 2.0));
                            n.top = Val::Px(0.0);
                            n.left = Val::Px(0.0);
                            n.width = Val::Px(SCROLLBAR_MIN_THUMB);
                        })
                        .on_spawn_insert(move || {
                            (
                                ScrollbarThumb { axis: ScrollAxis::Horizontal, content_entity },
                                BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.0)),
                            )
                        });
                });
        })
}
```

Note: `id()` on `ImmEntity` returns the `Entity`. Check if this method exists — if not, use `entity_commands().id()` or store the entity via `on_spawn` callback. The exact API depends on bevy_immediate. If `.id()` isn't available, an alternative is to use an `on_spawn` callback that writes the entity to a local, or restructure to use the `attach_scroll_handlers` observer pattern to spawn scrollbar nodes reactively when `ScrollableContent` is added.

**Step 2: Verify it compiles**

Run: `cargo check`
Expected: compiles (may need API adjustments for entity retrieval)

**Step 3: Commit**

```bash
git add src/theme/scroll.rs
git commit -m "feat: spawn scrollbar track and thumb nodes in scrollarea"
```

---

### Task 3: Add Scrollbar Sizing System

**Files:**
- Modify: `src/theme/scroll.rs`

**Step 1: Write the `update_scrollbar_layout` system**

This system runs each frame and positions/sizes thumbs based on content vs viewport sizes. It also hides tracks when there's no overflow.

```rust
fn update_scrollbar_layout(
    content_query: Query<(&UiScrollPosition, &ComputedNode, &ChildOf)>,
    viewport_query: Query<&ComputedNode>,
    mut track_query: Query<(&ScrollbarTrack, &mut Node, &ComputedNode)>,
    mut thumb_query: Query<(&ScrollbarThumb, &mut Node), Without<ScrollbarTrack>>,
) {
    for (track, mut track_node, track_computed) in track_query.iter_mut() {
        let Ok((scroll_pos, content_node, child_of)) = content_query.get(track.content_entity) else {
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

        let thumb_len = ((viewport_len / content_len) * track_len)
            .clamp(SCROLLBAR_MIN_THUMB, track_len);
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
```

**Step 2: Register the system in `ScrollWidgetPlugin::build`**

```rust
app.add_systems(Update, update_scrollbar_layout.after(update_scroll_layout));
```

**Step 3: Verify it compiles and run the app to see scrollbar thumbs appear**

Run: `cargo check`
Then: `bevy run` — navigate to Characters view, scrollbar thumbs should appear (invisible at first since alpha=0, but the layout is there)

**Step 4: Commit**

```bash
git add src/theme/scroll.rs
git commit -m "feat: add scrollbar sizing system"
```

---

### Task 4: Add Fade Animation System

**Files:**
- Modify: `src/theme/scroll.rs`

**Step 1: Update `on_scroll_event` to notify scrollbar visibility**

When a scroll event fires, find the scrollbar tracks for that content entity and update their `last_activity` + `target_opacity`.

Add to the end of `on_scroll_event` (after updating `scroll.x`/`scroll.y`):

```rust
// Signal scrollbar visibility — done in a separate system that reads UiScrollPosition changes
```

Actually, simpler: add a new system that watches `Changed<UiScrollPosition>` and updates `ScrollbarVisibility` for matching tracks.

```rust
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
```

**Step 2: Write the fade tick system**

```rust
fn tick_scrollbar_fade(
    time: Res<Time>,
    mut query: Query<(&mut ScrollbarVisibility, &mut BackgroundColor, &ScrollbarTrack)>,
    thumb_query: Query<(&ScrollbarThumb, &mut BackgroundColor), Without<ScrollbarTrack>>,
    drag_query: Query<&ScrollbarDragState>,
) {
    let dt = time.delta_secs();
    let now = time.elapsed_secs();

    for (mut vis, mut _track_bg, track) in query.iter_mut() {
        // Don't fade out while dragging
        let any_dragging = thumb_query.iter().any(|(thumb, _)| {
            thumb.content_entity == track.content_entity
                && thumb.axis == track.axis
                && drag_query.contains(/* thumb entity — need to restructure */)
        });

        if !any_dragging && now - vis.last_activity > vis.fade_delay {
            vis.target_opacity = 0.0;
        }

        // Lerp opacity toward target
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
```

**Step 3: Write system to apply visibility opacity to thumb BackgroundColor**

```rust
fn apply_scrollbar_opacity(
    track_query: Query<(&ScrollbarTrack, &ScrollbarVisibility)>,
    mut thumb_query: Query<(&ScrollbarThumb, &mut BackgroundColor)>,
) {
    for (thumb, mut bg) in thumb_query.iter_mut() {
        // Find matching track's visibility
        for (track, vis) in track_query.iter() {
            if track.content_entity == thumb.content_entity && track.axis == thumb.axis {
                bg.0 = Color::srgba(0.5, 0.5, 0.5, vis.opacity);
                break;
            }
        }
    }
}
```

**Step 4: Register all three systems**

```rust
app.add_systems(Update, (
    update_scrollbar_visibility_on_scroll,
    tick_scrollbar_fade,
    apply_scrollbar_opacity,
).chain().after(update_scrollbar_layout));
```

**Step 5: Verify it compiles and test visually**

Run: `cargo check` then `bevy run`
Expected: scrollbar thumbs appear when scrolling, fade out after 1.5s

**Step 6: Commit**

```bash
git add src/theme/scroll.rs
git commit -m "feat: add scrollbar fade animation"
```

---

### Task 5: Add Thumb Drag Interaction

**Files:**
- Modify: `src/theme/scroll.rs`

**Step 1: Add drag observers to thumbs**

Add a new observer that triggers when `ScrollbarThumb` is added, attaching drag handlers:

```rust
fn attach_scrollbar_drag_handlers(
    trigger: On<Add, ScrollbarThumb>,
    mut commands: Commands,
) {
    commands.entity(trigger.entity)
        .observe(on_thumb_drag_start)
        .observe(on_thumb_drag)
        .observe(on_thumb_drag_end);
}

fn on_thumb_drag_start(
    trigger: On<Pointer<bevy::picking::events::DragStart>>,
    mut commands: Commands,
    thumb_query: Query<&ScrollbarThumb>,
    scroll_query: Query<&UiScrollPosition>,
) {
    let entity = trigger.entity;
    let Ok(thumb) = thumb_query.get(entity) else { return };
    let Ok(scroll) = scroll_query.get(thumb.content_entity) else { return };

    let start_scroll = match thumb.axis {
        ScrollAxis::Vertical => scroll.y,
        ScrollAxis::Horizontal => scroll.x,
    };
    let start_mouse = match thumb.axis {
        ScrollAxis::Vertical => trigger.event().pointer_location.position.y,
        ScrollAxis::Horizontal => trigger.event().pointer_location.position.x,
    };

    commands.entity(entity).insert(ScrollbarDragState {
        start_scroll,
        start_mouse,
    });
}

fn on_thumb_drag(
    trigger: On<Pointer<bevy::picking::events::Drag>>,
    thumb_query: Query<(&ScrollbarThumb, &ScrollbarDragState)>,
    mut scroll_query: Query<(&mut UiScrollPosition, &ComputedNode, &ChildOf)>,
    viewport_query: Query<&ComputedNode>,
    track_query: Query<(&ScrollbarTrack, &ComputedNode)>,
) {
    let Ok((thumb, drag)) = thumb_query.get(trigger.entity) else { return };
    let Ok((mut scroll, content_node, child_of)) = scroll_query.get_mut(thumb.content_entity) else { return };
    let Ok(viewport_node) = viewport_query.get(child_of.parent()) else { return };

    let content_size = content_node.size();
    let viewport_size = viewport_node.size();

    let (content_len, viewport_len) = match thumb.axis {
        ScrollAxis::Vertical => (content_size.y, viewport_size.y),
        ScrollAxis::Horizontal => (content_size.x, viewport_size.x),
    };
    let max_scroll = (content_len - viewport_len).max(0.0);
    if max_scroll <= 0.0 { return; }

    // Find track length
    let track_len = track_query.iter()
        .find(|(t, _)| t.content_entity == thumb.content_entity && t.axis == thumb.axis)
        .map(|(_, cn)| match thumb.axis {
            ScrollAxis::Vertical => cn.size().y,
            ScrollAxis::Horizontal => cn.size().x,
        })
        .unwrap_or(1.0);

    let thumb_len = ((viewport_len / content_len) * track_len)
        .clamp(SCROLLBAR_MIN_THUMB, track_len);
    let scrollable_track = track_len - thumb_len;
    if scrollable_track <= 0.0 { return; }

    let mouse_pos = match thumb.axis {
        ScrollAxis::Vertical => trigger.event().pointer_location.position.y,
        ScrollAxis::Horizontal => trigger.event().pointer_location.position.x,
    };
    let mouse_delta = mouse_pos - drag.start_mouse;
    let scroll_delta = mouse_delta * (max_scroll / scrollable_track);

    let new_scroll = (drag.start_scroll + scroll_delta).clamp(0.0, max_scroll);
    match thumb.axis {
        ScrollAxis::Vertical => scroll.y = new_scroll,
        ScrollAxis::Horizontal => scroll.x = new_scroll,
    }
}

fn on_thumb_drag_end(
    trigger: On<Pointer<bevy::picking::events::DragEnd>>,
    mut commands: Commands,
) {
    commands.entity(trigger.entity).remove::<ScrollbarDragState>();
}
```

**Step 2: Register the observer in plugin build**

```rust
app.add_observer(attach_scrollbar_drag_handlers);
```

**Step 3: Verify it compiles**

Run: `cargo check`
Expected: compiles. Note: `Pointer<DragStart>` may have a different event structure in Bevy 0.17 — check `bevy::picking::events` for exact types and field names (`pointer_location.position` vs `pointer_location` vs direct fields). Adjust accordingly.

**Step 4: Test visually**

Run: `bevy run`
Expected: dragging a scrollbar thumb scrolls the content

**Step 5: Commit**

```bash
git add src/theme/scroll.rs
git commit -m "feat: add scrollbar thumb drag interaction"
```

---

### Task 6: Add Click-on-Track to Jump

**Files:**
- Modify: `src/theme/scroll.rs`

**Step 1: Add click observer to tracks**

Attach a `Pointer<Click>` observer when `ScrollbarTrack` is added:

```rust
fn attach_scrollbar_track_click(
    trigger: On<Add, ScrollbarTrack>,
    mut commands: Commands,
) {
    commands.entity(trigger.entity).observe(on_track_click);
}

fn on_track_click(
    trigger: On<Pointer<Click>>,
    track_query: Query<(&ScrollbarTrack, &ComputedNode, &GlobalTransform)>,
    mut scroll_query: Query<(&mut UiScrollPosition, &ComputedNode, &ChildOf)>,
    viewport_query: Query<&ComputedNode>,
) {
    let Ok((track, track_computed, track_transform)) = track_query.get(trigger.entity) else { return };
    let Ok((mut scroll, content_node, child_of)) = scroll_query.get_mut(track.content_entity) else { return };
    let Ok(viewport_node) = viewport_query.get(child_of.parent()) else { return };

    let content_size = content_node.size();
    let viewport_size = viewport_node.size();

    let (content_len, viewport_len) = match track.axis {
        ScrollAxis::Vertical => (content_size.y, viewport_size.y),
        ScrollAxis::Horizontal => (content_size.x, viewport_size.x),
    };
    let max_scroll = (content_len - viewport_len).max(0.0);
    if max_scroll <= 0.0 { return; }

    let track_len = match track.axis {
        ScrollAxis::Vertical => track_computed.size().y,
        ScrollAxis::Horizontal => track_computed.size().x,
    };

    // Compute click position relative to track
    let track_origin = track_transform.translation().truncate();
    let click_pos = trigger.event().pointer_location.position;
    let relative = match track.axis {
        ScrollAxis::Vertical => click_pos.y - track_origin.y,
        ScrollAxis::Horizontal => click_pos.x - track_origin.x,
    };

    // Jump so thumb centers on click
    let ratio = (relative / track_len).clamp(0.0, 1.0);
    let new_scroll = (ratio * max_scroll).clamp(0.0, max_scroll);
    match track.axis {
        ScrollAxis::Vertical => scroll.y = new_scroll,
        ScrollAxis::Horizontal => scroll.x = new_scroll,
    }
}
```

**Step 2: Register observer**

```rust
app.add_observer(attach_scrollbar_track_click);
```

**Step 3: Verify and test**

Run: `cargo check` then `bevy run`
Expected: clicking on the track area jumps the scroll position

**Step 4: Commit**

```bash
git add src/theme/scroll.rs
git commit -m "feat: add click-on-track scroll jump"
```

---

### Task 7: Add Hover State for Thumb

**Files:**
- Modify: `src/theme/scroll.rs`

**Step 1: Add hover observers to thumb to change opacity state**

Extend `attach_scrollbar_drag_handlers` (or add a new observer) to also observe `Pointer<Over>` and `Pointer<Out>` on thumbs:

```rust
// In attach_scrollbar_drag_handlers, also add:
commands.entity(trigger.entity)
    .observe(on_thumb_hover_in)
    .observe(on_thumb_hover_out);

fn on_thumb_hover_in(
    trigger: On<Pointer<Over>>,
    thumb_query: Query<&ScrollbarThumb>,
    mut track_query: Query<(&ScrollbarTrack, &mut ScrollbarVisibility)>,
) {
    let Ok(thumb) = thumb_query.get(trigger.entity) else { return };
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
    mut track_query: Query<(&ScrollbarTrack, &mut ScrollbarVisibility)>,
    drag_query: Query<&ScrollbarDragState>,
    time: Res<Time>,
) {
    let Ok(thumb) = thumb_query.get(trigger.entity) else { return };
    // Don't reduce if dragging
    if drag_query.contains(trigger.entity) { return; }
    for (track, mut vis) in track_query.iter_mut() {
        if track.content_entity == thumb.content_entity && track.axis == thumb.axis {
            vis.target_opacity = SCROLLBAR_IDLE_ALPHA;
            vis.last_activity = time.elapsed_secs();
            break;
        }
    }
}
```

Also update `on_thumb_drag_start` to set target opacity to `SCROLLBAR_DRAG_ALPHA`, and `on_thumb_drag_end` to reset to `SCROLLBAR_IDLE_ALPHA`.

**Step 2: Verify and test**

Run: `cargo check` then `bevy run`
Expected: thumb brightens on hover, brightens more while dragging

**Step 3: Commit**

```bash
git add src/theme/scroll.rs
git commit -m "feat: add scrollbar hover and drag visual states"
```

---

### Task 8: Visual Verification & Cleanup

**Files:**
- Modify: `src/theme/scroll.rs` (if cleanup needed)

**Step 1: Run the app and verify all scrollareas**

Run: `bevy run`
Test all 5 scrollarea usages:
- Characters view (horizontal + vertical)
- Squads view (2 scrollareas)
- Inspector panel
- New Game screen

Check:
- Scrollbars appear on scroll, fade after 1.5s
- Thumb drag works in both axes
- Click-on-track jumps work
- No visual glitches when no overflow exists
- Shift+scroll still works for horizontal

**Step 2: Take screenshots via BRP for verification**

Use bevy-eyes-on skill to capture Characters view showing scrollbar.

**Step 3: Final commit if any cleanup was needed**

```bash
git add src/theme/scroll.rs
git commit -m "chore: scrollbar cleanup and polish"
```
