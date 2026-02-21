use bevy::prelude::*;
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::{CapSet, ImmEntity, ImplCap};

use crate::theme::prelude::*;

// -----------------------------------------------------------------------------
// Checkbox / Toggle Widget
// -----------------------------------------------------------------------------
// Usage:
//   ui.ch_id("my_checkbox")
//     .checkbox(is_checked)
//     .checkbox_label("Enable notifications");
//
//   // Attach click handler to toggle state:
//   ui.ch_id("my_checkbox")
//     .checkbox(is_checked)
//     .checkbox_label("Enable notifications")
//     .on_click_once(|_trigger, mut state: ResMut<MyState>| {
//         state.notifications = !state.notifications;
//     });

pub trait ImmUiCheckbox {
    /// Creates a checkbox with the given checked state.
    fn checkbox(self, checked: bool) -> Self;

    /// Adds a text label next to the checkbox.
    fn checkbox_label(self, text: impl Into<String>) -> Self;

    /// Creates a toggle switch (pill-shaped) with the given state.
    fn toggle_switch(self, on: bool) -> Self;

    /// Adds a text label next to the toggle switch.
    fn toggle_label(self, text: impl Into<String>) -> Self;
}

impl<Cap> ImmUiCheckbox for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiTextStyle>
        + ImplCap<CapabilityUiText>
        + ImplCap<CapabilityButton>
        + ImplCap<CapabilityObserver>
        + CapSet,
{
    fn checkbox(mut self, checked: bool) -> Self {
        // Insert interaction components
        self = self.on_spawn_insert(|| {
            (
                Button,
                Node::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            )
        });

        // Container: row layout with gap
        self = self.style(|s| {
            s.display = Display::Flex;
            s.flex_direction = FlexDirection::Row;
            s.align_items = AlignItems::Center;
            s.column_gap = Val::Px(8.0);
            s.flex_shrink = 0.0;
        });

        // Checkbox box
        let is_checked = checked;
        self.add(move |ui| {
            let mut box_entity = ui.ch_id("checkbox_box")
                .style(|s| {
                    s.width = Val::Px(18.0);
                    s.height = Val::Px(18.0);
                    s.align_items = AlignItems::Center;
                    s.justify_content = JustifyContent::Center;
                    s.border = UiRect::all(Val::Px(2.0));
                    s.flex_shrink = 0.0;
                })
                .rounded(4.0);

            if is_checked {
                box_entity = box_entity
                    .bg(PRIMARY_500)
                    .border_color(PRIMARY_500);

                // Checkmark icon
                box_entity.add(|ui| {
                    ui.ch_id("check_icon")
                        .icon(lucide_icons::Icon::Check)
                        .text_size(12.0)
                        .text_color(Color::WHITE);
                });
            } else {
                box_entity
                    .bg(TRANSPARENT)
                    .border_color(GRAY_700);
            }
        })
    }

    fn checkbox_label(self, text: impl Into<String>) -> Self {
        let t = text.into();
        self.add(move |ui| {
            ui.ch_id("checkbox_label")
                .label(t)
                .text_sm()
                .color(GRAY_100);
        })
    }

    fn toggle_switch(mut self, on: bool) -> Self {
        // Insert interaction components
        self = self.on_spawn_insert(|| {
            (
                Button,
                Node::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            )
        });

        // Container: row layout
        self = self.style(|s| {
            s.display = Display::Flex;
            s.flex_direction = FlexDirection::Row;
            s.align_items = AlignItems::Center;
            s.column_gap = Val::Px(8.0);
            s.flex_shrink = 0.0;
        });

        // Toggle track
        let is_on = on;
        self.add(move |ui| {
            let track_color = if is_on { PRIMARY_500 } else { GRAY_700 };

            ui.ch_id("toggle_track")
                .style(move |s| {
                    s.width = Val::Px(36.0);
                    s.height = Val::Px(20.0);
                    s.padding = UiRect::all(Val::Px(2.0));
                    s.flex_shrink = 0.0;
                    // Align the knob to the correct side
                    if is_on {
                        s.justify_content = JustifyContent::FlexEnd;
                    } else {
                        s.justify_content = JustifyContent::FlexStart;
                    }
                })
                .bg(track_color)
                .rounded(10.0)
                // Knob
                .add(|ui| {
                    ui.ch_id("toggle_knob")
                        .style(|s| {
                            s.width = Val::Px(16.0);
                            s.height = Val::Px(16.0);
                            s.flex_shrink = 0.0;
                        })
                        .bg(Color::WHITE)
                        .rounded(8.0);
                });
        })
    }

    fn toggle_label(self, text: impl Into<String>) -> Self {
        let t = text.into();
        self.add(move |ui| {
            ui.ch_id("toggle_label")
                .label(t)
                .text_sm()
                .color(GRAY_100);
        })
    }
}
