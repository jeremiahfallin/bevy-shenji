use bevy::prelude::*;

pub mod bottom_bar;
pub mod character;
pub mod content;
pub mod context_menu;
pub mod inspector;
pub mod layout;
pub mod sidebar;

use crate::game::resources::UiState;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<UiState>();
    app.init_resource::<context_menu::ContextMenuState>();
    app.add_plugins((
        bottom_bar::plugin,
        character::plugin,
        content::plugin,
        context_menu::plugin,
        sidebar::plugin,
        layout::plugin,
    ));
}

pub use layout::spawn_game_layout;
