use bevy::prelude::*;

pub mod character;
pub mod research;
pub mod resources;
pub mod save;
pub mod scenarios;
pub mod systems;
pub mod ui;

pub fn plugin(app: &mut App) {
    app.init_resource::<resources::GameState>();
    app.init_resource::<resources::PlayerState>();
    app.init_resource::<resources::SquadState>();
    app.init_resource::<research::TechTree>();
    app.init_resource::<research::ResearchState>();
    app.init_resource::<resources::BaseState>();

    app.register_type::<research::ResearchState>();
    app.register_type::<resources::GameState>();
    app.register_type::<resources::PlayerState>();
    app.register_type::<resources::SquadState>();

    app.add_plugins(save::SaveLoadPlugin);
    app.add_plugins(ui::plugin);
}
