use bevy::prelude::*;
use lucide_icons::LUCIDE_FONT_BYTES;

#[derive(Resource)]
pub struct LucideAssets {
    pub font: Handle<Font>,
}

impl FromWorld for LucideAssets {
    fn from_world(world: &mut World) -> Self {
        let mut fonts = world.resource_mut::<Assets<Font>>();
        // Load the raw bytes directly from the crate constant
        let font = Font::try_from_bytes(LUCIDE_FONT_BYTES.to_vec())
            .expect("Failed to load Lucide font bytes");

        Self {
            font: fonts.add(font),
        }
    }
}
