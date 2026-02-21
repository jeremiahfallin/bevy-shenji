//! Generic context menu infrastructure.
//!
//! Provides a reusable state resource and overlay positioning helpers.
//! Games define their own `ContextMenuType` enum and rendering logic.

use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::{CapSet, Imm, ImmEntity, ImplCap};

/// Generic context menu state.
///
/// Games should newtype or extend this for their specific needs:
/// ```rust,ignore
/// #[derive(Resource, Default)]
/// pub struct MyContextMenuState {
///     pub base: ContextMenuBase,
///     pub context_type: MyContextType,
///     pub target: Option<Entity>,
/// }
/// ```
#[derive(Default, Clone, Debug)]
pub struct ContextMenuBase {
    /// Whether the menu is currently visible.
    pub is_open: bool,
    /// Screen position (pixels) where the menu was opened.
    pub position: Vec2,
}

impl ContextMenuBase {
    /// Open the menu at the given screen position.
    pub fn open(&mut self, position: Vec2) {
        self.is_open = true;
        self.position = position;
    }

    /// Close the menu.
    pub fn close(&mut self) {
        self.is_open = false;
    }
}

/// Style helper: renders a context menu container positioned at the given screen coordinates.
///
/// Use this inside an `ImmediateAttach::construct` to wrap your menu items:
/// ```rust,ignore
/// fn construct(ui: &mut Imm<CapsUi>, params: &mut (...)) {
///     if !state.base.is_open { return; }
///     style_context_menu_panel(ui, state.base.position, |ui| {
///         // render your menu items here
///     });
/// }
/// ```
pub fn style_context_menu_panel<Cap>(
    ui: &mut Imm<'_, '_, Cap>,
    position: Vec2,
    children: impl FnOnce(&mut Imm<'_, '_, Cap>),
) where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    ui.ch_id("context_menu")
        .style(move |n: &mut Node| {
            n.position_type = PositionType::Absolute;
            n.left = Val::Px(position.x);
            n.top = Val::Px(position.y);
            n.min_width = Val::Px(180.0);
            n.max_height = Val::Px(400.0);
            n.flex_direction = FlexDirection::Column;
            n.padding = UiRect::all(Val::Px(SPACE_1));
        })
        .bg(GRAY_800)
        .border(BORDER_WIDTH_DEFAULT)
        .border_color(GRAY_700)
        .rounded(RADIUS_DEFAULT)
        .z_index(300)
        .add(children);
}

/// Style helper: renders a context menu item (row) with standard padding.
///
/// Returns the entity so you can chain `.on_click_once(...)`.
pub fn style_context_menu_item<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiTextStyle>,
{
    entity
        .w_full()
        .px(Val::Px(SPACE_2))
        .py(Val::Px(SPACE_1))
        .flex_row()
        .items_center()
        .rounded(RADIUS_SM)
        .text_size(13.0)
        .text_color(GRAY_100)
}

/// Style helper: renders a horizontal divider inside a context menu.
pub fn context_menu_divider<Cap>(ui: &mut Imm<'_, '_, Cap>)
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    ui.ch()
        .w_full()
        .h(Val::Px(1.0))
        .my(Val::Px(SPACE_1))
        .bg(GRAY_700);
}

/// Style helper: renders a context menu header label.
pub fn context_menu_header<Cap>(ui: &mut Imm<'_, '_, Cap>, text: &str)
where
    Cap: CapSet
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiTextStyle>
        + ImplCap<CapabilityUiText>,
{
    ui.ch()
        .label(text)
        .text_size(13.0)
        .text_color(GRAY_100)
        .font_bold()
        .px(Val::Px(SPACE_2))
        .py(Val::Px(SPACE_1));
}
