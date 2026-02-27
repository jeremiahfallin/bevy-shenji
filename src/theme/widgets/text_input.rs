use bevy::prelude::*;
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::{CapSet, ImmEntity, ImplCap};

use crate::theme::prelude::*;

// -----------------------------------------------------------------------------
// TextInput / TextField Widget
// -----------------------------------------------------------------------------
// Bevy 0.17 does not provide a built-in text input widget. This provides
// a visual container styled as a text field. Actual text editing requires
// keyboard input handling at the game level.
//
// Usage:
//   ui.ch_id("player_name")
//     .text_input("Player name...", &current_value, is_focused);
//
//   // With custom sizing:
//   ui.ch_id("search")
//     .text_input("Search...", &query, focused)
//     .text_input_size(TextInputSize::Large);
//
// NOTE: This provides the visual styling only. Wire up keyboard input
// handling in your game systems using Bevy's keyboard events and
// InputFocus resource.

#[derive(Clone, Copy, Debug, Default)]
pub enum TextInputSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl TextInputSize {
    fn height(self) -> Val {
        match self {
            TextInputSize::Small => Val::Px(28.0),
            TextInputSize::Medium => Val::Px(36.0),
            TextInputSize::Large => Val::Px(44.0),
        }
    }

    fn padding_x(self) -> Val {
        match self {
            TextInputSize::Small => Val::Px(8.0),
            TextInputSize::Medium => Val::Px(12.0),
            TextInputSize::Large => Val::Px(16.0),
        }
    }

    fn text_size(self) -> f32 {
        match self {
            TextInputSize::Small => 12.0,
            TextInputSize::Medium => 14.0,
            TextInputSize::Large => 16.0,
        }
    }
}

pub trait ImmUiTextInput {
    /// Creates a styled text input field.
    /// `placeholder` is shown when `value` is empty.
    /// `focused` controls the visual focus ring.
    fn text_input(self, placeholder: impl Into<String>, value: &str, focused: bool) -> Self;

    /// Sets the input size variant.
    fn text_input_size(self, size: TextInputSize) -> Self;

    /// Makes the text input take full available width.
    fn text_input_full_width(self) -> Self;
}

impl<Cap> ImmUiTextInput for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiTextStyle>
        + ImplCap<CapabilityUiText>
        + CapSet,
{
    fn text_input(self, placeholder: impl Into<String>, value: &str, focused: bool) -> Self {
        let placeholder_text = placeholder.into();
        let current_value = value.to_string();
        let is_empty = current_value.is_empty();

        let mut entity = self
            .style(|s| {
                s.display = Display::Flex;
                s.flex_direction = FlexDirection::Row;
                s.align_items = AlignItems::Center;
                s.height = TextInputSize::Medium.height();
                s.padding = UiRect::axes(TextInputSize::Medium.padding_x(), Val::Px(0.0));
                s.border = UiRect::all(Val::Px(1.0));
                s.min_width = Val::Px(120.0);
                s.flex_shrink = 0.0;
                // Clip overflow text
                s.overflow.x = OverflowAxis::Clip;
            })
            .bg(GRAY_900)
            .rounded(6.0);

        // Focus ring
        if focused {
            entity = entity.border_color(PRIMARY_500);
        } else {
            entity = entity.border_color(GRAY_700);
        }

        // Text content
        entity.add(move |ui| {
            if is_empty {
                // Show placeholder
                ui.ch_id("placeholder")
                    .label(placeholder_text)
                    .text_size(TextInputSize::Medium.text_size())
                    .color(Color::srgba(1.0, 1.0, 1.0, 0.35))
                    .single_line();
            } else {
                // Show current value
                ui.ch_id("value")
                    .label(current_value)
                    .text_size(TextInputSize::Medium.text_size())
                    .color(GRAY_100)
                    .single_line();
            }
        })
    }

    fn text_input_size(self, size: TextInputSize) -> Self {
        self.style(move |s| {
            s.height = size.height();
            s.padding = UiRect::axes(size.padding_x(), Val::Px(0.0));
        })
    }

    fn text_input_full_width(self) -> Self {
        self.style(|s| {
            s.width = Val::Percent(100.0);
        })
    }
}
