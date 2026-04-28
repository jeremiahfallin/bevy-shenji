//! Container presets — `Div` builders for panels, cards, and surfaces.

use bevy_declarative::prelude::*;

use crate::ui::tokens::palette::*;
use crate::ui::tokens::spacing::*;

/// Raised panel — primary container for grouped content (sidebars, sections).
pub fn panel() -> Div {
    div()
        .flex()
        .col()
        .p(px(SPACE_4))
        .bg(SURFACE_RAISED)
        .rounded(px(SPACE_2))
}

/// Card — compact raised container for individual items in lists/grids.
pub fn card() -> Div {
    div()
        .flex()
        .col()
        .p(px(SPACE_3))
        .bg(SURFACE_RAISED)
        .rounded(px(SPACE_1))
}

/// Surface — top-level page background container.
pub fn surface() -> Div {
    div().flex().col().p(px(SPACE_5)).bg(SURFACE_BASE)
}
