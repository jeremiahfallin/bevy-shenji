//! Font-size and line-height scales. Preset functions live in
//! `crate::ui::presets::text`.

pub const TEXT_XS: f32 = 12.0;
pub const TEXT_SM: f32 = 14.0;
pub const TEXT_BASE: f32 = 16.0;
pub const TEXT_LG: f32 = 18.0;
pub const TEXT_XL: f32 = 20.0;
pub const TEXT_2XL: f32 = 24.0;
pub const TEXT_3XL: f32 = 30.0;
pub const TEXT_4XL: f32 = 48.0;
pub const TEXT_5XL: f32 = 64.0;

pub const LEADING_TIGHT: f32 = 1.25;
pub const LEADING_NORMAL: f32 = 1.5;
pub const LEADING_RELAXED: f32 = 1.75;

use bevy::text::LineHeight;

/// Default line height for body and heading text — matches CSS `line-height: 1.5`
/// semantics by multiplying the per-glyph font size. Use this on every Text spawn
/// in Bevy 0.18+ where a `LineHeight` component is required.
pub const LINE_HEIGHT_NORMAL: LineHeight = LineHeight::RelativeToFont(LEADING_NORMAL);
