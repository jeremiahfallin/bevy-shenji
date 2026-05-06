//! Buildings view — under-construction list, completed list, and a
//! categorized blueprint catalog with a Build button per blueprint.
//!
//! Spawned once on `On<Add, BuildingsView>`; rebuilt whenever any source
//! changes. The Build button uses a `BuildBlueprintButton(def_id)` marker;
//! the click observer looks the building def up in `GameData` at click time
//! to deduct cost, spawn the building entity, assign an idle worker, and
//! push a notification.

use bevy::prelude::*;
use bevy_declarative::prelude::px;

use crate::game::action::{Action, ActionState};
use crate::game::building::Building;
use crate::game::character::{CharacterInfo, CharacterLocation};
use crate::game::data::GameData;
use crate::game::research::ResearchState;
use crate::game::resources::{BaseInventory, BaseState, NotificationLevel, NotificationState};
use crate::screens::Screen;
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(populate_buildings_on_add);
    app.add_systems(
        Update,
        refresh_buildings.run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component)]
pub struct BuildingsView;

#[derive(Component)]
struct BuildBlueprintButton(String);

#[allow(clippy::too_many_arguments)]
fn build_blueprint_on_click(
    click: On<Pointer<Click>>,
    q: Query<&BuildBlueprintButton>,
    mut commands: Commands,
    game_data: Res<GameData>,
    mut base_inv: ResMut<BaseInventory>,
    mut action_query: Query<&mut ActionState>,
    char_query: Query<(Entity, &CharacterInfo, &CharacterLocation)>,
    mut notifications: ResMut<NotificationState>,
) {
    let Ok(marker) = q.get(click.entity) else {
        return;
    };
    let Some(def) = game_data.get_building(&marker.0) else {
        return;
    };

    // Re-verify affordability at click time.
    let can_afford = def
        .cost
        .iter()
        .all(|(item, &amount)| base_inv.count(item) >= amount);
    if !can_afford {
        return;
    }

    // Find an idle worker at base. Pull ActionState via the &mut query as
    // an immutable read here; using a separate `&ActionState` query would
    // conflict with `action_query` (B0001).
    let assignee: Option<(Entity, String)> = char_query
        .iter()
        .find(|(e, _, loc)| {
            if loc.location_id != "base" {
                return false;
            }
            action_query
                .get(*e)
                .map(|s| matches!(&s.current_action, None | Some(Action::Idle)))
                .unwrap_or(false)
        })
        .map(|(e, info, _)| (e, info.name.clone()));
    let Some((worker_entity, worker_name)) = assignee else {
        return;
    };

    // Deduct costs.
    for (item, &amount) in &def.cost {
        if !base_inv.remove(item, amount) {
            return;
        }
    }

    commands.spawn(Building {
        def_id: def.id.clone(),
        name: def.name.clone(),
        progress: 0,
        required: def.build_time,
        complete: false,
    });
    if let Ok(mut action_state) = action_query.get_mut(worker_entity) {
        action_state.queue_action(Action::Build {
            building: def.id.clone(),
        });
    }
    notifications.push(
        format!("{} started building {}", worker_name, def.name),
        NotificationLevel::Info,
    );
}

#[allow(clippy::too_many_arguments)]
fn populate_buildings_on_add(
    add: On<Add, BuildingsView>,
    mut commands: Commands,
    game_data: Res<GameData>,
    research_state: Res<ResearchState>,
    base_inventory: Res<BaseInventory>,
    base_state: Res<BaseState>,
    buildings_query: Query<&Building>,
    character_query: Query<(Entity, &CharacterInfo, &ActionState, &CharacterLocation)>,
) {
    spawn_buildings_children(
        &mut commands,
        add.entity,
        &game_data,
        &research_state,
        &base_inventory,
        &base_state,
        &buildings_query,
        &character_query,
    );
}

