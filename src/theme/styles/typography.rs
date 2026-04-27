// ── Font size scale ─────────────────────────────────────────────────────────
pub const TEXT_XS: f32 = 12.0;
pub const TEXT_SM: f32 = 14.0;
pub const TEXT_BASE: f32 = 16.0;
pub const TEXT_LG: f32 = 18.0;
pub const TEXT_XL: f32 = 20.0;
pub const TEXT_2XL: f32 = 24.0;
pub const TEXT_3XL: f32 = 30.0;
pub const TEXT_4XL: f32 = 48.0;
pub const TEXT_5XL: f32 = 64.0;

// ── Line height scale ──────────────────────────────────────────────────────
/// Tight line height — headings, compact UI.
pub const LEADING_TIGHT: f32 = 1.25;
/// Normal line height — body text.
pub const LEADING_NORMAL: f32 = 1.5;
/// Relaxed line height — long-form text.
pub const LEADING_RELAXED: f32 = 1.75;

use bevy::text::LineHeight;

/// Default line height for body and heading text — matches CSS `line-height: 1.5`
/// semantics by multiplying the per-glyph font size. Use this on every Text spawn
/// in Bevy 0.18+ where a `LineHeight` component is required.
pub const LINE_HEIGHT_NORMAL: LineHeight = LineHeight::RelativeToFont(LEADING_NORMAL);

// ── Typography preset functions ────────────────────────────────────────────
//
// These are style functions matching the pattern in buttons.rs / containers.rs.
// They apply a combination of font size, weight, and color to an entity.

use super::palette::*;
use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::{CapSet, ImmEntity, ImplCap};

/// Heading 1 — large page/section title.
pub fn style_heading_1<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiTextStyle> + ImplCap<CapabilityUiLayout>,
{
    entity
        .text_size(TEXT_3XL)
        .font_bold()
        .text_color(TEXT_PRIMARY)
}

/// Heading 2 — subsection title.
pub fn style_heading_2<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiTextStyle> + ImplCap<CapabilityUiLayout>,
{
    entity.text_2xl().font_bold().text_color(TEXT_PRIMARY)
}

/// Heading 3 — card/panel title.
pub fn style_heading_3<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiTextStyle> + ImplCap<CapabilityUiLayout>,
{
    entity.text_xl().font_bold().text_color(TEXT_PRIMARY)
}

/// Body text — default readable text.
pub fn style_body<'w, 's, 'a, Cap>(entity: ImmEntity<'w, 's, 'a, Cap>) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiTextStyle> + ImplCap<CapabilityUiLayout>,
{
    entity.text_base().text_color(TEXT_SECONDARY)
}

/// Body small — compact body text.
pub fn style_body_sm<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiTextStyle> + ImplCap<CapabilityUiLayout>,
{
    entity.text_sm().text_color(TEXT_SECONDARY)
}

/// Caption — small, muted annotation text.
pub fn style_caption<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiTextStyle> + ImplCap<CapabilityUiLayout>,
{
    entity.text_xs().text_color(TEXT_MUTED)
}

/// Overline — uppercase label above content.
pub fn style_overline<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiTextStyle> + ImplCap<CapabilityUiLayout>,
{
    entity.text_xs().font_bold().text_color(TEXT_MUTED)
}

/// Code / monospaced — for inline code or data values.
pub fn style_code<'w, 's, 'a, Cap>(entity: ImmEntity<'w, 's, 'a, Cap>) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiTextStyle> + ImplCap<CapabilityUiLayout>,
{
    entity.text_sm().text_color(TEXT_SECONDARY)
}
