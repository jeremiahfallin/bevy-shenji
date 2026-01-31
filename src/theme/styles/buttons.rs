use super::palette::*;
use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::ui::base::CapabilityUiBase;
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::{CapSet, ImmEntity, ImplCap};

/// Applies the visual style of a Primary Button.
pub fn style_btn_primary<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiTextStyle>,
{
    entity
        .bg(PRIMARY_500)
        .h(Val::Px(40.0))
        .px(Val::Px(16.0))
        .rounded(6.0)
        .flex_row()
        .items_center()
        .justify_center()
        .text_color(Color::WHITE)
        .text_sm()
        .font_bold()
}

/// Applies the hover style.
pub fn style_btn_primary_hover<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity.bg(PRIMARY_600)
}
