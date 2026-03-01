use bevy::prelude::*;
use bevy::{color::Color, ui::BackgroundColor};

use bevy_immediate::ui::base::CapabilityUiBase;
use bevy_immediate::{CapSet, ImmEntity, ImplCap};

use crate::theme::components::list::{List, ListItem};
use crate::theme::prelude::*;
use crate::theme::widgets::label::{FontWeight, LabelSize};
use crate::theme::widgets::table::Table;
use bevy_immediate::Imm;

// Re-export from primitives
pub use crate::theme::primitives::{CapabilityUiLayout, CapabilityUiTextStyle, ImmUiLayout};

pub struct ExtensionPlugin;

impl bevy::app::Plugin for ExtensionPlugin {
    fn build(&self, _app: &mut bevy::app::App) {
        // No need to implement plugin in this case
    }
}

// CapUiBase removed, using CapabilityUiLayout from primitives instead.

pub trait GpuiStyleExtensions {
    fn p_4(self) -> Self;
    fn bg_red(self) -> Self;
}

impl<Cap> GpuiStyleExtensions for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout> + ImplCap<CapabilityUiBase>,
{
    fn p_4(self) -> Self {
        self.p(Val::Px(SPACE_4))
    }

    fn bg_red(self) -> Self {
        self.on_spawn_insert(|| BackgroundColor(Color::hsla(200.0, 1.0, 0.5, 1.0)))
    }
}

pub trait ImmUiCompositeWidgets<Cap: CapSet> {
    fn list(&mut self) -> List;
    fn list_item(&mut self, id: impl Into<String>) -> ListItem<Cap>;
    fn table(&mut self) -> Table;
}

impl<Cap> ImmUiCompositeWidgets<Cap> for Imm<'_, '_, Cap>
where
    Cap: CapSet,
{
    fn list(&mut self) -> List {
        List::new()
    }

    fn list_item(&mut self, id: impl Into<String>) -> ListItem<Cap> {
        ListItem::new(id)
    }

    fn table(&mut self) -> Table {
        Table::new()
    }
}

pub trait ImmUiHeader {
    /// Creates a styled header (H1-like).
    fn header(self, text: impl Into<String>) -> Self;

    /// Creates a sub-header (H2-like) with slightly smaller text/muted color.
    fn sub_header(self, text: impl Into<String>) -> Self;
}

impl<Cap> ImmUiHeader for bevy_immediate::ImmEntity<'_, '_, '_, Cap>
where
    // Ensure the entity has the capabilities required by .label()
    Cap: ImplCap<CapabilityUiTextStyle> + ImplCap<bevy_immediate::ui::text::CapabilityUiText>,
{
    fn header(self, text: impl Into<String>) -> Self {
        // Composition: Start with a label, then apply the "Header" style
        self.label(text)
            .size(LabelSize::Large)
            .weight(FontWeight::Bold)
        // Optional: Set a default color if headers should always be distinct
        // .color(Color::WHITE)
    }

    fn sub_header(self, text: impl Into<String>) -> Self {
        self.label(text)
            .size(LabelSize::Default)
            .weight(FontWeight::Bold)
            .alpha(0.8) // Slightly muted
    }
}

pub trait ImmUiTextStyle {
    // Size Utilities
    fn text_xs(self) -> Self;
    fn text_sm(self) -> Self;
    fn text_base(self) -> Self;
    fn text_lg(self) -> Self;
    fn text_xl(self) -> Self;
    fn text_2xl(self) -> Self;
    fn text_3xl(self) -> Self;

    // Weight Utilities (If you have font variants loaded)
    fn font_bold(self) -> Self;
    fn font_italic(self) -> Self;

    // Generic Helper
    fn text_size(self, size: f32) -> Self;
    fn text_color(self, color: Color) -> Self;
}

// ImmUiStyleExt moved to primitives/style.rs

pub trait ImmUiApply: Sized {
    fn apply(self, func: impl FnOnce(Self) -> Self) -> Self;
}

impl<Cap> ImmUiApply for ImmEntity<'_, '_, '_, Cap>
where
    Cap: CapSet,
{
    fn apply(self, func: impl FnOnce(Self) -> Self) -> Self {
        func(self)
    }
}
