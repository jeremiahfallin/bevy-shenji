use bevy::prelude::*;
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::{CapSet, ImmEntity, ImplCap};

use crate::theme::prelude::*;

// -----------------------------------------------------------------------------
// Badge / Tag Widget
// -----------------------------------------------------------------------------
// Usage:
//   ui.ch().badge("New");                          // default badge (gray)
//   ui.ch().badge("Active").badge_variant(BadgeVariant::Success);
//   ui.ch().badge("3").badge_variant(BadgeVariant::Primary);
//   ui.ch().badge("Error").badge_variant(BadgeVariant::Danger);

#[derive(Clone, Copy, Debug, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Primary,
    Success,
    Danger,
    Info,
}

impl BadgeVariant {
    fn bg_color(self) -> Color {
        match self {
            BadgeVariant::Default => GRAY_700,
            BadgeVariant::Primary => PRIMARY_500,
            BadgeVariant::Success => SUCCESS_600,
            BadgeVariant::Danger => ERROR_600,
            BadgeVariant::Info => INFO_600,
        }
    }

    fn text_color(self) -> Color {
        Color::WHITE
    }
}

pub trait ImmUiBadge {
    /// Creates a small badge/tag with text.
    fn badge(self, text: impl Into<String>) -> Self;

    /// Sets the badge color variant.
    fn badge_variant(self, variant: BadgeVariant) -> Self;
}

impl<Cap> ImmUiBadge for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiTextStyle>
        + ImplCap<CapabilityUiText>
        + CapSet,
{
    fn badge(self, text: impl Into<String>) -> Self {
        let t = text.into();
        self
            // Container styling
            .style(|s| {
                s.padding = UiRect::axes(Val::Px(8.0), Val::Px(2.0));
                s.align_items = AlignItems::Center;
                s.justify_content = JustifyContent::Center;
                s.flex_shrink = 0.0;
            })
            .bg(GRAY_700)
            .rounded(4.0)
            // Text child
            .add(move |ui| {
                ui.ch_id("badge_text")
                    .label(t)
                    .size(LabelSize::Small)
                    .color(Color::WHITE)
                    .single_line();
            })
    }

    fn badge_variant(self, variant: BadgeVariant) -> Self {
        self.bg(variant.bg_color())
    }
}
