//! Icon widget — renders a Lucide glyph as a `TextEl`.
//!
//! Inserts the [`LucideIcon`](crate::ui::lucide::LucideIcon) marker so the
//! `apply_lucide_font` observer assigns the Lucide font.

use bevy_declarative::prelude::*;

use lucide_icons::Icon;

use crate::ui::lucide::LucideIcon;
use crate::ui::tokens::palette::*;
use crate::ui::tokens::typography::*;

/// Render a Lucide icon as a `TextEl` glyph.
///
/// Uses `char::from(Icon)` (the conversion provided by `lucide-icons`) to
/// produce the glyph string. Inserts `LucideIcon` so the existing observer
/// rewrites the entity's `TextFont` to the Lucide font asset.
pub fn icon(name: Icon) -> TextEl {
    let glyph = char::from(name).to_string();
    text(glyph)
        .font_size(TEXT_BASE)
        .color(TEXT_PRIMARY)
        .insert(LucideIcon)
}
