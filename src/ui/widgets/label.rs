//! Label widget — small body-secondary text for form labels and captions.

use bevy_declarative::prelude::*;

use crate::ui::tokens::palette::*;
use crate::ui::tokens::typography::*;

/// Small body-secondary label — used for form labels, captions next to widgets.
pub fn label(s: impl Into<String>) -> TextEl {
    text(s).font_size(TEXT_SM).color(TEXT_SECONDARY)
}
