//! Icon widget — renders a Lucide glyph as a `TextEl`.
//!
//! The caller is responsible for assigning the Lucide font (e.g. via a
//! global `IconFont` resource or a per-entity `TextFont`); this widget
//! only emits the glyph string.

use bevy_declarative::prelude::*;

use lucide_icons::Icon;

use crate::ui::tokens::palette::*;
use crate::ui::tokens::typography::*;

/// Render a Lucide icon as a `TextEl` glyph.
///
/// Uses `char::from(Icon)` (the conversion provided by `lucide-icons`) to
/// produce the single-glyph string. Caller must assign the Lucide font.
pub fn icon(name: Icon) -> TextEl {
    let glyph = char::from(name).to_string();
    text(glyph).font_size(TEXT_BASE).color(TEXT_PRIMARY)
}
