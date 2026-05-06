//! Lucide icon font wiring.
//!
//! - [`LucideIcon`] is a marker component placed on entities that should
//!   render with the Lucide icon font.
//! - [`LucideAssets`] holds the loaded `Handle<Font>` for the Lucide font.
//! - [`apply_lucide_font`] is an `On<Add, LucideIcon>` observer that swaps
//!   the entity's `TextFont::font` to the Lucide handle as soon as the
//!   marker is inserted.
//! - [`enforce_lucide_font`] runs in `PostUpdate` and re-asserts the font
//!   if something else (e.g. default text styling) overwrites it.
//!
//! Lifted from `src/theme/behaviors.rs` + `src/theme/resources.rs` +
//! `src/theme/widgets/icon.rs` so `src/ui/` no longer reaches across into
//! `src/theme/`. Phase C cleanup.

use bevy::prelude::*;
use lucide_icons::LUCIDE_FONT_BYTES;

pub fn plugin(app: &mut App) {
    app.init_resource::<LucideAssets>();
    app.add_observer(apply_lucide_font);
    app.add_systems(PostUpdate, enforce_lucide_font);
}

#[derive(Component)]
pub struct LucideIcon;

#[derive(Resource)]
pub struct LucideAssets {
    pub font: Handle<Font>,
}

impl FromWorld for LucideAssets {
    fn from_world(world: &mut World) -> Self {
        let mut fonts = world.resource_mut::<Assets<Font>>();
        let font = Font::try_from_bytes(LUCIDE_FONT_BYTES.to_vec())
            .expect("Failed to load Lucide font bytes");
        Self {
            font: fonts.add(font),
        }
    }
}

fn apply_lucide_font(
    trigger: On<Add, LucideIcon>,
    mut query: Query<&mut TextFont>,
    lucide_assets: Res<LucideAssets>,
) {
    if let Ok(mut font) = query.get_mut(trigger.entity) {
        font.font = lucide_assets.font.clone();
    }
}

fn enforce_lucide_font(
    mut query: Query<(&mut TextFont, &LucideIcon), Changed<TextFont>>,
    lucide_assets: Res<LucideAssets>,
) {
    for (mut font, _) in &mut query {
        if font.font != lucide_assets.font {
            font.font = lucide_assets.font.clone();
        }
    }
}
