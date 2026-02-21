use bevy::prelude::*;

// ── Primary ────────────────────────────────────────────────────────────────
pub const PRIMARY_400: Color = Color::srgb(0.3, 0.5, 0.9);
pub const PRIMARY_500: Color = Color::srgb(0.2, 0.4, 0.8);
pub const PRIMARY_600: Color = Color::srgb(0.15, 0.3, 0.7);
pub const PRIMARY_700: Color = Color::srgb(0.1, 0.22, 0.55);

// ── Secondary / Accent ─────────────────────────────────────────────────────
pub const SECONDARY_400: Color = Color::srgb(0.55, 0.4, 0.85);
pub const SECONDARY_500: Color = Color::srgb(0.45, 0.3, 0.75);
pub const SECONDARY_600: Color = Color::srgb(0.35, 0.22, 0.62);

pub const ACCENT_400: Color = Color::srgb(0.0, 0.75, 0.65);
pub const ACCENT_500: Color = Color::srgb(0.0, 0.65, 0.55);
pub const ACCENT_600: Color = Color::srgb(0.0, 0.52, 0.45);

// ── Gray ───────────────────────────────────────────────────────────────────
pub const GRAY_50: Color = Color::srgb(0.96, 0.96, 0.96);
pub const GRAY_100: Color = Color::srgb(0.9, 0.9, 0.9);
pub const GRAY_200: Color = Color::srgb(0.8, 0.8, 0.8);
pub const GRAY_300: Color = Color::srgb(0.7, 0.7, 0.7);
pub const GRAY_400: Color = Color::srgb(0.6, 0.6, 0.6);
pub const GRAY_500: Color = Color::srgb(0.5, 0.5, 0.5);
pub const GRAY_600: Color = Color::srgb(0.35, 0.35, 0.35);
pub const GRAY_700: Color = Color::srgb(0.25, 0.25, 0.25);
pub const GRAY_800: Color = Color::srgb(0.15, 0.15, 0.15);
pub const GRAY_900: Color = Color::srgb(0.1, 0.1, 0.1);
pub const GRAY_950: Color = Color::srgb(0.05, 0.05, 0.05);

// ── Semantic: Status ───────────────────────────────────────────────────────
pub const SUCCESS_400: Color = Color::srgb(0.25, 0.7, 0.35);
pub const SUCCESS_500: Color = Color::srgb(0.2, 0.6, 0.3);
pub const SUCCESS_600: Color = Color::srgb(0.15, 0.55, 0.25);

pub const WARNING_400: Color = Color::srgb(0.95, 0.75, 0.2);
pub const WARNING_500: Color = Color::srgb(0.85, 0.65, 0.15);
pub const WARNING_600: Color = Color::srgb(0.75, 0.55, 0.1);

pub const ERROR_400: Color = Color::srgb(0.85, 0.3, 0.3);
pub const ERROR_500: Color = Color::srgb(0.75, 0.25, 0.25);
pub const ERROR_600: Color = Color::srgb(0.7, 0.2, 0.2);

pub const INFO_400: Color = Color::srgb(0.3, 0.5, 0.8);
pub const INFO_500: Color = Color::srgb(0.25, 0.45, 0.75);
pub const INFO_600: Color = Color::srgb(0.2, 0.4, 0.7);

// ── Semantic: Text ─────────────────────────────────────────────────────────
/// Primary text color — high contrast on dark backgrounds.
pub const TEXT_PRIMARY: Color = Color::WHITE;
/// Secondary text color — slightly dimmed body text.
pub const TEXT_SECONDARY: Color = Color::srgb(0.8, 0.8, 0.8);
/// Muted text color — captions, labels, metadata.
pub const TEXT_MUTED: Color = Color::srgb(0.5, 0.5, 0.5);
/// Disabled text color — de-emphasized, non-interactive.
pub const TEXT_DISABLED: Color = Color::srgb(0.35, 0.35, 0.35);
/// Placeholder text color — input hints, empty states.
pub const TEXT_PLACEHOLDER: Color = Color::srgba(1.0, 1.0, 1.0, 0.35);
/// Inverse text color — text on light backgrounds.
pub const TEXT_INVERSE: Color = Color::srgb(0.1, 0.1, 0.1);

// ── Semantic: Surface / Background ─────────────────────────────────────────
/// App-level background (darkest).
pub const SURFACE_BASE: Color = Color::srgb(0.08, 0.08, 0.08);
/// Raised surface (cards, panels).
pub const SURFACE_RAISED: Color = Color::srgb(0.12, 0.12, 0.12);
/// Overlay background (modals, dropdowns).
pub const SURFACE_OVERLAY: Color = Color::srgb(0.15, 0.15, 0.15);
/// Inset surface (wells, input backgrounds).
pub const SURFACE_INSET: Color = Color::srgb(0.1, 0.1, 0.1);

// ── Semantic: Border ───────────────────────────────────────────────────────
/// Default border color — subtle separation.
pub const BORDER_DEFAULT: Color = Color::srgb(0.25, 0.25, 0.25);
/// Strong border color — emphasized separation.
pub const BORDER_STRONG: Color = Color::srgb(0.35, 0.35, 0.35);
/// Muted border color — very subtle lines.
pub const BORDER_MUTED: Color = Color::srgb(0.18, 0.18, 0.18);

// ── Semantic: Overlay ──────────────────────────────────────────────────────
/// Modal backdrop overlay.
pub const OVERLAY_BACKDROP: Color = Color::srgba(0.0, 0.0, 0.0, 0.5);
/// Subtle overlay (hover tints, light scrims).
pub const OVERLAY_SUBTLE: Color = Color::srgba(0.0, 0.0, 0.0, 0.2);

// ── Accent: Gold ───────────────────────────────────────────────────────────
pub const GOLD_400: Color = Color::srgb(0.85, 0.72, 0.35);
pub const GOLD_500: Color = Color::srgb(0.75, 0.62, 0.25);
pub const GOLD_600: Color = Color::srgb(0.62, 0.52, 0.18);

// ── Utility ────────────────────────────────────────────────────────────────
pub const TRANSPARENT: Color = Color::NONE;
pub const WHITE: Color = Color::WHITE;
pub const BLACK: Color = Color::BLACK;
