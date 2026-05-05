//! Dashboard view — at-a-glance summary of resources, workers, research,
//! and recent notifications.
//!
//! Spawned once when the `DashboardView` marker entity is added (the Content
//! container creates that marker on first frame). Whenever any of its
//! source resources change, the dashboard's children are despawned and the
//! tree is rebuilt — the master design's stated update policy. This also
//! fixes the pre-migration bug where the Resources section retained stale
//! entries from previous renders, since rebuild guarantees a fresh tree.

use bevy::prelude::*;
use bevy_declarative::prelude::px;

use crate::game::action::{Action, ActionState};
use crate::game::research::ResearchState;
use crate::game::resources::{BaseInventory, NotificationLevel, NotificationState};
use crate::screens::Screen;
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(populate_dashboard_on_add);
    app.add_systems(Update, refresh_dashboard.run_if(in_state(Screen::Gameplay)));
}

#[derive(Component)]
pub struct DashboardView;

#[allow(clippy::too_many_arguments)]
fn populate_dashboard_on_add(
    add: On<Add, DashboardView>,
    mut commands: Commands,
    base_inv: Res<BaseInventory>,
    research_state: Res<ResearchState>,
    notif_state: Res<NotificationState>,
    action_query: Query<&ActionState>,
) {
    spawn_dashboard_children(
        &mut commands,
        add.entity,
        &base_inv,
        &research_state,
        &notif_state,
        &action_query,
    );
}

#[allow(clippy::too_many_arguments)]
fn refresh_dashboard(
    mut commands: Commands,
    dashboard_q: Query<Entity, With<DashboardView>>,
    base_inv: Res<BaseInventory>,
    research_state: Res<ResearchState>,
    notif_state: Res<NotificationState>,
    action_query: Query<&ActionState>,
    action_changed: Query<(), Changed<ActionState>>,
) {
    let any_changed = base_inv.is_changed()
        || research_state.is_changed()
        || notif_state.is_changed()
        || !action_changed.is_empty();
    if !any_changed {
        return;
    }
    for dashboard in &dashboard_q {
        commands.entity(dashboard).despawn_related::<Children>();
        spawn_dashboard_children(
            &mut commands,
            dashboard,
            &base_inv,
            &research_state,
            &notif_state,
            &action_query,
        );
    }
}

fn spawn_dashboard_children(
    commands: &mut Commands,
    dashboard: Entity,
    base_inv: &BaseInventory,
    research_state: &ResearchState,
    notif_state: &NotificationState,
    action_query: &Query<&ActionState>,
) {
    let header = heading_2("Dashboard");

    let body = div()
        .col()
        .w_full()
        .p(px(SPACE_2_5))
        .gap_y(px(SPACE_2_5))
        .child(resources_section(base_inv))
        .child(workers_section(action_query))
        .child(research_section(research_state))
        .child(notifications_section(notif_state));

    let header_entity = header.spawn(commands).id();
    let body_entity = body.spawn(commands).id();
    commands
        .entity(dashboard)
        .add_children(&[header_entity, body_entity]);
}

fn resources_section(base_inv: &BaseInventory) -> Div {
    let mut section = div()
        .col()
        .w_full()
        .child(text("Resources").color(Color::WHITE).insert(SectionHeader));

    if base_inv.items.is_empty() {
        section = section.child(label("No resources yet").color(Color::srgb(0.5, 0.5, 0.5)));
    } else {
        let mut items: Vec<_> = base_inv.items.iter().collect();
        items.sort_by_key(|(name, _)| (*name).clone());
        for (name, count) in items {
            section = section.child(
                div()
                    .flex()
                    .row()
                    .justify_between()
                    .w_full()
                    .mb(px(SPACE_0_5))
                    .child(label(name.clone()).color(Color::srgb(0.8, 0.8, 0.8)))
                    .child(text(format!("{}", count)).color(Color::WHITE)),
            );
        }
    }
    section
}

fn workers_section(action_query: &Query<&ActionState>) -> Div {
    let mut active = 0u32;
    let mut idle = 0u32;
    for action_state in action_query.iter() {
        match &action_state.current_action {
            None | Some(Action::Idle) => idle += 1,
            Some(_) => active += 1,
        }
    }
    div()
        .col()
        .w_full()
        .child(text("Workers").color(Color::WHITE).insert(SectionHeader))
        .child(label(format!(
            "Workers: {} active, {} idle",
            active, idle
        )))
}

fn research_section(research_state: &ResearchState) -> Div {
    let unlocked_count = research_state.unlocked.len();
    div()
        .col()
        .w_full()
        .child(text("Research").color(Color::WHITE).insert(SectionHeader))
        .child(label(format!(
            "Technologies unlocked: {}",
            unlocked_count
        )))
}

fn notifications_section(notif_state: &NotificationState) -> Div {
    let mut section = div().col().w_full().child(
        text("Notifications")
            .color(Color::WHITE)
            .insert(SectionHeader),
    );

    let notifs: Vec<_> = notif_state.notifications.iter().rev().take(5).collect();
    if notifs.is_empty() {
        section = section.child(label("No notifications").color(Color::srgb(0.5, 0.5, 0.5)));
    } else {
        for notif in notifs {
            let color = match notif.level {
                NotificationLevel::Info => INFO_600,
                NotificationLevel::Success => SUCCESS_600,
                NotificationLevel::Error => ERROR_600,
            };
            section = section
                .child(label(notif.message.clone()).color(color).mb(px(SPACE_0_5)));
        }
    }
    section
}

/// Marker on section-header text. Reserved for future styling hooks
/// (font weight, divider line); currently inert.
#[derive(Component)]
struct SectionHeader;
