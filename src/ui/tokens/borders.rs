//! Border-width and border-radius design tokens.
//!
//! Ported verbatim from `crate::theme::styles::borders` for the
//! bevy_declarative UI port. Phase C deletes the original.

/// Border width constants for consistent border sizing across the UI.

/// No border — explicit zero.
pub const BORDER_WIDTH_0: f32 = 0.0;

/// Hairline border — 1px, subtle separator lines.
pub const BORDER_WIDTH_DEFAULT: f32 = 1.0;

/// Medium border — 2px, emphasized borders, focus rings.
pub const BORDER_WIDTH_2: f32 = 2.0;

/// Thick border — 3px, strong emphasis, active indicators.
pub const BORDER_WIDTH_3: f32 = 3.0;

/// Extra-thick border — 4px, decorative or tab-active indicators.
pub const BORDER_WIDTH_4: f32 = 4.0;

// ── Border radius constants ────────────────────────────────────────────────

/// No rounding.
pub const RADIUS_NONE: f32 = 0.0;

/// Slight rounding — 2px, barely perceptible.
pub const RADIUS_SM: f32 = 2.0;

/// Default rounding — 4px, standard for small elements.
pub const RADIUS_DEFAULT: f32 = 4.0;

/// Medium rounding — 6px, cards and buttons.
pub const RADIUS_MD: f32 = 6.0;

/// Large rounding — 8px, panels and larger containers.
pub const RADIUS_LG: f32 = 8.0;

/// Extra-large rounding — 12px, modals and prominent cards.
pub const RADIUS_XL: f32 = 12.0;

/// 2XL rounding — 16px, pills and large rounded elements.
pub const RADIUS_2XL: f32 = 16.0;

/// Full rounding — uses 9999px for a pill/circle shape.
pub const RADIUS_FULL: f32 = 9999.0;
