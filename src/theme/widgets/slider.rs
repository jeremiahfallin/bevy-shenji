use bevy::prelude::*;
use bevy::ui_widgets::{SliderRange, SliderThumb, SliderValue};
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::{CapSet, ImmEntity, ImplCap};

use crate::theme::prelude::*;

// -----------------------------------------------------------------------------
// Slider Widget — Styled wrapper around Bevy's headless Slider
// -----------------------------------------------------------------------------
// Bevy 0.17 provides `bevy_ui_widgets::Slider` which handles all input logic
// (drag, click, keyboard, accessibility). This widget provides the visual layer.
//
// Usage:
//   ui.ch_id("volume")
//     .slider(current_value, 0.0, 1.0);
//
//   // With custom styling:
//   ui.ch_id("brightness")
//     .slider(brightness, 0.0, 100.0)
//     .slider_track_color(GRAY_900)
//     .slider_thumb_color(PRIMARY_500);

pub trait ImmUiSlider {
    /// Creates a styled slider. Spawns a Bevy `Slider` component with track + thumb.
    /// `value` is the current value, `min`/`max` define the range.
    fn slider(self, value: f32, min: f32, max: f32) -> Self;

    /// Sets the track background color (default is GRAY_700).
    fn slider_track_color(self, color: impl Into<Color>) -> Self;

    /// Sets the slider height (default is 6px track, 18px thumb).
    fn slider_height(self, track_px: f32) -> Self;
}

impl<Cap> ImmUiSlider for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiTextStyle>
        + ImplCap<CapabilityUiText>
        + CapSet,
{
    fn slider(self, value: f32, min: f32, max: f32) -> Self {
        let range = SliderRange::new(min, max);
        let thumb_pct = range.thumb_position(value) * 100.0;

        self
            // Insert Bevy's headless slider components
            .on_spawn_insert(move || {
                (
                    bevy::ui_widgets::Slider::default(),
                    SliderValue(value),
                    range,
                )
            })
            // Track container styling
            .style(|s| {
                s.width = Val::Percent(100.0);
                s.height = Val::Px(6.0);
                s.flex_shrink = 0.0;
                s.align_items = AlignItems::Center;
                // Relative so thumb can be absolutely positioned
                s.position_type = PositionType::Relative;
            })
            .bg(GRAY_700)
            .rounded(3.0)
            // Thumb
            .add(move |ui| {
                ui.ch_id("thumb")
                    .on_spawn_insert(|| SliderThumb)
                    .style(move |s| {
                        s.width = Val::Px(18.0);
                        s.height = Val::Px(18.0);
                        s.position_type = PositionType::Absolute;
                        // Position thumb based on value percentage
                        // Offset by half thumb width to center it
                        s.left = Val::Percent(thumb_pct);
                        s.top = Val::Px(-6.0); // Center vertically on 6px track
                    })
                    .bg(PRIMARY_500)
                    .rounded(9.0); // Fully round
            })
    }

    fn slider_track_color(self, color: impl Into<Color>) -> Self {
        self.bg(color)
    }

    fn slider_height(self, track_px: f32) -> Self {
        let radius = track_px / 2.0;
        self.style(move |s| {
            s.height = Val::Px(track_px);
        })
        .rounded(radius)
    }
}
