# Scrollbar Overlay Design

Thin, semi-transparent scrollbar overlays for all scrollareas. Appear on hover/scroll, fade out after inactivity. Draggable thumbs for direct manipulation.

## Layout

Scrollbars are absolute-positioned sibling nodes inside the scrollarea outer container, alongside the existing inner content.

```
Outer (Flex, overflow: clip)
  Inner Content (flex_shrink: 0, ScrollableContent)
  Vertical Track (position: absolute, right: 0, top: 0, bottom: 0, width: 6px)
    Thumb (position: absolute, sized proportionally)
  Horizontal Track (position: absolute, bottom: 0, left: 0, right: 0, height: 6px)
    Thumb (position: absolute, sized proportionally)
```

Tracks and thumbs are spawned automatically inside `scrollarea()`. No per-view changes needed.

## Components

- `ScrollbarTrack { axis: Axis, content_entity: Entity }` — marker on track nodes
- `ScrollbarThumb { axis: Axis, content_entity: Entity }` — marker on thumb nodes
- `ScrollbarVisibility { opacity, target_opacity, last_activity, fade_delay }` — on each track, drives fade animation
- `ScrollbarDragState { start_scroll: f32, start_mouse: f32 }` — on thumb during drag

`Axis` is a simple enum: `Horizontal | Vertical`.

## Sizing (computed each frame)

- Thumb length = `(viewport_size / content_size) * track_length`, clamped to minimum 20px
- Thumb position = `(scroll_position / max_scroll) * (track_length - thumb_length)`
- Track hidden (`Display::None`) when no overflow exists in that axis

## Visual Style

- Track background: transparent
- Thumb idle: `GRAY_500` at 50% opacity
- Thumb hovered: `GRAY_400` at 70% opacity
- Thumb dragging: `GRAY_300` at 90% opacity
- Thumb border radius: 3px (fully rounded for 6px bar)

## Fade Animation

- Start hidden (0% opacity)
- On scroll event or mouse hover over scrollarea: fade to idle opacity over ~150ms
- After 1.5s of no activity and no hover: fade out over ~300ms
- While dragging: stays visible, no fade timeout
- System lerps `opacity` toward `target_opacity` each frame

## Drag Interaction

1. `DragStart` on thumb: record initial scroll position and mouse position in `ScrollbarDragState`
2. `Drag`: compute pixel delta along track axis, convert to scroll units via `pixel_delta * (max_scroll / (track_length - thumb_length))`, update `UiScrollPosition`
3. `DragEnd`: clear drag state

Click-on-track: `Click` observer on track jumps scroll so thumb centers on click point.

## Edge Cases

- No overflow in axis: track + thumb set to `Display::None`
- Both axes overflow: tracks overlap in 6x6px bottom-right corner (acceptable at this scale)
- Window resize / content size change: systems recompute from `ComputedNode` each frame, auto-adapts

## Files Modified

- `src/theme/scroll.rs` — all new components, systems, and scrollarea spawning changes
- `src/theme/styles/palette.rs` — scrollbar color constants (if needed beyond existing GRAY_*)
