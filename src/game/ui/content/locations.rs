//! Locations view — exploration stats, send-explorer panel, list of
//! discovered locations.
//!
//! Spawned once on `On<Add, LocationsView>`; rebuilt whenever any source
//! changes. One click handler ("Explore" button per idle character at base)
//! preserved via an `ExploreButton(Entity)` marker + observer system.

use bevy::prelude::*;
use bevy_declarative::prelude::px;

use crate::game::action::{Action, ActionState};
use crate::game::character::{CharacterInfo, CharacterLocation};
use crate::game::location::{LocationInfo, LocationResources, LocationType};
use crate::game::resources::{ExplorationState, NotificationLevel, NotificationState};
use crate::screens::Screen;
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(populate_locations_on_add);
    app.add_systems(
        Update,
        refresh_locations.run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component)]
pub struct LocationsView;

#[derive(Component)]
struct ExploreButton {
    entity: Entity,
    character_name: String,
}

fn explore_on_click(
    click: On<Pointer<Click>>,
    q: Query<&ExploreButton>,
    mut action_query: Query<&mut ActionState>,
    mut notifications: ResMut<NotificationState>,
) {
    if let Ok(marker) = q.get(click.entity) {
        if let Ok(mut action_state) = action_query.get_mut(marker.entity) {
            action_state.queue_action(Action::Explore);
            notifications.push(
                format!("{} sent to explore!", marker.character_name),
                NotificationLevel::Info,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn populate_locations_on_add(
    add: On<Add, LocationsView>,
    mut commands: Commands,
    location_query: Query<(&LocationInfo, &LocationResources)>,
    character_query: Query<(Entity, &CharacterInfo, &ActionState, &CharacterLocation)>,
    exploration_state: Res<ExplorationState>,
) {
    spawn_locations_children(
        &mut commands,
        add.entity,
        &location_query,
        &character_query,
        &exploration_state,
    );
}

#[allow(clippy::too_many_arguments)]
fn refresh_locations(
    mut commands: Commands,
    view_q: Query<Entity, With<LocationsView>>,
    location_query: Query<(&LocationInfo, &LocationResources)>,
    character_query: Query<(Entity, &CharacterInfo, &ActionState, &CharacterLocation)>,
    exploration_state: Res<ExplorationState>,
    changed_locs: Query<(), Or<(Changed<LocationInfo>, Changed<LocationResources>)>>,
    changed_chars: Query<
        (),
        Or<(
            Changed<CharacterInfo>,
            Changed<ActionState>,
            Changed<CharacterLocation>,
        )>,
    >,
) {
    let any_changed = exploration_state.is_changed()
        || !changed_locs.is_empty()
        || !changed_chars.is_empty();
    if !any_changed {
        return;
    }
    for view in &view_q {
        commands.entity(view).despawn_related::<Children>();
        spawn_locations_children(
            &mut commands,
            view,
            &location_query,
            &character_query,
            &exploration_state,
        );
    }
}

fn spawn_locations_children(
    commands: &mut Commands,
    view: Entity,
    location_query: &Query<(&LocationInfo, &LocationResources)>,
    character_query: &Query<(Entity, &CharacterInfo, &ActionState, &CharacterLocation)>,
    exploration_state: &ExplorationState,
) {
    let mut root = div()
        .col()
        .w_full()
        .p(px(SPACE_2_5))
        .gap_y(px(SPACE_2_5))
        .child(heading_2("Locations"))
        .child(exploration_stats_section(location_query, exploration_state))
        .child(send_explorer_section(character_query));

    let mut locations: Vec<_> = location_query
        .iter()
        .filter(|(info, _)| info.discovered)
        .collect();
    locations.sort_by_key(|(info, _)| info.distance);

    if locations.is_empty() {
        root = root.child(label("No locations discovered yet").color(Color::srgb(0.5, 0.5, 0.5)));
    } else {
        root = root.child(heading_3("Discovered Locations").mb(px(SPACE_1)));
        for (info, resources) in &locations {
            root = root.child(location_card(info, resources, character_query));
        }
    }

    let scroll = scroll_view().vertical(true).child(root);
    let scroll_entity = scroll.spawn(commands).id();
    commands.entity(view).add_child(scroll_entity);
}

fn exploration_stats_section(
    location_query: &Query<(&LocationInfo, &LocationResources)>,
    exploration_state: &ExplorationState,
) -> Div {
    let mut section = div()
        .col()
        .w_full()
        .p(px(SPACE_2))
        .bg(GRAY_800)
        .mb(px(SPACE_1))
        .rounded(Val::Px(4.0))
        .child(heading_3("Exploration"))
        .child(
            text(format!(
                "Total explorations: {}",
                exploration_state.total_explorations
            ))
            .font_size(12.0)
            .color(GRAY_300),
        );

    if !exploration_state.generated_nodes.is_empty() {
        let mut nodes: Vec<_> = exploration_state.generated_nodes.iter().collect();
        nodes.sort_by_key(|(k, _)| (*k).clone());
        for (resource_type, count) in nodes {
            section = section.child(
                text(format!(
                    "{} nodes: {}/{}",
                    resource_type,
                    count,
                    ExplorationState::MAX_GENERATED_PER_TYPE
                ))
                .font_size(11.0)
                .color(GRAY_400),
            );
        }
    }

    let undiscovered = location_query
        .iter()
        .filter(|(info, _)| !info.discovered)
        .count();
    if undiscovered > 0 {
        section = section.child(
            text(format!(
                "{} location{} remain undiscovered",
                undiscovered,
                if undiscovered == 1 { "" } else { "s" }
            ))
            .font_size(11.0)
            .color(Color::srgb(0.8, 0.7, 0.3)),
        );
    }
    section
}

fn send_explorer_section(
    character_query: &Query<(Entity, &CharacterInfo, &ActionState, &CharacterLocation)>,
) -> Div {
    let mut section = div()
        .col()
        .w_full()
        .p(px(SPACE_2))
        .bg(Color::srgb(0.12, 0.15, 0.2))
        .mb(px(SPACE_1))
        .rounded(Val::Px(4.0))
        .child(heading_3("Send Explorer"));

    let exploring: Vec<String> = character_query
        .iter()
        .filter(|(_, _, action_state, _)| {
            matches!(&action_state.current_action, Some(Action::Explore))
        })
        .map(|(_, info, _, _)| info.name.clone())
        .collect();
    if !exploring.is_empty() {
        section = section.child(
            text(format!("Exploring: {}", exploring.join(", ")))
                .font_size(11.0)
                .color(Color::srgb(0.4, 0.7, 0.9))
                .mb(px(SPACE_1)),
        );
    }

    let idle_at_base: Vec<(Entity, String)> = character_query
        .iter()
        .filter(|(_, _, action_state, loc)| {
            loc.location_id == "base"
                && matches!(
                    &action_state.current_action,
                    None | Some(Action::Idle)
                )
        })
        .map(|(e, info, _, _)| (e, info.name.clone()))
        .collect();

    if idle_at_base.is_empty() {
        section = section.child(
            text("No idle characters at base")
                .font_size(11.0)
                .color(GRAY_500),
        );
    } else {
        for (entity, name) in idle_at_base {
            section = section.child(
                div()
                    .flex()
                    .row()
                    .w_full()
                    .justify_between()
                    .items_center()
                    .mb(px(SPACE_0_5))
                    .child(text(name.clone()).font_size(12.0).color(Color::WHITE))
                    .child(
                        div()
                            .pad_x(px(SPACE_3))
                            .py(px(SPACE_1))
                            .bg(PRIMARY_500)
                            .rounded(Val::Px(4.0))
                            .insert(ExploreButton {
                                entity,
                                character_name: name,
                            })
                            .on_click(explore_on_click)
                            .child(text("Explore").font_size(11.0).color(Color::WHITE)),
                    ),
            );
        }
    }
    section
}

fn location_card(
    info: &LocationInfo,
    resources: &LocationResources,
    character_query: &Query<(Entity, &CharacterInfo, &ActionState, &CharacterLocation)>,
) -> Div {
    let type_str = match info.loc_type {
        LocationType::Base => "Base",
        LocationType::Mine => "Mine",
        LocationType::Forest => "Forest",
        LocationType::Ruins => "Ruins",
        LocationType::City => "City",
        LocationType::Wilderness => "Wilderness",
    };

    let mut card = div()
        .col()
        .w_full()
        .p(px(SPACE_2))
        .bg(GRAY_800)
        .mb(px(SPACE_1))
        .rounded(Val::Px(4.0))
        .child(
            div()
                .flex()
                .row()
                .justify_between()
                .w_full()
                .mb(px(SPACE_1))
                .child(text(info.name.clone()).color(Color::WHITE))
                .child(text(type_str).color(Color::srgb(0.6, 0.6, 0.6))),
        );

    if info.distance > 0 {
        card = card.child(
            text(format!("Distance: {}", info.distance))
                .color(Color::srgb(0.7, 0.7, 0.7))
                .mb(px(SPACE_0_5)),
        );
    }

    if !resources.resource_type.is_empty() && resources.capacity > 0 {
        card = card.child(
            div()
                .flex()
                .row()
                .w_full()
                .mb(px(SPACE_0_5))
                .child(
                    text(format!(
                        "{}: {}/{}",
                        resources.resource_type, resources.current_amount, resources.capacity
                    ))
                    .color(Color::srgb(0.8, 0.8, 0.5)),
                )
                .child(
                    text(format!(" (yield: {}/tick)", resources.yield_rate))
                        .color(Color::srgb(0.5, 0.7, 0.5)),
                ),
        );
    }

    let chars_here: Vec<_> = character_query
        .iter()
        .filter(|(_, _, _, loc)| loc.location_id == info.id)
        .map(|(_, ci, _, _)| ci.name.clone())
        .collect();
    if !chars_here.is_empty() {
        card = card.child(
            text(format!("Characters: {}", chars_here.join(", ")))
                .color(Color::srgb(0.5, 0.8, 0.5)),
        );
    }
    card
}
