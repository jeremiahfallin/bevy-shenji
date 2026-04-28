//! Progress bar widget — horizontal fill bar with clamped fraction.

use bevy_declarative::prelude::*;

use crate::ui::tokens::palette::*;
use crate::ui::tokens::spacing::*;

/// Horizontal progress bar. `fraction` is clamped to `[0.0, 1.0]`.
pub fn progress_bar(fraction: f32) -> Div {
    let pct_filled = fraction.clamp(0.0, 1.0) * 100.0;
    div()
        .w_full()
        .h(px(SPACE_2))
        .bg(SURFACE_INSET)
        .rounded(px(SPACE_0_5))
        .child(
            div()
                .w(pct(pct_filled))
                .h_full()
                .bg(PRIMARY_500)
                .rounded(px(SPACE_0_5)),
        )
}
