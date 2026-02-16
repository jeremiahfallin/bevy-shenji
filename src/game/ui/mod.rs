use bevy::prelude::*;

pub mod bottom_bar;
pub mod character;
pub mod content;
pub mod inspector;
pub mod layout;
pub mod sidebar;

use crate::game::resources::UiState;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<UiState>();
    app.add_plugins((
        bottom_bar::plugin,
        character::plugin,
        content::plugin,
        sidebar::plugin,
        layout::plugin,
    ));
}

pub use layout::spawn_game_layout;
