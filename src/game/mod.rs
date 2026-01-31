use bevy::prelude::*;

pub mod character;
pub mod resources;
pub mod save;
pub mod scenarios;
pub mod systems;
pub mod ui;

pub fn plugin(app: &mut App) {
    app.init_resource::<resources::GameState>();
    app.init_resource::<resources::PlayerState>();
    app.init_resource::<resources::SquadState>();

    app.register_type::<resources::GameState>();
    app.register_type::<resources::PlayerState>();
    app.register_type::<resources::SquadState>();

    app.add_plugins(save::SaveLoadPlugin);
    app.add_plugins(ui::plugin);
}
