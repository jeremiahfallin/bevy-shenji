use bevy::prelude::*;

pub mod action;
pub mod building;
pub mod character;
pub mod data;
pub mod location;
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
    app.init_resource::<resources::EventLog>();
    app.init_resource::<resources::BaseInventory>();
    app.init_resource::<resources::ExplorationState>();
    app.init_resource::<research::ResearchState>();
    app.init_resource::<resources::BaseState>();

    app.register_type::<research::ResearchState>();
    app.register_type::<resources::GameState>();
    app.register_type::<resources::PlayerState>();
    app.register_type::<resources::SquadState>();
    app.register_type::<resources::NotificationState>();
    app.register_type::<resources::EventLog>();
    app.register_type::<resources::BaseInventory>();
    app.register_type::<resources::ExplorationState>();

    app.add_systems(Update, tick_notifications);

    data::plugin(app);
    action::plugin(app);
    building::plugin(app);
    location::plugin(app);
    simulation::plugin(app);
    app.add_plugins(save::plugin);
    app.add_plugins(ui::plugin);
}

fn tick_notifications(
    time: Res<Time>,
    mut notifications: ResMut<resources::NotificationState>,
    mut event_log: ResMut<resources::EventLog>,
    sim: Res<simulation::SimulationState>,
) {
    // Copy new notifications to event log before ticking
    for notification in &notifications.notifications {
        // Only copy notifications that are brand new (ttl close to 4.0)
        if notification.ttl > 3.9 {
            event_log.push(&notification.message, notification.level, sim.game_time);
        }
    }
    notifications.tick(time.delta_secs());
}
