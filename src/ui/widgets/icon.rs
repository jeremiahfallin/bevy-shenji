//! Icon widget — renders a Lucide glyph as a `TextEl`.
//!
//! Inserts the existing `crate::theme::widgets::icon::LucideIcon` marker so
//! the theme's `apply_lucide_font` observer assigns the Lucide font. Phase C
//! will move that observer + the marker into `src/ui/` once `src/theme/` is
//! deleted.

use bevy_declarative::prelude::*;

use lucide_icons::Icon;

use crate::theme::widgets::icon::LucideIcon;
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
