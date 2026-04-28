//! Tooltip widget — small dark popover containing body text.
//!
//! This widget ships only the visual container. Position and visibility
//! (hover-to-show, etc.) are the call site's responsibility.
//!
//! TODO: hover trigger system — interactive tooltips (Pointer<Over>/Out
//! observers, anchored positioning) are a Phase B follow-up. For now,
//! screens compose their own hover observers around this visual.

use bevy_declarative::prelude::*;

use crate::ui::tokens::palette::*;
use crate::ui::tokens::spacing::*;
use crate::ui::tokens::typography::*;

/// Small dark popover containing a string of body text.
///
/// Position and visibility are the call-site's responsibility — this
/// returns the visual content only.
pub fn tooltip(body: impl Into<String>) -> Div {
    div()
        .pad_x(px(SPACE_2))
        .py(px(SPACE_1))
        .bg(SURFACE_RAISED)
        .rounded(px(SPACE_1))
        .child(text(body).font_size(TEXT_XS).color(TEXT_PRIMARY))
}
