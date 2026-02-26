use super::palette::*;
use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::{CapSet, ImmEntity, ImplCap};

// ── Shared button base ─────────────────────────────────────────────────────

/// Common button layout shared by all variants.
fn style_btn_base<'w, 's, 'a, Cap>(entity: ImmEntity<'w, 's, 'a, Cap>) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiTextStyle>,
{
    entity
        .h(Val::Px(40.0))
        .px(Val::Px(SPACE_4))
        .rounded(6.0)
        .flex_row()
        .items_center()
        .justify_center()
        .text_sm()
        .font_bold()
}

// ── Primary (filled) ───────────────────────────────────────────────────────

/// Primary button — solid background, white text.
pub fn style_btn_primary<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiTextStyle>,
{
    style_btn_base(entity)
        .bg(PRIMARY_500)
        .text_color(TEXT_PRIMARY)
}

/// Primary button hover state.
pub fn style_btn_primary_hover<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity.bg(PRIMARY_600)
}

// ── Secondary (soft fill) ──────────────────────────────────────────────────

/// Secondary button — subtle background, muted text.
pub fn style_btn_secondary<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiTextStyle>,
{
    style_btn_base(entity).bg(GRAY_700).text_color(GRAY_200)
}

/// Secondary button hover state.
pub fn style_btn_secondary_hover<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity.bg(GRAY_600)
}

// ── Ghost (transparent) ────────────────────────────────────────────────────

/// Ghost button — transparent background, text-only appearance.
pub fn style_btn_ghost<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiTextStyle>,
{
    style_btn_base(entity).bg(TRANSPARENT).text_color(GRAY_200)
}

/// Ghost button hover state.
pub fn style_btn_ghost_hover<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity.bg(GRAY_700)
}

// ── Outline ────────────────────────────────────────────────────────────────

/// Outline button — bordered, transparent fill.
pub fn style_btn_outline<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiTextStyle>,
{
    style_btn_base(entity)
        .bg(TRANSPARENT)
        .border(BORDER_WIDTH_DEFAULT)
        .border_color(BORDER_DEFAULT)
        .text_color(GRAY_200)
}

/// Outline button hover state.
pub fn style_btn_outline_hover<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity.bg(GRAY_800).border_color(BORDER_STRONG)
}

// ── Danger ─────────────────────────────────────────────────────────────────

/// Danger button — destructive action, red fill.
pub fn style_btn_danger<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiTextStyle>,
{
    style_btn_base(entity)
        .bg(ERROR_600)
        .text_color(TEXT_PRIMARY)
}

/// Danger button hover state.
pub fn style_btn_danger_hover<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity.bg(ERROR_500)
}

// ── Size variants ──────────────────────────────────────────────────────────

/// Small button — 32px height, tighter padding.
pub fn style_btn_sm<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiTextStyle>,
{
    entity.h(Val::Px(32.0)).px(Val::Px(SPACE_3)).text_xs()
}

/// Large button — 48px height, wider padding.
pub fn style_btn_lg<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiTextStyle>,
{
    entity.h(Val::Px(48.0)).px(Val::Px(SPACE_6)).text_base()
}

// ── Disabled state (applies on top of any variant) ─────────────────────────

/// Disabled state — reduces contrast, apply after any variant style.
pub fn style_btn_disabled<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiTextStyle>,
{
    entity.bg(GRAY_800).text_color(TEXT_DISABLED)
}
