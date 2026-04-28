//! Button presets — stateless `Div` builders configured via `bevy_declarative`.

use bevy_declarative::prelude::*;

use crate::ui::tokens::palette::*;
use crate::ui::tokens::spacing::*;

/// Filled primary action button — used for the dominant CTA.
pub fn btn_primary(label: impl Into<String>) -> Div {
    div()
        .flex()
        .row()
        .items_center()
        .justify_center()
        .pad_x(px(SPACE_4))
        .py(px(SPACE_2))
        .bg(PRIMARY_500)
        .rounded(px(SPACE_1))
        .child(text(label).color(TEXT_PRIMARY))
}

/// Ghost button — transparent fill, used for secondary actions and cancellations.
pub fn btn_ghost(label: impl Into<String>) -> Div {
    div()
        .flex()
        .row()
        .items_center()
        .justify_center()
        .pad_x(px(SPACE_4))
        .py(px(SPACE_2))
        .rounded(px(SPACE_1))
        .child(text(label).color(TEXT_PRIMARY))
}

/// Danger button — destructive actions (delete, abandon, reset).
pub fn btn_danger(label: impl Into<String>) -> Div {
    div()
        .flex()
        .row()
        .items_center()
        .justify_center()
        .pad_x(px(SPACE_4))
        .py(px(SPACE_2))
        .bg(ERROR_500)
        .rounded(px(SPACE_1))
        .child(text(label).color(TEXT_PRIMARY))
}
