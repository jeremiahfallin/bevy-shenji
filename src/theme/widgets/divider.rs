use bevy::prelude::*;
use bevy_immediate::{CapSet, ImmEntity, ImplCap};

use crate::theme::primitives::style::{CapabilityUiLayout, ImmUiStyleExt};
use crate::theme::primitives::visuals::{CapabilityUiVisuals, ImmUiVisuals};
use crate::theme::styles::palette::*;

// -----------------------------------------------------------------------------
// Divider / Separator Widget
// -----------------------------------------------------------------------------
// Usage:
//   ui.ch().divider();                  // horizontal, default color
//   ui.ch().divider_vertical();         // vertical
//   ui.ch().divider().divider_color(ERROR_600);  // custom color

pub trait ImmUiDivider {
    /// Creates a horizontal divider (full width, 1px height).
    fn divider(self) -> Self;

    /// Creates a vertical divider (full height, 1px width).
    fn divider_vertical(self) -> Self;

    /// Override the divider color (default is GRAY_700).
    fn divider_color(self, color: impl Into<Color>) -> Self;

    /// Override the divider thickness (default is 1px).
    fn divider_thickness(self, px: f32) -> Self;
}

impl<Cap> ImmUiDivider for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout> + ImplCap<CapabilityUiVisuals> + CapSet,
{
    fn divider(self) -> Self {
        self.style(|s| {
            s.width = Val::Percent(100.0);
            s.height = Val::Px(1.0);
            s.flex_shrink = 0.0;
        })
        .bg(GRAY_700)
    }

    fn divider_vertical(self) -> Self {
        self.style(|s| {
            s.width = Val::Px(1.0);
            s.height = Val::Percent(100.0);
            s.flex_shrink = 0.0;
        })
        .bg(GRAY_700)
    }

    fn divider_color(self, color: impl Into<Color>) -> Self {
        self.bg(color)
    }

    fn divider_thickness(self, px: f32) -> Self {
        self.style(move |s| {
            // Detect orientation: if width is 100% it's horizontal, set height.
            // If height is 100% it's vertical, set width.
            // Fallback: set height (assume horizontal).
            match s.width {
                Val::Percent(p) if p >= 99.0 => s.height = Val::Px(px),
                _ => match s.height {
                    Val::Percent(p) if p >= 99.0 => s.width = Val::Px(px),
                    _ => s.height = Val::Px(px),
                },
            }
        })
    }
}
