use bevy::prelude::*;

pub mod bottom_bar;
pub mod character;
pub mod content;
pub mod layout;
pub mod sidebar;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        bottom_bar::plugin,
        character::plugin,
        content::plugin,
        sidebar::plugin,
        layout::plugin,
    ));
}

pub use layout::spawn_game_layout;
