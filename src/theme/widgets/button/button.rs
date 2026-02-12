use crate::theme::primitives::image::{CapabilityUiImage, ImmUiImageExt};
use crate::theme::primitives::text::CapabilityUiTextStyle;
use bevy::prelude::*;
use bevy_immediate::ui::CapsUi;
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::{CapSet, ImmCapAccessRequests, ImmCapability, ImmEntity, ImplCap};

use crate::theme::prelude::*;
use crate::theme::styles::buttons::{style_btn_primary, style_btn_primary_hover};

// 1. Define the Capability
pub struct CapabilityButton;

impl ImmCapability for CapabilityButton {
    fn build<Cap: CapSet>(app: &mut bevy::app::App, cap_req: &mut ImmCapAccessRequests<Cap>) {
        cap_req.request_component_write::<Interaction>(app.world_mut());
    }
}

impl ImplCap<CapabilityButton> for CapsUi {}

// 2. Define the API
pub trait ImmUiButton<Cap: CapSet> {
    fn button(self) -> Self;
    fn icon_button(self) -> Self;
    fn with_label(self, text: impl Into<String>) -> Self;
    fn with_styled_label(
        self,
        text: impl Into<String>,
        style_fn: impl for<'a, 'w, 's> FnOnce(ImmEntity<'a, 'w, 's, Cap>) -> ImmEntity<'a, 'w, 's, Cap>,
    ) -> Self;
    fn with_icon(self, icon: lucide_icons::Icon) -> Self;
}

impl<Cap> ImmUiButton<Cap> for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityButton>
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiTextStyle>
        + ImplCap<CapabilityUiText>
        + ImplCap<CapabilityUiImage>
        + CapSet,
{
    fn button(mut self) -> Self {
        // 1. Apply Structure & Logic
        self = self.on_spawn_insert(|| {
            (
                Button,
                Node::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            )
        });

        // 2. Apply Base CSS
        self = style_btn_primary(self);

        // 3. Apply Dynamic CSS
        if let Ok(Some(interaction)) = self.cap_get_component::<Interaction>() {
            match *interaction {
                Interaction::Hovered => {
                    self = style_btn_primary_hover(self);
                }
                _ => {}
            }
        }

        self
    }

    fn icon_button(self) -> Self {
        // Just setup the container layout here.
        // We defer content injection to the chainable methods.
        self.button().style(|s| {
            s.column_gap = Val::Px(8.0);
            // Ensure items align correctly regardless of order
            s.align_items = AlignItems::Center;
        })
    }

    fn with_icon(self, icon: lucide_icons::Icon) -> Self {
        self.add(|ui| {
            ui.ch().icon(icon).text_base().text_color(Color::WHITE);
        })
    }

    // 1. Basic version (uses defaults)
    fn with_label(self, text: impl Into<String>) -> Self {
        self.with_styled_label(text, |ui| ui)
    }

    // 2. Advanced version (allows "prop" overrides)
    fn with_styled_label(
        self,
        text: impl Into<String>,
        style_fn: impl for<'a, 'w, 's> FnOnce(ImmEntity<'a, 'w, 's, Cap>) -> ImmEntity<'a, 'w, 's, Cap>,
    ) -> Self {
        let text = text.into();
        self.add(|ui| {
            let label = ui
                .ch()
                .label(text)
                .text_sm()
                .font_bold()
                .text_color(Color::WHITE);

            // Apply the user's overrides
            style_fn(label);
        })
    }
}
