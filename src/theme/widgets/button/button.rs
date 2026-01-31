use crate::theme::primitives::text::CapabilityUiTextStyle;
use bevy::prelude::*;
use bevy_immediate::ui::CapsUi;
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
pub trait ImmUiButton {
    fn button(self) -> Self;
}

impl<Cap> ImmUiButton for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityButton>
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiTextStyle>,
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
}
