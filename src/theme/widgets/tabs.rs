use bevy::prelude::*;
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::{CapSet, Imm, ImmEntity, ImplCap};

use crate::theme::prelude::*;

// -----------------------------------------------------------------------------
// Tabs Widget
// -----------------------------------------------------------------------------
// Usage:
//   let tabs = ["Health", "Skills", "Inventory"];
//   let active = 0usize;
//
//   ui.ch().tab_bar(|ui| {
//       for (i, name) in tabs.iter().enumerate() {
//           ui.ch_id(*name)
//             .tab(name.to_string(), i == active);
//       }
//   });
//
//   // Then render tab content based on `active` index.

pub trait ImmUiTabBar<Cap> {
    /// Creates a horizontal tab bar container. Add `.tab()` children inside.
    fn tab_bar(self, children: impl FnOnce(&mut Imm<'_, '_, Cap>)) -> Self;
}

pub trait ImmUiTab {
    /// Renders a single tab item with label and active state.
    fn tab(self, label: impl Into<String>, active: bool) -> Self;
}

impl<Cap> ImmUiTabBar<Cap> for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout> + ImplCap<CapabilityUiVisuals> + CapSet,
{
    fn tab_bar(self, children: impl FnOnce(&mut Imm<'_, '_, Cap>)) -> Self {
        self.style(|s| {
            s.display = Display::Flex;
            s.flex_direction = FlexDirection::Row;
            s.width = Val::Percent(100.0);
            s.flex_shrink = 0.0;
            s.column_gap = Val::Px(2.0);
            // Bottom border to visually separate tabs from content
            s.border = UiRect::bottom(Val::Px(1.0));
        })
        .border_color(GRAY_700)
        .add(children)
    }
}

impl<Cap> ImmUiTab for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiTextStyle>
        + ImplCap<CapabilityUiText>
        + ImplCap<CapabilityButton>
        + ImplCap<CapabilityObserver>
        + CapSet,
{
    fn tab(mut self, label: impl Into<String>, active: bool) -> Self {
        let text = label.into();

        // Insert interaction components for hover/click
        self = self.on_spawn_insert(|| {
            (
                Button,
                Node::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            )
        });

        // Base tab styling
        self = self.style(|s| {
            s.padding = UiRect::axes(Val::Px(12.0), Val::Px(8.0));
            s.align_items = AlignItems::Center;
            s.justify_content = JustifyContent::Center;
            // Offset the bottom border so active tab overlaps the bar border
            s.margin = UiRect::bottom(Val::Px(-1.0));
            s.border = UiRect::bottom(Val::Px(2.0));
        });

        if active {
            self = self.border_color(PRIMARY_500).bg(TRANSPARENT);
        } else {
            self = self.border_color(TRANSPARENT).bg(TRANSPARENT);

            // Apply hover state
            if let Ok(Some(interaction)) = self.cap_get_component::<Interaction>() {
                if *interaction == Interaction::Hovered {
                    self = self.bg(GRAY_800);
                }
            }
        }

        // Label
        let is_active = active;
        self.add(move |ui| {
            let mut label_entity = ui.ch_id("tab_label").label(text).text_sm();

            if is_active {
                label_entity = label_entity.color(Color::WHITE).weight(FontWeight::Bold);
            } else {
                label_entity.color(Color::srgba(1.0, 1.0, 1.0, 0.6));
            }
        })
    }
}