#[allow(clippy::too_many_arguments)]
fn refresh_buildings(
    mut commands: Commands,
    view_q: Query<Entity, With<BuildingsView>>,
    game_data: Res<GameData>,
    research_state: Res<ResearchState>,
    base_inventory: Res<BaseInventory>,
    base_state: Res<BaseState>,
    buildings_query: Query<&Building>,
    character_query: Query<(Entity, &CharacterInfo, &ActionState, &CharacterLocation)>,
    changed_buildings: Query<(), Changed<Building>>,
    changed_chars: Query<
        (),
        Or<(
            Changed<CharacterInfo>,
            Changed<ActionState>,
            Changed<CharacterLocation>,
        )>,
    >,
) {
    let any_changed = game_data.is_changed()
        || research_state.is_changed()
        || base_inventory.is_changed()
        || base_state.is_changed()
        || !changed_buildings.is_empty()
        || !changed_chars.is_empty();
    if !any_changed {
        return;
    }
    for view in &view_q {
        commands.entity(view).despawn_related::<Children>();
        spawn_buildings_children(
            &mut commands,
            view,
            &game_data,
            &research_state,
            &base_inventory,
            &base_state,
            &buildings_query,
            &character_query,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_buildings_children(
    commands: &mut Commands,
    view: Entity,
    game_data: &GameData,
    research_state: &ResearchState,
    base_inventory: &BaseInventory,
    base_state: &BaseState,
    buildings_query: &Query<&Building>,
    character_query: &Query<(Entity, &CharacterInfo, &ActionState, &CharacterLocation)>,
) {
    let mut root = div()
        .col()
        .w_full()
        .p(px(SPACE_2_5))
        .gap_y(px(SPACE_2_5))
        .child(heading_2("Buildings"));

    // Under construction.
    let in_progress: Vec<_> = buildings_query.iter().filter(|b| !b.complete).collect();
    if !in_progress.is_empty() {
        root = root.child(heading_3("Under Construction").mb(px(SPACE_1)));
        for building in &in_progress {
            let frac = if building.required > 0 {
                building.progress as f32 / building.required as f32
            } else {
                1.0
            };
            root = root.child(
                div()
                    .col()
                    .w_full()
                    .p(px(SPACE_2))
                    .bg(Color::srgb(0.15, 0.15, 0.3))
                    .mb(px(SPACE_1))
                    .rounded(Val::Px(4.0))
                    .child(text(building.name.clone()).color(Color::WHITE))
                    .child(
                        text(format!(
                            "{}/{} ticks ({:.0}%)",
                            building.progress,
                            building.required,
                            frac * 100.0
                        ))
                        .font_size(11.0)
                        .color(GRAY_400),
                    )
                    .child(progress_bar_div(frac)),
            );
        }
    }

    // Completed.
    let completed: Vec<_> = buildings_query.iter().filter(|b| b.complete).collect();
    if !completed.is_empty() {
        root = root.child(heading_3("Completed").mb(px(SPACE_1)));
        for building in &completed {
            let description = game_data
                .get_building(&building.def_id)
                .map(|d| d.description.as_str())
                .unwrap_or("");
            let mut card = div()
                .col()
                .w_full()
                .p(px(SPACE_2))
                .bg(Color::srgb(0.15, 0.3, 0.15))
                .mb(px(SPACE_1))
                .rounded(Val::Px(4.0))
                .child(text(building.name.clone()).color(Color::WHITE));
            if !description.is_empty() {
                card = card.child(text(description.to_string()).font_size(11.0).color(GRAY_400));
            }
            root = root.child(card);
        }
    }

    // Available blueprints.
    root = root.child(heading_3("Available Blueprints").mb(px(SPACE_1)));

    let categories = [
        ("tech", "Technology"),
        ("food", "Food"),
        ("storage", "Storage"),
        ("power", "Power"),
        ("training", "Training"),
        ("defense", "Defense"),
    ];
    let current_tech = base_state.value.tech_level.max(1);
    let idle_at_base_present = character_query.iter().any(|(_, _, action_state, loc)| {
        loc.location_id == "base"
            && matches!(&action_state.current_action, None | Some(Action::Idle))
    });

    for (category_id, category_name) in &categories {
        let mut buildings_in_cat: Vec<_> = game_data
            .buildings
            .values()
            .filter(|def| def.category == *category_id && def.tech_level <= current_tech)
            .collect();
        if buildings_in_cat.is_empty() {
            continue;
        }
        buildings_in_cat.sort_by_key(|def| &def.name);

        root = root.child(
            text((*category_name).to_string())
                .color(GRAY_300)
                .mb(px(SPACE_0_5)),
        );

        for def in &buildings_in_cat {
            let research_met = def
                .required_research
                .iter()
                .all(|r| research_state.is_unlocked(r));
            let can_afford = def
                .cost
                .iter()
                .all(|(item, &amount)| base_inventory.count(item) >= amount);
            let bg_color = if !research_met { GRAY_900 } else { GRAY_800 };

            let mut card = div()
                .col()
                .w_full()
                .p(px(SPACE_2))
                .bg(bg_color)
                .mb(px(SPACE_1))
                .rounded(Val::Px(4.0))
                .child(text(def.name.clone()).color(Color::WHITE))
                .child(
                    text(def.description.clone())
                        .font_size(11.0)
                        .color(GRAY_400),
                );

            // Cost.
            let mut cost_parts: Vec<String> = def
                .cost
                .iter()
                .map(|(item, &amount)| {
                    let have = base_inventory.count(item);
                    format!("{}: {}/{}", item, have, amount)
                })
                .collect();
            cost_parts.sort();
            card = card.child(
                text(format!("Cost: {}", cost_parts.join(", ")))
                    .font_size(11.0)
                    .color(if can_afford { Color::WHITE } else { ERROR_500 }),
            );

            card = card.child(
                text(format!("Build time: {} ticks", def.build_time))
                    .font_size(11.0)
                    .color(GRAY_400),
            );

            if def.power_generation != 0 {
                let (power_text, power_color) = if def.power_generation > 0 {
                    (
                        format!("Generates: {} power", def.power_generation),
                        SUCCESS_600,
                    )
                } else {
                    (
                        format!("Consumes: {} power", def.power_generation.abs()),
                        Color::srgb(0.8, 0.6, 0.2),
                    )
                };
                card = card.child(text(power_text).font_size(11.0).color(power_color));
            }

            if def.provides_storage > 0 {
                card = card.child(
                    text(format!("Storage: +{} capacity", def.provides_storage))
                        .font_size(11.0)
                        .color(INFO_600),
                );
            }

            if !research_met {
                let missing: Vec<String> = def
                    .required_research
                    .iter()
                    .filter(|r| !research_state.is_unlocked(r))
                    .map(|r| {
                        game_data
                            .get_research(r)
                            .map(|rd| rd.name.clone())
                            .unwrap_or_else(|| r.clone())
                    })
                    .collect();
                card = card.child(
                    text(format!("Requires: {}", missing.join(", ")))
                        .font_size(10.0)
                        .color(ERROR_500),
                );
            }

            if research_met {
                let is_buildable = can_afford && idle_at_base_present;
                let button_label = if !can_afford {
                    "Need Resources"
                } else if !idle_at_base_present {
                    "No Idle Worker"
                } else {
                    "Build"
                };
                let button_bg = if is_buildable { PRIMARY_500 } else { GRAY_700 };

                card = card.child(
                    div()
                        .w_full()
                        .pad_x(px(SPACE_3))
                        .py(px(SPACE_2))
                        .mt(px(SPACE_1))
                        .bg(button_bg)
                        .rounded(Val::Px(4.0))
                        .insert(BuildBlueprintButton(def.id.clone()))
                        .on_click(build_blueprint_on_click)
                        .child(text(button_label).font_size(12.0).color(Color::WHITE)),
                );
            }

            root = root.child(card);
        }
    }

    let scroll = scroll_view().vertical(true).child(root);
    let scroll_entity = scroll.spawn(commands).id();
    commands.entity(view).add_child(scroll_entity);
}

/// Local progress-bar div replacing the theme widget's `progress_bar`.
fn progress_bar_div(frac: f32) -> Div {
    div()
        .w_full()
        .h(Val::Px(8.0))
        .bg(GRAY_800)
        .rounded(Val::Px(2.0))
        .child(
            div()
                .w(Val::Percent(frac.clamp(0.0, 1.0) * 100.0))
                .h(Val::Percent(100.0))
                .bg(SUCCESS_500)
                .rounded(Val::Px(2.0)),
        )
}
