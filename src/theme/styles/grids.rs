use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy::ui::{GridTrack, RepeatedGridTrack};
use bevy_immediate::{CapSet, ImmEntity, ImplCap};

pub fn style_grid_2col<'w, 's, 'a, Cap>(
    entity: ImmEntity<'w, 's, 'a, Cap>,
) -> ImmEntity<'w, 's, 'a, Cap>
where
    Cap: CapSet + ImplCap<CapabilityUiVisuals> + ImplCap<CapabilityUiLayout>,
{
    entity
        .grid()
        .grid_template_columns(vec![
            RepeatedGridTrack::auto(1),
            RepeatedGridTrack::flex(1, 1.0),
        ])
        .row_gap(SPACE_2_5)
        .column_gap(SPACE_5)
}
