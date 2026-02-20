use super::palette::*;
use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::{CapSet, ImmEntity, ImplCap};

pub fn style_panel_central<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity.flex_col().w_full().h_full().bg(GRAY_900)
}

pub fn style_sidebar<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity.w(Val::Px(250.0)).h_full().bg(GRAY_800)
}

pub fn style_bottom_bar<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity.w_full().h(Val::Px(50.0)).bg(GRAY_700)
}

pub fn style_card<'w, 's, 'a, Cap>(entity: ImmEntity<'w, 's, 'a, Cap>) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity
        .flex_col()
        .w(Val::Percent(50.0))
        .p(Val::Px(SPACE_2_5))
        .rounded(8.0)
        .bg(GRAY_700)
}
