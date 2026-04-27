use bevy::prelude::*;
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::{CapSet, Imm, ImmEntity, ImplCap};

use crate::theme::prelude::*;
use crate::theme::widgets::label::FontWeight;

// -----------------------------------------------------------------------------
// Dropdown / Select Widget
// -----------------------------------------------------------------------------
// A compact option picker. Since Bevy 0.17 has no built-in dropdown,
// this provides a basic implementation using immediate-mode patterns.
//
// Usage:
//   let options = ["Low", "Medium", "High"];
//   let selected = 1; // "Medium"
//   let is_open = ui_state.dropdown_open;
//
//   ui.ch_id("quality_dropdown")
//     .dropdown_trigger(options[selected], is_open);
//
//   if is_open {
//       ui.ch_id("quality_menu").dropdown_menu(|ui| {
//           for (i, option) in options.iter().enumerate() {
//               ui.ch_id(*option)
//                 .dropdown_item(option.to_string(), i == selected);
//           }
//       });
//   }

pub trait ImmUiDropdown<Cap> {
    /// Creates the dropdown trigger button showing the current selection.
    fn dropdown_trigger(self, current_label: impl Into<String>, is_open: bool) -> Self;

    /// Creates the dropdown menu container (absolutely positioned below trigger).
    /// Place `.dropdown_item()` children inside.
    fn dropdown_menu(self, children: impl FnOnce(&mut Imm<'_, '_, Cap>)) -> Self;
}

pub trait ImmUiDropdownItem {
    /// Creates a selectable item inside a dropdown menu.
    fn dropdown_item(self, label: impl Into<String>, selected: bool) -> Self;
}

impl<Cap> ImmUiDropdown<Cap> for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiTextStyle>
        + ImplCap<CapabilityUiText>
        + ImplCap<CapabilityButton>
        + ImplCap<CapabilityObserver>
        + CapSet,
{
    fn dropdown_trigger(mut self, current_label: impl Into<String>, is_open: bool) -> Self {
        let text = current_label.into();

        // Make it a clickable button
        self = self.on_spawn_insert(|| {
            (
                Button,
                Node::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            )
        });

        // Trigger button styling
        self = self
            .style(|s| {
                s.display = Display::Flex;
                s.flex_direction = FlexDirection::Row;
                s.align_items = AlignItems::Center;
                s.justify_content = JustifyContent::SpaceBetween;
                s.padding = UiRect::axes(Val::Px(12.0), Val::Px(8.0));
                s.min_width = Val::Px(120.0);
                s.border = UiRect::all(Val::Px(1.0));
                s.flex_shrink = 0.0;
            })
            .bg(GRAY_800)
            .border_color(GRAY_700)
            .rounded(6.0);

        // Apply hover state
        if let Ok(Some(interaction)) = self.cap_get_component::<Interaction>() {
            if *interaction == Interaction::Hovered {
                self = self.border_color(PRIMARY_500);
            }
        }

        // Open state
        if is_open {
            self = self.border_color(PRIMARY_500);
        }

        let is_open_state = is_open;
        self.add(move |ui| {
            // Current selection label
            ui.ch_id("dropdown_label")
                .label(text)
                .text_sm()
                .color(GRAY_100)
                .single_line();

            // Chevron icon
            let chevron = if is_open_state {
                lucide_icons::Icon::ChevronUp
            } else {
                lucide_icons::Icon::ChevronDown
            };
            ui.ch_id("dropdown_chevron")
                .icon(chevron)
                .text_sm()
                .text_color(Color::srgba(1.0, 1.0, 1.0, 0.5));
        })
    }

    fn dropdown_menu(self, children: impl FnOnce(&mut Imm<'_, '_, Cap>)) -> Self {
        self.style(|s| {
            s.position_type = PositionType::Absolute;
            s.top = Val::Percent(100.0);
            s.left = Val::Px(0.0);
            s.right = Val::Px(0.0);
            s.display = Display::Flex;
            s.flex_direction = FlexDirection::Column;
            s.margin = UiRect::top(Val::Px(2.0));
            s.padding = UiRect::axes(Val::Px(0.0), Val::Px(4.0));
            s.max_height = Val::Px(200.0);
            s.overflow.y = OverflowAxis::Scroll;
        })
        .bg(GRAY_800)
        .border(1.0)
        .border_color(GRAY_700)
        .rounded(6.0)
        .z_index(50)
        .add(children)
    }
}

impl<Cap> ImmUiDropdownItem for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiTextStyle>
        + ImplCap<CapabilityUiText>
        + ImplCap<CapabilityButton>
        + ImplCap<CapabilityObserver>
        + CapSet,
{
    fn dropdown_item(mut self, label: impl Into<String>, selected: bool) -> Self {
        let text = label.into();

        // Make clickable
        self = self.on_spawn_insert(|| {
            (
                Button,
                Node::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            )
        });

        // Item styling
        self = self.style(|s| {
            s.display = Display::Flex;
            s.flex_direction = FlexDirection::Row;
            s.align_items = AlignItems::Center;
            s.padding = UiRect::axes(Val::Px(12.0), Val::Px(6.0));
            s.width = Val::Percent(100.0);
        });

        if selected {
            self = self.bg(PRIMARY_600);
        } else {
            self = self.bg(TRANSPARENT);

            // Hover state
            if let Ok(Some(interaction)) = self.cap_get_component::<Interaction>() {
                if *interaction == Interaction::Hovered {
                    self = self.bg(GRAY_700);
                }
            }
        }

        let is_selected = selected;
        self.add(move |ui| {
            let mut label_ent = ui.ch_id("item_label").label(text).text_sm();

            if is_selected {
                label_ent = label_ent.color(Color::WHITE).weight(FontWeight::Bold);
            } else {
                label_ent.color(GRAY_100);
            }
        })
    }
}
