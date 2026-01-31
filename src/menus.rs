//! The game's menus and transitions between them.

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Menu>();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum Menu {
    #[default]
    None,
    Main,
    Settings,
    Credits,
    Pause,
}
