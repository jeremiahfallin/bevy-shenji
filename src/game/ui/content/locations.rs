use crate::game::action::{Action, ActionState};
use crate::game::character::{CharacterInfo, CharacterLocation};
use crate::game::location::{LocationInfo, LocationResources, LocationType};
use crate::game::resources::{ExplorationState, NotificationLevel, NotificationState};
use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::{Imm, attach::ImmediateAttach, ui::CapsUi};

#[derive(Component)]
pub struct LocationsView;

impl ImmediateAttach<CapsUi> for LocationsView {
    type Params = (
        Query<'static, 'static, (&'static LocationInfo, &'static LocationResources)>,
        Query<
            'static,
            'static,
            (
                Entity,
                &'static CharacterInfo,
                &'static ActionState,
                &'static CharacterLocation,
            ),
        >,
        Res<'static, ExplorationState>,
    );

    fn construct(
        ui: &mut Imm<CapsUi>,
        (location_query, character_query, exploration_state): &mut (
            Query<(&LocationInfo, &LocationResources)>,
            Query<(Entity, &CharacterInfo, &ActionState, &CharacterLocation)>,
            Res<ExplorationState>,
        ),
    ) {
        ui.ch().header("Locations");

        ui.ch()
            .flex_col()
            .w_full()
            .p(Val::Px(SPACE_2_5))
            .row_gap(SPACE_2_5)
            .scroll_y()
            .add(|ui| {
                // --- Exploration Stats ---
                ui.ch()
                    .flex_col()
                    .w_full()
                    .p(Val::Px(SPACE_2))
                    .bg(GRAY_800)
                    .mb(Val::Px(SPACE_1))
                    .rounded(4.0)
                    .add(|ui| {
                        ui.ch().sub_header("Exploration");

                        ui.ch()
                            .label(format!(
                                "Total explorations: {}",
                                exploration_state.total_explorations
                            ))
                            .text_size(12.0)
                            .text_color(GRAY_300);

                        // Generated node counts
                        if !exploration_state.generated_nodes.is_empty() {
                            let mut nodes: Vec<_> =
                                exploration_state.generated_nodes.iter().collect();
                            nodes.sort_by_key(|(k, _)| (*k).clone());

                            for (resource_type, count) in nodes {
                                ui.ch()
                                    .label(format!(
                                        "{} nodes: {}/{}",
                                        resource_type,
                                        count,
                                        ExplorationState::MAX_GENERATED_PER_TYPE
                                    ))
                                    .text_size(11.0)
                                    .text_color(GRAY_400);
                            }
                        }

                        // Undiscovered location hint
                        let undiscovered = location_query
                            .iter()
                            .filter(|(info, _)| !info.discovered)
                            .count();
                        if undiscovered > 0 {
                            ui.ch()
                                .label(format!(
                                    "{} location{} remain undiscovered",
                                    undiscovered,
                                    if undiscovered == 1 { "" } else { "s" }
                                ))
                                .text_size(11.0)
                                .text_color(Color::srgb(0.8, 0.7, 0.3));
                        }
                    });

                // --- Send Explorer ---
                ui.ch()
                    .flex_col()
                    .w_full()
                    .p(Val::Px(SPACE_2))
                    .bg(Color::srgb(0.12, 0.15, 0.2))
                    .mb(Val::Px(SPACE_1))
                    .rounded(4.0)
                    .add(|ui| {
                        ui.ch().sub_header("Send Explorer");

                        // Characters currently exploring
                        let exploring: Vec<String> = character_query
                            .iter()
                            .filter(|(_, _, action_state, _)| {
                                matches!(&action_state.current_action, Some(Action::Explore))
                            })
                            .map(|(_, info, _, _)| info.name.clone())
                            .collect();

                        if !exploring.is_empty() {
                            ui.ch()
                                .label(format!(
                                    "Exploring: {}",
                                    exploring.join(", ")
                                ))
                                .text_size(11.0)
                                .text_color(Color::srgb(0.4, 0.7, 0.9))
                                .mb(Val::Px(SPACE_1));
                        }

                        // Idle characters at base
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
                            ui.ch()
                                .label("No idle characters at base")
                                .text_size(11.0)
                                .text_color(GRAY_500);
                        } else {
                            for (entity, name) in idle_at_base {
                                let char_name = name.clone();
                                let char_name_notif = name.clone();
                                ui.ch()
                                    .flex_row()
                                    .w_full()
                                    .justify_between()
                                    .items_center()
                                    .mb(Val::Px(SPACE_0_5))
                                    .add(|ui| {
                                        ui.ch()
                                            .label(&char_name)
                                            .text_size(12.0)
                                            .text_color(Color::WHITE);

                                        ui.ch()
                                            .button()
                                            .on_click_once(
                                                move |_: On<Pointer<Click>>,
                                                      mut action_query: Query<
                                                    &mut ActionState,
                                                >,
                                                      mut notifications: ResMut<
                                                    NotificationState,
                                                >| {
                                                    if let Ok(mut action_state) =
                                                        action_query.get_mut(entity)
                                                    {
                                                        action_state
                                                            .queue_action(Action::Explore);
                                                        notifications.push(
                                                            format!(
                                                                "{} sent to explore!",
                                                                char_name_notif
                                                            ),
                                                            NotificationLevel::Info,
                                                        );
                                                    }
                                                },
                                            )
                                            .add(|ui| {
                                                ui.ch()
                                                    .label("Explore")
                                                    .text_size(11.0)
                                                    .text_color(Color::WHITE);
                                            });
                                    });
                            }
                        }
                    });

                // --- Discovered Locations ---
                let mut locations: Vec<_> = location_query
                    .iter()
                    .filter(|(info, _)| info.discovered)
                    .collect();
                locations.sort_by_key(|(info, _)| info.distance);

                if locations.is_empty() {
                    ui.ch()
                        .label("No locations discovered yet")
                        .text_color(Color::srgb(0.5, 0.5, 0.5));
                    return;
                }

                ui.ch()
                    .sub_header("Discovered Locations")
                    .mb(Val::Px(SPACE_1));

                for (info, resources) in &locations {
                    ui.ch()
                        .flex_col()
                        .w_full()
                        .p(Val::Px(SPACE_2))
                        .bg(GRAY_800)
                        .mb(Val::Px(SPACE_1))
                        .rounded(4.0)
                        .add(|ui| {
                            // Name and type
                            ui.ch()
                                .flex_row()
                                .justify_between()
                                .w_full()
                                .mb(Val::Px(SPACE_1))
                                .add(|ui| {
                                    ui.ch()
                                        .label(&info.name)
                                        .font_bold()
                                        .text_color(Color::WHITE);

                                    let type_str = match info.loc_type {
                                        LocationType::Base => "Base",
                                        LocationType::Mine => "Mine",
                                        LocationType::Forest => "Forest",
                                        LocationType::Ruins => "Ruins",
                                        LocationType::City => "City",
                                        LocationType::Wilderness => "Wilderness",
                                    };
                                    ui.ch()
                                        .label(type_str)
                                        .text_color(Color::srgb(0.6, 0.6, 0.6));
                                });

                            // Distance
                            if info.distance > 0 {
                                ui.ch()
                                    .label(format!("Distance: {}", info.distance))
                                    .text_color(Color::srgb(0.7, 0.7, 0.7))
                                    .mb(Val::Px(SPACE_0_5));
                            }

                            // Resources (if applicable)
                            if !resources.resource_type.is_empty() && resources.capacity > 0 {
                                ui.ch()
                                    .flex_row()
                                    .w_full()
                                    .mb(Val::Px(SPACE_0_5))
                                    .add(|ui| {
                                        ui.ch()
                                            .label(format!(
                                                "{}: {}/{}",
                                                resources.resource_type,
                                                resources.current_amount,
                                                resources.capacity
                                            ))
                                            .text_color(Color::srgb(0.8, 0.8, 0.5));
                                        ui.ch()
                                            .label(format!(
                                                " (yield: {}/tick)",
                                                resources.yield_rate
                                            ))
                                            .text_color(Color::srgb(0.5, 0.7, 0.5));
                                    });
                            }

                            // Characters at this location
                            let chars_here: Vec<_> = character_query
                                .iter()
                                .filter(|(_, _, _, loc)| loc.location_id == info.id)
                                .map(|(_, ci, _, _)| ci.name.clone())
                                .collect();

                            if !chars_here.is_empty() {
                                ui.ch()
                                    .label(format!(
                                        "Characters: {}",
                                        chars_here.join(", ")
                                    ))
                                    .text_color(Color::srgb(0.5, 0.8, 0.5));
                            }
                        });
                }
            });
    }
}
