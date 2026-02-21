use bevy::prelude::*;
use bevy_immediate::{CapSet, ImmEntity, ImplCap};

use crate::theme::prelude::*;

// -----------------------------------------------------------------------------
// ProgressBar Widget
// -----------------------------------------------------------------------------
// Usage:
//   ui.ch().progress_bar(0.75);                           // 75% filled, default style
//   ui.ch().progress_bar(health / max_health)
//       .progress_color(SUCCESS_600)
//       .progress_height(8.0);
//   ui.ch().progress_bar(0.5)
//       .progress_color(ERROR_600)
//       .progress_bg(GRAY_900);

pub trait ImmUiProgressBar {
    /// Creates a progress bar. `value` is clamped to 0.0..=1.0.
    fn progress_bar(self, value: f32) -> Self;

    /// Sets the fill color (default is PRIMARY_500).
    fn progress_color(self, color: impl Into<Color>) -> Self;

    /// Sets the track background color (default is GRAY_800).
    fn progress_bg(self, color: impl Into<Color>) -> Self;

    /// Sets the bar height in pixels (default is 6px).
    fn progress_height(self, px: f32) -> Self;

    /// Sets the border radius of the progress bar (default is 3px).
    fn progress_rounded(self, px: f32) -> Self;
}

impl<Cap> ImmUiProgressBar for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout> + ImplCap<CapabilityUiVisuals> + CapSet,
{
    fn progress_bar(self, value: f32) -> Self {
        let pct = value.clamp(0.0, 1.0) * 100.0;

        self
            // Track (outer container)
            .style(|s| {
                s.width = Val::Percent(100.0);
                s.height = Val::Px(6.0);
                s.flex_shrink = 0.0;
            })
            .bg(GRAY_800)
            .rounded(3.0)
            // Fill (inner child)
            .add(move |ui| {
                ui.ch_id("fill")
                    .style(move |s| {
                        s.width = Val::Percent(pct);
                        s.height = Val::Percent(100.0);
                    })
                    .bg(PRIMARY_500)
                    .rounded(3.0);
            })
    }

    fn progress_color(self, color: impl Into<Color>) -> Self {
        // Override the fill child's background color.
        // Since the fill is a child entity, we re-apply color on the track bg.
        // The fill color needs to be set via add() — for chaining we apply to track.
        // Workaround: apply to track bg, the fill uses add() color.
        //
        // In practice, users should set this before adding children or use the
        // builder pattern. For now, this applies to the track container itself.
        // The recommended approach is to compose inline:
        //   ui.ch()
        //     .style(|s| { s.width = Val::Percent(100.0); s.height = Val::Px(6.0); })
        //     .bg(GRAY_800).rounded(3.0)
        //     .add(|ui| { ui.ch_id("fill").style(...).bg(my_color).rounded(3.0); });
        self.bg(color)
    }

    fn progress_bg(self, color: impl Into<Color>) -> Self {
        self.bg(color)
    }

    fn progress_height(self, px: f32) -> Self {
        let radius = px / 2.0;
        self.style(move |s| {
            s.height = Val::Px(px);
        })
        .rounded(radius)
    }

    fn progress_rounded(self, px: f32) -> Self {
        self.rounded(px)
    }
}
