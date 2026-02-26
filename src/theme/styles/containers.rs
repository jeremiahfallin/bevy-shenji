use super::borders::*;
use super::palette::*;
use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::{CapSet, ImmEntity, ImplCap};

// ── Layout containers ──────────────────────────────────────────────────────

/// Central content panel — full-size, dark background.
pub fn style_panel_central<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity.flex_col().w_full().h_full().bg(GRAY_900)
}

/// Sidebar panel — fixed-width, slightly lighter background.
pub fn style_sidebar<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity.w(Val::Px(250.0)).h_full().bg(GRAY_800)
}

/// Bottom bar — fixed-height, medium background.
pub fn style_bottom_bar<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity.w_full().h(Val::Px(50.0)).bg(GRAY_700)
}

// ── Card variants ──────────────────────────────────────────────────────────

/// Standard card — raised surface with padding and rounding.
pub fn style_card<'w, 's, 'a, Cap>(entity: ImmEntity<'w, 's, 'a, Cap>) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity
        .flex_col()
        .w(Val::Percent(50.0))
        .p(Val::Px(SPACE_2_5))
        .rounded(RADIUS_LG)
        .bg(GRAY_700)
}

/// Elevated card — card with a subtle shadow for depth.
pub fn style_card_elevated<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity
        .flex_col()
        .p(Val::Px(SPACE_4))
        .rounded(RADIUS_LG)
        .bg(SURFACE_RAISED)
        .shadow_md()
}

/// Outlined card — bordered, transparent fill.
pub fn style_card_outlined<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity
        .flex_col()
        .p(Val::Px(SPACE_4))
        .rounded(RADIUS_LG)
        .bg(TRANSPARENT)
        .border(BORDER_WIDTH_DEFAULT)
        .border_color(BORDER_DEFAULT)
}

// ── Overlay containers ─────────────────────────────────────────────────────

/// Modal overlay backdrop — fullscreen semi-transparent cover.
pub fn style_modal_overlay<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity
        .absolute()
        .inset_0()
        .bg(OVERLAY_BACKDROP)
        .flex_col()
        .items_center()
        .justify_center()
        .z_index(100)
}

/// Modal dialog body — centered elevated surface.
pub fn style_modal_dialog<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity
        .flex_col()
        .p(Val::Px(SPACE_6))
        .rounded(RADIUS_XL)
        .bg(SURFACE_OVERLAY)
        .shadow_lg()
        .min_w(Val::Px(320.0))
        .max_w(Val::Px(560.0))
}

/// Tooltip container — small floating surface near a target.
pub fn style_tooltip<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity
        .absolute()
        .p(Val::Px(SPACE_2))
        .rounded(RADIUS_DEFAULT)
        .bg(GRAY_800)
        .shadow_sm()
        .z_index(200)
}

/// Toast / notification container — fixed-width notice card.
pub fn style_toast<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity
        .flex_row()
        .items_center()
        .p(Val::Px(SPACE_4))
        .rounded(RADIUS_LG)
        .bg(SURFACE_RAISED)
        .shadow_md()
        .min_w(Val::Px(280.0))
        .max_w(Val::Px(420.0))
        .border(BORDER_WIDTH_DEFAULT)
        .border_color(BORDER_MUTED)
}

// ── Well / inset containers ────────────────────────────────────────────────

/// Inset well — for grouped form fields or embedded content.
pub fn style_well<'w, 's, 'a, Cap>(entity: ImmEntity<'w, 's, 'a, Cap>) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity
        .flex_col()
        .p(Val::Px(SPACE_4))
        .rounded(RADIUS_MD)
        .bg(SURFACE_INSET)
}
