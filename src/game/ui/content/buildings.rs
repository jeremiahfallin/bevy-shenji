use crate::game::action::{Action, ActionState};
use crate::game::building::Building;
use crate::game::character::{CharacterInfo, CharacterLocation};
use crate::game::data::GameData;
use crate::game::research::ResearchState;
use crate::game::resources::{BaseInventory, BaseState, NotificationLevel, NotificationState};
use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::{Imm, attach::ImmediateAttach, ui::CapsUi};

#[derive(Component)]
pub struct BuildingsView;

impl ImmediateAttach<CapsUi> for BuildingsView {
    type Params = (
        Res<'static, GameData>,
        Res<'static, ResearchState>,
        Res<'static, BaseInventory>,
        Res<'static, BaseState>,
        Query<'static, 'static, &'static Building>,
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
    );

    fn construct(
        ui: &mut Imm<CapsUi>,
        (game_data, research_state, base_inventory, base_state, buildings_query, character_query): &mut (
            Res<GameData>,
            Res<ResearchState>,
            Res<BaseInventory>,
            Res<BaseState>,
            Query<&Building>,
            Query<(Entity, &CharacterInfo, &ActionState, &CharacterLocation)>,
        ),
    ) {
        ui.ch().header("Buildings");

        ui.ch()
            .flex_col()
            .w_full()
            .p(Val::Px(SPACE_2_5))
            .row_gap(SPACE_2_5)
            .scroll_y()
            .add(|ui| {
                // --- Under Construction ---
                let in_progress: Vec<_> = buildings_query.iter().filter(|b| !b.complete).collect();

                if !in_progress.is_empty() {
                    ui.ch()
                        .sub_header("Under Construction")
                        .mb(Val::Px(SPACE_1));

                    for building in &in_progress {
                        let frac = if building.required > 0 {
                            building.progress as f32 / building.required as f32
                        } else {
                            1.0
                        };

                        ui.ch()
                            .flex_col()
                            .w_full()
                            .p(Val::Px(SPACE_2))
                            .bg(Color::srgb(0.15, 0.15, 0.3))
                            .mb(Val::Px(SPACE_1))
                            .rounded(4.0)
                            .add(|ui| {
                                ui.ch()
                                    .label(&building.name)
                                    .font_bold()
                                    .text_color(Color::WHITE);

                                ui.ch()
                                    .label(format!(
                                        "{}/{} ticks ({:.0}%)",
                                        building.progress,
                                        building.required,
                                        frac * 100.0
                                    ))
                                    .text_size(11.0)
                                    .text_color(GRAY_400);

                                ui.ch().progress_bar(frac);
                            });
                    }
                }

                // --- Completed Buildings ---
                let completed: Vec<_> = buildings_query.iter().filter(|b| b.complete).collect();

                if !completed.is_empty() {
                    ui.ch().sub_header("Completed").mb(Val::Px(SPACE_1));

                    for building in &completed {
                        let description = game_data
                            .get_building(&building.def_id)
                            .map(|d| d.description.as_str())
                            .unwrap_or("");

                        ui.ch()
                            .flex_col()
                            .w_full()
                            .p(Val::Px(SPACE_2))
                            .bg(Color::srgb(0.15, 0.3, 0.15))
                            .mb(Val::Px(SPACE_1))
                            .rounded(4.0)
                            .add(|ui| {
                                ui.ch()
                                    .label(&building.name)
                                    .font_bold()
                                    .text_color(Color::WHITE);
                                if !description.is_empty() {
                                    ui.ch()
                                        .label(description)
                                        .text_size(11.0)
                                        .text_color(GRAY_400);
                                }
                            });
                    }
                }

                // --- Available Blueprints ---
                ui.ch()
                    .sub_header("Available Blueprints")
                    .mb(Val::Px(SPACE_1));

                let categories = [
                    ("tech", "Technology"),
                    ("food", "Food"),
                    ("storage", "Storage"),
                    ("power", "Power"),
                    ("training", "Training"),
                    ("defense", "Defense"),
                ];

                let current_tech = base_state.value.tech_level.max(1);

                for (category_id, category_name) in &categories {
                    let mut buildings_in_cat: Vec<_> = game_data
                        .buildings
                        .values()
                        .filter(|def| {
                            def.category == *category_id && def.tech_level <= current_tech
                        })
                        .collect();

                    if buildings_in_cat.is_empty() {
                        continue;
                    }

                    buildings_in_cat.sort_by_key(|def| &def.name);

                    ui.ch()
                        .label(*category_name)
                        .font_bold()
                        .text_color(GRAY_300)
                        .mb(Val::Px(SPACE_0_5));

                    for def in &buildings_in_cat {
                        let research_met = def
                            .required_research
                            .iter()
                            .all(|r| research_state.is_unlocked(r));

                        let can_afford = def
                            .cost
                            .iter()
                            .all(|(item, &amount)| base_inventory.count(item) >= amount);

                        // Find an idle character at base
                        let idle_at_base: Option<(Entity, String)> = character_query
                            .iter()
                            .find(|(_, _, action_state, loc)| {
                                loc.location_id == "base"
                                    && matches!(
                                        &action_state.current_action,
                                        None | Some(Action::Idle)
                                    )
                            })
                            .map(|(e, info, _, _)| (e, info.name.clone()));

                        let bg_color = if !research_met { GRAY_900 } else { GRAY_800 };

                        ui.ch()
                            .flex_col()
                            .w_full()
                            .p(Val::Px(SPACE_2))
                            .bg(bg_color)
                            .mb(Val::Px(SPACE_1))
                            .rounded(4.0)
                            .add(|ui| {
                                // Name
                                ui.ch()
                                    .label(&def.name)
                                    .font_bold()
                                    .text_color(Color::WHITE);

                                // Description
                                ui.ch()
                                    .label(&def.description)
                                    .text_size(11.0)
                                    .text_color(GRAY_400);

                                // Cost display
                                let mut cost_parts: Vec<String> = def
                                    .cost
                                    .iter()
                                    .map(|(item, &amount)| {
                                        let have = base_inventory.count(item);
                                        format!("{}: {}/{}", item, have, amount)
                                    })
                                    .collect();
                                cost_parts.sort();

                                ui.ch()
                                    .label(format!("Cost: {}", cost_parts.join(", ")))
                                    .text_size(11.0)
                                    .text_color(if can_afford { Color::WHITE } else { ERROR_500 });

                                // Build time
                                ui.ch()
                                    .label(format!("Build time: {} ticks", def.build_time))
                                    .text_size(11.0)
                                    .text_color(GRAY_400);

                                // Power info
                                if def.power_generation != 0 {
                                    let power_text = if def.power_generation > 0 {
                                        format!("Generates: {} power", def.power_generation)
                                    } else {
                                        format!("Consumes: {} power", def.power_generation.abs())
                                    };
                                    ui.ch().label(power_text).text_size(11.0).text_color(
                                        if def.power_generation > 0 {
                                            SUCCESS_600
                                        } else {
                                            Color::srgb(0.8, 0.6, 0.2)
                                        },
                                    );
                                }

                                // Storage info
                                if def.provides_storage > 0 {
                                    ui.ch()
                                        .label(format!(
                                            "Storage: +{} capacity",
                                            def.provides_storage
                                        ))
                                        .text_size(11.0)
                                        .text_color(INFO_600);
                                }

                                // Missing research
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
                                    ui.ch()
                                        .label(format!("Requires: {}", missing.join(", ")))
                                        .text_size(10.0)
                                        .text_color(ERROR_500);
                                }

                                // Build button (only show if research is met)
                                if research_met {
                                    let is_buildable = can_afford && idle_at_base.is_some();

                                    let building_id = def.id.clone();
                                    let building_name = def.name.clone();
                                    let build_time = def.build_time;
                                    let cost_map = def.cost.clone();
                                    let assignee = idle_at_base.as_ref().map(|(e, _)| *e);

                                    let button_label = if !can_afford {
                                        "Need Resources"
                                    } else if idle_at_base.is_none() {
                                        "No Idle Worker"
                                    } else {
                                        "Build"
                                    };

                                    let worker_name = idle_at_base
                                        .as_ref()
                                        .map(|(_, n)| n.clone())
                                        .unwrap_or_default();

                                    ui.ch()
                                        .button()
                                        .disabled(!is_buildable)
                                        .w_full()
                                        .style(|n: &mut Node| {
                                            n.margin.top = Val::Px(SPACE_1);
                                        })
                                        .on_click_once(
                                            move |_: On<Pointer<Click>>,
                                                  mut commands: Commands,
                                                  mut base_inv: ResMut<BaseInventory>,
                                                  mut action_query: Query<&mut ActionState>,
                                                  mut notifications: ResMut<
                                                NotificationState,
                                            >| {
                                                // Deduct costs
                                                for (item, &amount) in &cost_map {
                                                    if !base_inv.remove(item, amount) {
                                                        return;
                                                    }
                                                }
                                                // Spawn building entity
                                                commands.spawn(Building {
                                                    def_id: building_id.clone(),
                                                    name: building_name.clone(),
                                                    progress: 0,
                                                    required: build_time,
                                                    complete: false,
                                                });
                                                // Assign character
                                                if let Some(entity) = assignee {
                                                    if let Ok(mut action_state) =
                                                        action_query.get_mut(entity)
                                                    {
                                                        action_state.queue_action(
                                                            Action::Build {
                                                                building: building_id.clone(),
                                                            },
                                                        );
                                                    }
                                                }
                                                notifications.push(
                                                    format!(
                                                        "{} started building {}",
                                                        worker_name, building_name
                                                    ),
                                                    NotificationLevel::Info,
                                                );
                                            },
                                        )
                                        .add(|ui| {
                                            ui.ch()
                                                .label(button_label)
                                                .text_size(12.0)
                                                .text_color(Color::WHITE);
                                        });
                                }
                            });
                    }
                }
            });
    }
}
