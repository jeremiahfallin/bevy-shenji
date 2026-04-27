use bevy::prelude::*;
use bevy::ui::BoxShadow;
use bevy_immediate::ui::CapsUi;
use bevy_immediate::{CapSet, ImmCapAccessRequests, ImmCapability, ImmEntity, ImplCap};

use super::style::{CapabilityUiLayout, ImmUiStyleExt};

// 1. Define the Capability
pub struct CapabilityUiVisuals;

impl ImmCapability for CapabilityUiVisuals {
    fn build<Cap: CapSet>(app: &mut bevy::app::App, cap_req: &mut ImmCapAccessRequests<Cap>) {
        cap_req.request_component_write::<BackgroundColor>(app.world_mut());
        cap_req.request_component_write::<BorderColor>(app.world_mut());
        cap_req.request_component_write::<Outline>(app.world_mut());
        cap_req.request_component_write::<ZIndex>(app.world_mut());
        cap_req.request_component_write::<BoxShadow>(app.world_mut());
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

    fn z_index(self, val: i32) -> Self;
    fn z_index_global(self, val: i32) -> Self;

    // Outline
    fn outline(self, width: f32, color: impl Into<Color>) -> Self;
    fn outline_width(self, width: f32) -> Self;
    fn outline_color(self, color: impl Into<Color>) -> Self;
    fn outline_offset(self, offset: f32) -> Self;
    fn outline_none(self) -> Self;

    // Box Shadow
    fn shadow(self, x: f32, y: f32, blur: f32, spread: f32, color: impl Into<Color>) -> Self;
    fn shadow_sm(self) -> Self;
    fn shadow_md(self) -> Self;
    fn shadow_lg(self) -> Self;
    fn shadow_none(self) -> Self;
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

    fn rounded(self, val: f32) -> Self {
        self.style(|n| n.border_radius = BorderRadius::all(Val::Px(val)))
    }

    fn rounded_md(self) -> Self {
        self.rounded(6.0)
    }
    fn rounded_full(self) -> Self {
        self.style(|n| n.border_radius = BorderRadius::all(Val::Percent(50.0)))
    }

    fn opacity(mut self, val: f32) -> Self {
        // Simple alpha modification for background
        if let Ok(Some(mut bg)) = self.cap_get_component_mut::<BackgroundColor>() {
            bg.0.set_alpha(val);
        }
        self
    }

    fn z_index(mut self, val: i32) -> Self {
        if let Ok(Some(mut z)) = self.cap_get_component_mut::<ZIndex>() {
            *z = ZIndex(val);
        } else {
            self.entity_commands().insert(ZIndex(val));
        }
        self
    }

    fn z_index_global(mut self, val: i32) -> Self {
        if let Ok(Some(mut z)) = self.cap_get_component_mut::<GlobalZIndex>() {
            *z = GlobalZIndex(val);
        } else {
            self.entity_commands().insert(GlobalZIndex(val));
        }
        self
    }

    // --- Outline ---
    fn outline(mut self, width: f32, color: impl Into<Color>) -> Self {
        let c = color.into();
        if let Ok(Some(mut o)) = self.cap_get_component_mut::<Outline>() {
            o.width = Val::Px(width);
            o.color = c;
        } else {
            self.entity_commands()
                .insert(Outline::new(Val::Px(width), Val::ZERO, c));
        }
        self
    }

    fn outline_width(mut self, width: f32) -> Self {
        if let Ok(Some(mut o)) = self.cap_get_component_mut::<Outline>() {
            o.width = Val::Px(width);
        } else {
            self.entity_commands()
                .insert(Outline::new(Val::Px(width), Val::ZERO, Color::WHITE));
        }
        self
    }

    fn outline_color(mut self, color: impl Into<Color>) -> Self {
        let c = color.into();
        if let Ok(Some(mut o)) = self.cap_get_component_mut::<Outline>() {
            o.color = c;
        } else {
            self.entity_commands()
                .insert(Outline::new(Val::Px(1.0), Val::ZERO, c));
        }
        self
    }

    fn outline_offset(mut self, offset: f32) -> Self {
        if let Ok(Some(mut o)) = self.cap_get_component_mut::<Outline>() {
            o.offset = Val::Px(offset);
        } else {
            self.entity_commands().insert(Outline::new(
                Val::Px(1.0),
                Val::Px(offset),
                Color::WHITE,
            ));
        }
        self
    }

    fn outline_none(mut self) -> Self {
        if let Ok(Some(mut o)) = self.cap_get_component_mut::<Outline>() {
            o.color = Color::NONE;
            o.width = Val::ZERO;
        }
        self
    }

    // --- Box Shadow ---
    fn shadow(mut self, x: f32, y: f32, blur: f32, spread: f32, color: impl Into<Color>) -> Self {
        let shadow = BoxShadow::new(
            color.into(),
            Val::Px(x),
            Val::Px(y),
            Val::Px(spread),
            Val::Px(blur),
        );
        if let Ok(Some(mut bs)) = self.cap_get_component_mut::<BoxShadow>() {
            *bs = shadow;
        } else {
            self.entity_commands().insert(shadow);
        }
        self
    }

    fn shadow_sm(self) -> Self {
        self.shadow(0.0, 1.0, 2.0, 0.0, Color::srgba(0.0, 0.0, 0.0, 0.05))
    }

    fn shadow_md(self) -> Self {
        self.shadow(0.0, 4.0, 6.0, -1.0, Color::srgba(0.0, 0.0, 0.0, 0.1))
    }

    fn shadow_lg(self) -> Self {
        self.shadow(0.0, 10.0, 15.0, -3.0, Color::srgba(0.0, 0.0, 0.0, 0.1))
    }

    fn shadow_none(mut self) -> Self {
        if let Ok(Some(mut bs)) = self.cap_get_component_mut::<BoxShadow>() {
            bs.0.clear();
        }
        self
    }
}
