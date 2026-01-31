use bevy::prelude::*;
use bevy_immediate::ui::CapsUi;
use bevy_immediate::ui::base::CapabilityUiBase;
use bevy_immediate::{CapSet, ImmCapAccessRequests, ImmCapability, ImmEntity, ImplCap};

use super::style::{CapabilityUiLayout, ImmUiStyleExt};

// 1. Define the Capability
pub struct CapabilityUiVisuals;

impl ImmCapability for CapabilityUiVisuals {
    fn build<Cap: CapSet>(app: &mut bevy::app::App, cap_req: &mut ImmCapAccessRequests<Cap>) {
        cap_req.request_component_write::<BackgroundColor>(app.world_mut());
        cap_req.request_component_write::<BorderColor>(app.world_mut());
        // In Bevy 0.15, Outline is its own component
        cap_req.request_component_write::<Outline>(app.world_mut());
        cap_req.request_component_write::<BorderRadius>(app.world_mut());
    }
}

impl ImplCap<CapabilityUiVisuals> for CapsUi {}

// 2. Define the Fluent API
pub trait ImmUiVisuals {
    fn bg(self, color: impl Into<Color>) -> Self;
    fn border_color(self, color: impl Into<Color>) -> Self;
    fn border(self, width: f32) -> Self;

    // Rounded modifies Style, so it needs CapabilityUiBase
    fn rounded(self, val: f32) -> Self;
    fn rounded_md(self) -> Self;
    fn rounded_full(self) -> Self;

    fn opacity(self, val: f32) -> Self;
}

impl<Cap> ImmUiVisuals for ImmEntity<'_, '_, '_, Cap>
where
    // Visuals often need both styling (radius) and colors
    Cap: ImplCap<CapabilityUiLayout> + ImplCap<CapabilityUiVisuals>,
{
    fn bg(mut self, color: impl Into<Color>) -> Self {
        if let Ok(Some(mut bg)) = self.cap_get_component_mut::<BackgroundColor>() {
            bg.0 = color.into();
        } else {
            self.entity_commands().insert(BackgroundColor(color.into()));
        }
        self
    }

    fn border_color(mut self, color: impl Into<Color>) -> Self {
        if let Ok(Some(mut border)) = self.cap_get_component_mut::<BorderColor>() {
            *border = BorderColor::all(color.into());
        } else {
            self.entity_commands()
                .insert(BorderColor::all(color.into()));
        }
        self
    }

    fn border(self, width: f32) -> Self {
        self.style(|s| s.border = UiRect::all(Val::Px(width)))
    }

    fn rounded(mut self, val: f32) -> Self {
        if let Ok(Some(mut radius)) = self.cap_get_component_mut::<BorderRadius>() {
            *radius = BorderRadius::all(Val::Px(val));
        } else {
            self.entity_commands()
                .insert(BorderRadius::all(Val::Px(val)));
        }
        self
    }

    fn rounded_md(self) -> Self {
        self.rounded(6.0)
    }
    fn rounded_full(mut self) -> Self {
        if let Ok(Some(mut radius)) = self.cap_get_component_mut::<BorderRadius>() {
            *radius = BorderRadius::all(Val::Percent(50.0));
        } else {
            self.entity_commands()
                .insert(BorderRadius::all(Val::Percent(50.0)));
        }
        self
    }

    fn opacity(mut self, val: f32) -> Self {
        // Simple alpha modification for background
        if let Ok(Some(mut bg)) = self.cap_get_component_mut::<BackgroundColor>() {
            bg.0.set_alpha(val);
        }
        self
    }
}
