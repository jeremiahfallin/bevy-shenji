//! Badge widget — compact pill label.

use bevy_declarative::prelude::*;

use crate::ui::tokens::palette::*;
use crate::ui::tokens::spacing::*;

/// Compact pill label — small accent badge for status/count indicators.
pub fn badge(label: impl Into<String>) -> Div {
    div()
        .flex()
        .row()
        .items_center()
        .justify_center()
        .pad_x(px(SPACE_2))
        .py(px(SPACE_0_5))
        .bg(ACCENT_500)
        .rounded(px(SPACE_2))
        .child(text(label).color(TEXT_PRIMARY))
}
