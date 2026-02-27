use bevy::prelude::*;
use bevy::ui::Checked;
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::{CapSet, Imm, ImmEntity, ImplCap};

use crate::theme::prelude::*;

// -----------------------------------------------------------------------------
// RadioGroup Widget — Styled wrapper around Bevy's headless RadioGroup
// -----------------------------------------------------------------------------
// Bevy 0.17 provides `bevy::ui_widgets::RadioGroup` and `RadioButton` which
// handle mutual exclusion, keyboard nav, and accessibility. This provides
// the visual layer.
//
// Usage:
//   ui.ch_id("difficulty").radio_group(|ui| {
//       ui.ch_id("easy").radio_button("Easy", selected == 0);
//       ui.ch_id("normal").radio_button("Normal", selected == 1);
//       ui.ch_id("hard").radio_button("Hard", selected == 2);
//   });

pub trait ImmUiRadioGroup<Cap> {
    /// Creates a radio group container. Place `.radio_button()` children inside.
    fn radio_group(self, children: impl FnOnce(&mut Imm<'_, '_, Cap>)) -> Self;
}

pub trait ImmUiRadioButton {
    /// Creates a styled radio button with a label and checked state.
    fn radio_button(self, label: impl Into<String>, checked: bool) -> Self;
}

impl<Cap> ImmUiRadioGroup<Cap> for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout> + ImplCap<CapabilityUiVisuals> + CapSet,
{
    fn radio_group(self, children: impl FnOnce(&mut Imm<'_, '_, Cap>)) -> Self {
        self.on_spawn_insert(|| bevy::ui_widgets::RadioGroup)
            .style(|s| {
                s.display = Display::Flex;
                s.flex_direction = FlexDirection::Column;
                s.row_gap = Val::Px(4.0);
            })
            .add(children)
    }
}

impl<Cap> ImmUiRadioButton for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiTextStyle>
        + ImplCap<CapabilityUiText>
        + ImplCap<CapabilityButton>
        + ImplCap<CapabilityObserver>
        + CapSet,
{
    fn radio_button(mut self, label: impl Into<String>, checked: bool) -> Self {
        let text = label.into();

        // Insert Bevy's headless radio button + checked state
        if checked {
            self = self.on_spawn_insert(|| (bevy::ui_widgets::RadioButton, Checked));
        } else {
            self = self.on_spawn_insert(|| bevy::ui_widgets::RadioButton);
        }

        // Row layout: circle + label
        self = self.style(|s| {
            s.display = Display::Flex;
            s.flex_direction = FlexDirection::Row;
            s.align_items = AlignItems::Center;
            s.column_gap = Val::Px(8.0);
            s.padding = UiRect::axes(Val::Px(4.0), Val::Px(4.0));
            s.flex_shrink = 0.0;
        });

        // Radio circle indicator
        let is_checked = checked;
        self.add(move |ui| {
            // Outer circle
            let mut circle = ui
                .ch_id("radio_circle")
                .style(|s| {
                    s.width = Val::Px(18.0);
                    s.height = Val::Px(18.0);
                    s.align_items = AlignItems::Center;
                    s.justify_content = JustifyContent::Center;
                    s.border = UiRect::all(Val::Px(2.0));
                    s.flex_shrink = 0.0;
                })
                .rounded(9.0); // Fully round

            if is_checked {
                circle = circle.border_color(PRIMARY_500).bg(TRANSPARENT);

                // Inner filled circle
                circle.add(|ui| {
                    ui.ch_id("radio_dot")
                        .style(|s| {
                            s.width = Val::Px(10.0);
                            s.height = Val::Px(10.0);
                            s.flex_shrink = 0.0;
                        })
                        .bg(PRIMARY_500)
                        .rounded(5.0);
                });
            } else {
                circle.border_color(GRAY_700).bg(TRANSPARENT);
            }

            // Label text
            ui.ch_id("radio_label")
                .label(text)
                .text_sm()
                .color(GRAY_100);
        })
    }
}
