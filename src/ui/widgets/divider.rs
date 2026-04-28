//! Divider widget — hairline separator (horizontal or vertical).

use bevy_declarative::prelude::*;

use crate::ui::tokens::palette::*;

/// Horizontal hairline separator — full width, 1px tall.
pub fn divider() -> Div {
    div().w_full().h(px(1.0)).bg(BORDER_DEFAULT)
}

/// Vertical hairline separator — full height, 1px wide.
pub fn divider_vertical() -> Div {
    div().w(px(1.0)).h_full().bg(BORDER_DEFAULT)
}
