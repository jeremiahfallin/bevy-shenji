use bevy::prelude::*;
use bevy_immediate::{ImmEntity, ImplCap};
use lucide_icons::Icon;

// Reuse your existing text capabilities
use crate::theme::primitives::text::CapabilityUiTextStyle;
use crate::theme::widgets::label::ImmUiLabel;

// Marker component to trigger the font replacement
#[derive(Component)]
pub struct LucideIcon;

pub trait ImmUiIconExt {
    fn icon(self, icon: Icon) -> Self;
}

impl<Cap> ImmUiIconExt for ImmEntity<'_, '_, '_, Cap>
where
    // We rely on Label to set up the Text components
    Cap: ImplCap<CapabilityUiTextStyle> + ImplCap<bevy_immediate::ui::text::CapabilityUiText>,
{
    fn icon(self, icon: Icon) -> Self {
        let glyph = char::from(icon);
        let mut entity = self.label(glyph.to_string());
        entity.entity_commands().insert(LucideIcon);
        entity
    }
}
