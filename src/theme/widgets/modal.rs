use bevy::prelude::*;
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::{CapSet, Imm, ImmEntity, ImplCap};

use crate::theme::prelude::*;

// -----------------------------------------------------------------------------
// Modal / Dialog Widget
// -----------------------------------------------------------------------------
// Usage:
//   // Conditionally render based on state
//   if show_modal {
//       ui.ch().modal_overlay(|ui| {
//           ui.ch().modal_dialog(|ui| {
//               ui.ch().modal_header("Confirm Action");
//               ui.ch().modal_body(|ui| {
//                   ui.ch().label("Are you sure you want to proceed?");
//               });
//               ui.ch().modal_footer(|ui| {
//                   ui.ch().button().with_label("Cancel");
//                   ui.ch().button().with_label("Confirm");
//               });
//           });
//       });
//   }

/// Sizes for the modal dialog.
#[derive(Clone, Copy, Debug, Default)]
pub enum ModalSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ModalSize {
    fn width(self) -> Val {
        match self {
            ModalSize::Small => Val::Px(320.0),
            ModalSize::Medium => Val::Px(480.0),
            ModalSize::Large => Val::Px(640.0),
        }
    }

    fn max_height(self) -> Val {
        match self {
            ModalSize::Small => Val::Percent(60.0),
            ModalSize::Medium => Val::Percent(75.0),
            ModalSize::Large => Val::Percent(85.0),
        }
    }
}

pub trait ImmUiModal<Cap> {
    /// Creates a full-screen semi-transparent overlay for the modal backdrop.
    /// Place the modal_dialog inside this.
    fn modal_overlay(self, children: impl FnOnce(&mut Imm<'_, '_, Cap>)) -> Self;

    /// Creates the modal dialog container (centered card).
    fn modal_dialog(self, children: impl FnOnce(&mut Imm<'_, '_, Cap>)) -> Self;

    /// Sets the modal dialog size.
    fn modal_size(self, size: ModalSize) -> Self;

    /// Creates a modal header section with title text.
    fn modal_header(self, title: impl Into<String>) -> Self;

    /// Creates a modal body section for content.
    fn modal_body(self, children: impl FnOnce(&mut Imm<'_, '_, Cap>)) -> Self;

    /// Creates a modal footer section (typically for action buttons).
    fn modal_footer(self, children: impl FnOnce(&mut Imm<'_, '_, Cap>)) -> Self;
}

impl<Cap> ImmUiModal<Cap> for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiTextStyle>
        + ImplCap<CapabilityUiText>
        + CapSet,
{
    fn modal_overlay(self, children: impl FnOnce(&mut Imm<'_, '_, Cap>)) -> Self {
        self.style(|s| {
            s.position_type = PositionType::Absolute;
            s.left = Val::Px(0.0);
            s.right = Val::Px(0.0);
            s.top = Val::Px(0.0);
            s.bottom = Val::Px(0.0);
            s.display = Display::Flex;
            s.align_items = AlignItems::Center;
            s.justify_content = JustifyContent::Center;
        })
        .bg(Color::srgba(0.0, 0.0, 0.0, 0.5))
        .z_index_global(1000)
        .add(children)
    }

    fn modal_dialog(self, children: impl FnOnce(&mut Imm<'_, '_, Cap>)) -> Self {
        self.style(|s| {
            s.display = Display::Flex;
            s.flex_direction = FlexDirection::Column;
            s.width = ModalSize::Medium.width();
            s.max_height = ModalSize::Medium.max_height();
            // Prevent dialog from being squished
            s.flex_shrink = 0.0;
        })
        .bg(GRAY_800)
        .rounded(8.0)
        .border(1.0)
        .border_color(GRAY_700)
        .add(children)
    }

    fn modal_size(self, size: ModalSize) -> Self {
        self.style(move |s| {
            s.width = size.width();
            s.max_height = size.max_height();
        })
    }

    fn modal_header(self, title: impl Into<String>) -> Self {
        let t = title.into();
        self.style(|s| {
            s.display = Display::Flex;
            s.flex_direction = FlexDirection::Row;
            s.align_items = AlignItems::Center;
            s.padding = UiRect::axes(Val::Px(16.0), Val::Px(12.0));
            s.border = UiRect::bottom(Val::Px(1.0));
            s.flex_shrink = 0.0;
        })
        .border_color(GRAY_700)
        .add(move |ui| {
            ui.ch_id("modal_title")
                .label(t)
                .size(LabelSize::Large)
                .weight(FontWeight::Bold)
                .color(Color::WHITE);
        })
    }

    fn modal_body(self, children: impl FnOnce(&mut Imm<'_, '_, Cap>)) -> Self {
        self.style(|s| {
            s.display = Display::Flex;
            s.flex_direction = FlexDirection::Column;
            s.padding = UiRect::all(Val::Px(16.0));
            s.flex_grow = 1.0;
            s.overflow.y = OverflowAxis::Scroll;
        })
        .add(children)
    }

    fn modal_footer(self, children: impl FnOnce(&mut Imm<'_, '_, Cap>)) -> Self {
        self.style(|s| {
            s.display = Display::Flex;
            s.flex_direction = FlexDirection::Row;
            s.align_items = AlignItems::Center;
            s.justify_content = JustifyContent::FlexEnd;
            s.padding = UiRect::axes(Val::Px(16.0), Val::Px(12.0));
            s.column_gap = Val::Px(8.0);
            s.border = UiRect::top(Val::Px(1.0));
            s.flex_shrink = 0.0;
        })
        .border_color(GRAY_700)
        .add(children)
    }
}
