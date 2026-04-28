//! Typography presets — `TextEl` builders for the heading/body scale.

use bevy_declarative::prelude::*;

use crate::ui::tokens::palette::*;
use crate::ui::tokens::typography::*;

pub fn heading_1(s: impl Into<String>) -> TextEl {
    text(s).font_size(TEXT_3XL).color(TEXT_PRIMARY)
}

pub fn heading_2(s: impl Into<String>) -> TextEl {
    text(s).font_size(TEXT_2XL).color(TEXT_PRIMARY)
}

pub fn heading_3(s: impl Into<String>) -> TextEl {
    text(s).font_size(TEXT_XL).color(TEXT_PRIMARY)
}

pub fn body(s: impl Into<String>) -> TextEl {
    text(s).font_size(TEXT_BASE).color(TEXT_SECONDARY)
}

pub fn body_sm(s: impl Into<String>) -> TextEl {
    text(s).font_size(TEXT_SM).color(TEXT_SECONDARY)
}

pub fn caption(s: impl Into<String>) -> TextEl {
    text(s).font_size(TEXT_XS).color(TEXT_MUTED)
}

pub fn overline(s: impl Into<String>) -> TextEl {
    text(s).font_size(TEXT_XS).color(TEXT_MUTED)
}
