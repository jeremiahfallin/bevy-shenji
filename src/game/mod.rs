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
    app.add_systems(Update, update_squad_statuses);

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
    // Copy unlogged notifications to event log exactly once
    for notification in notifications.notifications.iter_mut() {
        if !notification.logged {
            event_log.push(&notification.message, notification.level, sim.game_time);
            notification.logged = true;
        }
    }
    notifications.tick(time.delta_secs());
}

fn update_squad_statuses(
    mut squad_state: ResMut<resources::SquadState>,
    action_query: Query<&crate::game::action::ActionState>,
) {
    let characters = squad_state.characters.clone();
    for squad in squad_state.squads.values_mut() {
        let mut idle_count = 0u32;
        let mut traveling_count = 0u32;
        let mut active_count = 0u32;

        for member_id in &squad.members {
            if let Some(&entity) = characters.get(member_id) {
                if let Ok(action_state) = action_query.get(entity) {
                    match &action_state.current_action {
                        None | Some(crate::game::action::Action::Idle) => idle_count += 1,
                        Some(crate::game::action::Action::Travel { .. }) => traveling_count += 1,
                        _ => active_count += 1,
                    }
                }
            }
        }

        squad.status = if traveling_count >= idle_count && traveling_count >= active_count {
            resources::SquadStatus::Traveling
        } else if active_count > 0 {
            resources::SquadStatus::Active
        } else {
            resources::SquadStatus::Idle
        };
    }
}
