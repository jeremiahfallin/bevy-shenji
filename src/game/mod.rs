use bevy::prelude::*;

pub mod action;
pub mod character;
pub mod research;
pub mod resources;
pub mod save;
pub mod scenarios;
pub mod simulation;
pub mod systems;
pub mod ui;

pub fn plugin(app: &mut App) {
    app.init_resource::<resources::GameState>();
    app.init_resource::<resources::PlayerState>();
    app.init_resource::<resources::SquadState>();
    app.init_resource::<resources::NotificationState>();
    app.init_resource::<research::TechTree>();
    app.init_resource::<research::ResearchState>();
    app.init_resource::<resources::BaseState>();

    app.register_type::<research::ResearchState>();
    app.register_type::<resources::GameState>();
    app.register_type::<resources::PlayerState>();
    app.register_type::<resources::SquadState>();
    app.register_type::<resources::NotificationState>();

    app.add_systems(Update, tick_notifications);

    action::plugin(app);
    simulation::plugin(app);
    app.add_plugins(save::plugin);
    app.add_plugins(ui::plugin);
}

fn tick_notifications(time: Res<Time>, mut notifications: ResMut<resources::NotificationState>) {
    notifications.tick(time.delta_secs());
}
