use bevy::prelude::*;
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::{CapSet, ImmEntity, ImplCap};

use crate::theme::prelude::*;

// -----------------------------------------------------------------------------
// Tooltip Widget
// -----------------------------------------------------------------------------
// Tooltips are rendered as hidden overlays that become visible on hover.
// In immediate-mode UI, the pattern is to check Interaction state and
// conditionally render the tooltip.
//
// Usage:
//   // Check hover on the parent entity, then render tooltip
//   let hovered = /* check Interaction::Hovered on parent */;
//   if hovered {
//       ui.ch().tooltip("This is helpful info");
//   }
//
//   // Or with positioning:
//   ui.ch().tooltip("Helpful info").tooltip_position(TooltipPosition::Bottom);

#[derive(Clone, Copy, Debug, Default)]
pub enum TooltipPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

pub trait ImmUiTooltip {
    /// Creates a tooltip overlay with the given text.
    /// Should be rendered inside a relatively-positioned parent, and
    /// conditionally shown when the parent is hovered.
    fn tooltip(self, text: impl Into<String>) -> Self;

    /// Sets the tooltip position relative to its parent.
    fn tooltip_position(self, position: TooltipPosition) -> Self;
}

impl<Cap> ImmUiTooltip for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiTextStyle>
        + ImplCap<CapabilityUiText>
        + CapSet,
{
    fn tooltip(self, text: impl Into<String>) -> Self {
        let t = text.into();

        self
            // Absolutely positioned above parent by default
            .style(|s| {
                s.position_type = PositionType::Absolute;
                s.bottom = Val::Percent(100.0);
                s.left = Val::Percent(50.0);
                // Padding and layout
                s.padding = UiRect::axes(Val::Px(8.0), Val::Px(4.0));
                s.flex_shrink = 0.0;
            })
            .bg(GRAY_900)
            .border(1.0)
            .border_color(GRAY_700)
            .rounded(4.0)
            .z_index(100)
            // Text child
            .add(move |ui| {
                ui.ch_id("tooltip_text")
                    .label(t)
                    .size(LabelSize::Small)
                    .color(GRAY_100)
                    .single_line();
            })
    }

    fn tooltip_position(self, position: TooltipPosition) -> Self {
        self.style(move |s| {
            // Reset all positioning
            s.top = Val::Auto;
            s.bottom = Val::Auto;
            s.left = Val::Auto;
            s.right = Val::Auto;

            match position {
                TooltipPosition::Top => {
                    s.bottom = Val::Percent(100.0);
                    s.left = Val::Percent(50.0);
                }
                TooltipPosition::Bottom => {
                    s.top = Val::Percent(100.0);
                    s.left = Val::Percent(50.0);
                }
                TooltipPosition::Left => {
                    s.right = Val::Percent(100.0);
                    s.top = Val::Percent(50.0);
                }
                TooltipPosition::Right => {
                    s.left = Val::Percent(100.0);
                    s.top = Val::Percent(50.0);
                }
            }
        })
    }
}
