use crate::game::action::{Action, ActionState, make_gather_job};
use crate::game::character::{CharacterInfo, Skills};
use crate::game::location::{LocationInfo, LocationResources};
use crate::game::resources::{EventLog, NotificationLevel, SquadState, SquadStatus};
use crate::game::simulation::SimulationState;
use crate::game::ui::inspector::{InspectorState, InspectorTab, JobPickerMode};
use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::{Imm, attach::ImmediateAttach, ui::CapsUi};

#[derive(Component)]
pub struct SquadsView;

impl ImmediateAttach<CapsUi> for SquadsView {
    type Params = (
        Res<'static, SquadState>,
        Res<'static, InspectorState>,
        Res<'static, EventLog>,
        Res<'static, SimulationState>,
        Query<
            'static,
            'static,
            (
                &'static CharacterInfo,
                &'static Skills,
                &'static ActionState,
            ),
        >,
        Query<'static, 'static, (&'static LocationInfo, Option<&'static LocationResources>)>,
    );

    fn construct(
        ui: &mut Imm<CapsUi>,
        params: &mut (
            Res<SquadState>,
            Res<InspectorState>,
            Res<EventLog>,
            Res<SimulationState>,
            Query<(&CharacterInfo, &Skills, &ActionState)>,
            Query<(&LocationInfo, Option<&LocationResources>)>,
        ),
    ) {
        let (squad_state, inspector_state, event_log, sim_state, char_query, location_query) = (
            &params.0, &params.1, &params.2, &params.3, &params.4, &params.5,
        );

        // Top-level horizontal layout: main content + event log sidebar
        ui.ch().flex_row().w_full().h_full().add(|ui| {
            // === MAIN CONTENT (scrollable) ===
            ui.ch()
                .flex_col()
                .flex_1()
                .h_full()
                .overflow_clip()
                .add(|ui| {
                    ui.ch().flex_col().scrollarea(
                        |n| {
                            n.flex_direction = FlexDirection::Column;
                            n.row_gap = Val::Px(SPACE_5);
                            n.padding = UiRect::all(Val::Px(SPACE_2_5));
                        },
                        |ui| {
                            // Render squad cards
                            for &squad_id in &squad_state.squad_order {
                                let Some(squad) = squad_state.squads.get(&squad_id) else {
                                    continue;
                                };

                                render_squad_card(
                                    ui,
                                    squad_state,
                                    inspector_state,
                                    char_query,
                                    location_query,
                                    squad,
                                );
                            }
                        },
                    );
                });

            // === EVENT LOG SIDEBAR ===
            render_event_log_sidebar(ui, event_log, sim_state);
        });
    }
}

// ── Squad Card ────────────────────────────────────────────────────────────────

/// Render a single squad card with header, progress, agent grid, and job queue.
fn render_squad_card(
    ui: &mut Imm<CapsUi>,
    squad_state: &SquadState,
    inspector_state: &InspectorState,
    char_query: &Query<(&CharacterInfo, &Skills, &ActionState)>,
    location_query: &Query<(&LocationInfo, Option<&LocationResources>)>,
    squad: &crate::game::resources::Squad,
) {
    ui.ch()
        .flex_col()
        .w_full()
        .bg(SURFACE_RAISED)
        .border(BORDER_WIDTH_DEFAULT)
        .border_color(BORDER_DEFAULT)
        .rounded(RADIUS_XL)
        .p(Val::Px(SPACE_5))
        .add(|ui| {
            // --- Header Row ---
            ui.ch()
                .flex_row()
                .w_full()
                .justify_between()
                .items_start()
                .mb(Val::Px(SPACE_5))
                .add(|ui| {
                    // Left: name + status badge + operation
                    ui.ch().flex_col().add(|ui| {
                        // Name + badge row
                        ui.ch()
                            .flex_row()
                            .items_center()
                            .column_gap(SPACE_2)
                            .mb(Val::Px(SPACE_1))
                            .add(|ui| {
                                ui.ch()
                                    .label(&squad.name)
                                    .text_size(TEXT_XL)
                                    .font_bold()
                                    .text_color(TEXT_PRIMARY);

                                // Status badge pill
                                let (badge_text, badge_color) = match squad.status {
                                    SquadStatus::Active => ("ACTIVE", PRIMARY_500),
                                    SquadStatus::Traveling => ("TRAVELING", WARNING_500),
                                    SquadStatus::Idle => ("IDLE", GRAY_500),
                                };
                                ui.ch()
                                    .px(Val::Px(SPACE_2))
                                    .py(Val::Px(SPACE_0_5))
                                    .bg(badge_color.with_alpha(0.15))
                                    .border(BORDER_WIDTH_DEFAULT)
                                    .border_color(badge_color.with_alpha(0.3))
                                    .rounded(RADIUS_DEFAULT)
                                    .add(|ui| {
                                        ui.ch()
                                            .label(badge_text)
                                            .text_size(TEXT_XS)
                                            .font_bold()
                                            .text_color(badge_color);
                                    });
                            });

                        // Operation text
                        let operation = derive_squad_operation(squad, squad_state, char_query);
                        ui.ch()
                            .label(format!("Operation: {}", operation))
                            .text_size(TEXT_SM)
                            .text_color(TEXT_MUTED);
                    });

                    // Right: squad status summary
                    ui.ch().flex_col().items_end().add(|ui| {
                        ui.ch()
                            .label("Squad Status")
                            .text_size(TEXT_XS)
                            .text_color(TEXT_MUTED)
                            .mb(Val::Px(SPACE_0_5));

                        let status_text = derive_squad_status_text(squad, squad_state, char_query);
                        let status_color = match squad.status {
                            SquadStatus::Active | SquadStatus::Traveling => SUCCESS_400,
                            SquadStatus::Idle => TEXT_MUTED,
                        };
                        ui.ch()
                            .label(&status_text)
                            .text_size(TEXT_SM)
                            .text_color(status_color);
                    });
                });

            // --- Selected Character Progress ---
            render_selected_progress(ui, squad, squad_state, inspector_state, char_query);

            // --- Agent Selection Grid ---
            ui.ch()
                .label("Select Agent to Configure")
                .text_size(TEXT_XS)
                .font_bold()
                .text_color(TEXT_MUTED)
                .mb(Val::Px(SPACE_3));

            ui.ch()
                .flex_row()
                .w_full()
                .flex_wrap()
                .gap(SPACE_4)
                .add(|ui| {
                    for member_id in &squad.members {
                        let Some(&entity) = squad_state.characters.get(member_id) else {
                            continue;
                        };
                        let Ok((info, skills, action_state)) = char_query.get(entity) else {
                            continue;
                        };

                        let is_selected =
                            inspector_state.selected_character_id.as_deref() == Some(&info.id);
                        render_agent_card(ui, info, skills, action_state, is_selected);
                    }
                });

            // --- Job Queue for Selected Character ---
            if let Some(selected_id) = &inspector_state.selected_character_id {
                if squad.members.contains(selected_id) {
                    if let Some(&entity) = squad_state.characters.get(selected_id) {
                        if let Ok((info, _skills, action_state)) = char_query.get(entity) {
                            let gather_locations: Vec<(String, String, String)> = location_query
                                .iter()
                                .filter(|(loc_info, res)| loc_info.discovered && res.is_some())
                                .filter_map(|(loc_info, res)| {
                                    let res = res?;
                                    if res.resource_type.is_empty() || res.current_amount == 0 {
                                        return None;
                                    }
                                    Some((
                                        loc_info.id.clone(),
                                        loc_info.name.clone(),
                                        res.resource_type.clone(),
                                    ))
                                })
                                .collect();

                            render_job_queue_panel(
                                ui,
                                entity,
                                info,
                                action_state,
                                &gather_locations,
                                inspector_state.job_picker_mode,
                            );
                        }
                    }
                }
            }
        });
}

/// Derive a summary operation name from the squad's member actions.
fn derive_squad_operation(
    squad: &crate::game::resources::Squad,
    squad_state: &SquadState,
    char_query: &Query<(&CharacterInfo, &Skills, &ActionState)>,
) -> String {
    for member_id in &squad.members {
        if let Some(&entity) = squad_state.characters.get(member_id) {
            if let Ok((_, _, action_state)) = char_query.get(entity) {
                match &action_state.current_action {
                    Some(Action::Gather { resource, .. }) => {
                        return format!("Resource Extraction ({})", resource);
                    }
                    Some(Action::Research { tech_id }) => {
                        return format!("Research ({})", tech_id);
                    }
                    Some(Action::Build { building }) => {
                        return format!("Construction ({})", building);
                    }
                    Some(Action::Craft { recipe_id, .. }) => {
                        return format!("Crafting ({})", recipe_id);
                    }
                    Some(Action::Explore) => {
                        return "Exploration".to_string();
                    }
                    Some(Action::Travel { destination }) => {
                        return format!("Traveling to {}", destination);
                    }
                    _ => continue,
                }
            }
        }
    }
    "Standby".to_string()
}

/// Derive status text from squad member actions.
fn derive_squad_status_text(
    squad: &crate::game::resources::Squad,
    squad_state: &SquadState,
    char_query: &Query<(&CharacterInfo, &Skills, &ActionState)>,
) -> String {
    let mut active = 0;
    let mut idle = 0;
    let total = squad.members.len();

    for member_id in &squad.members {
        if let Some(&entity) = squad_state.characters.get(member_id) {
            if let Ok((_, _, action_state)) = char_query.get(entity) {
                match &action_state.current_action {
                    None | Some(Action::Idle) => idle += 1,
                    _ => active += 1,
                }
            }
        }
    }

    if total == 0 {
        "No members".to_string()
    } else if idle == total {
        "All members idle".to_string()
    } else {
        format!("{}/{} members active", active, total)
    }
}

// ── Progress Bar ──────────────────────────────────────────────────────────────

/// Render the progress bar for the currently selected character (if in this squad).
fn render_selected_progress(
    ui: &mut Imm<CapsUi>,
    squad: &crate::game::resources::Squad,
    squad_state: &SquadState,
    inspector_state: &InspectorState,
    char_query: &Query<(&CharacterInfo, &Skills, &ActionState)>,
) {
    let Some(selected_id) = &inspector_state.selected_character_id else {
        return;
    };
    // Only show if the selected character is in this squad
    if !squad.members.contains(selected_id) {
        return;
    }
    let Some(&entity) = squad_state.characters.get(selected_id) else {
        return;
    };
    let Ok((_info, _skills, action_state)) = char_query.get(entity) else {
        return;
    };

    // Only show if there's a non-idle action with progress
    if action_state.current_action.is_none()
        || matches!(action_state.current_action, Some(Action::Idle))
        || action_state.progress.required == 0
    {
        return;
    }

    let fraction = action_state.progress.fraction();
    let pct = (fraction * 100.0) as u32;
    let action_text = format_action_short(&action_state.current_action);

    ui.ch().flex_col().w_full().mb(Val::Px(SPACE_6)).add(|ui| {
        // Header row
        ui.ch()
            .flex_row()
            .w_full()
            .justify_between()
            .items_end()
            .mb(Val::Px(SPACE_2))
            .add(|ui| {
                ui.ch()
                    .label("MISSION PROGRESS")
                    .text_size(TEXT_XS)
                    .font_bold()
                    .text_color(TEXT_MUTED);
                ui.ch()
                    .label(format!("{}%", pct))
                    .text_size(TEXT_SM)
                    .text_color(SUCCESS_400);
            });

        // Progress bar track
        ui.ch()
            .w_full()
            .style(|n: &mut Node| {
                n.height = Val::Px(10.0);
            })
            .bg(GRAY_800)
            .rounded(RADIUS_FULL)
            .overflow_clip()
            .mb(Val::Px(SPACE_1))
            .add(move |ui| {
                // Progress bar fill
                ui.ch()
                    .style(move |n: &mut Node| {
                        n.width = Val::Percent(fraction * 100.0);
                        n.height = Val::Percent(100.0);
                    })
                    .bg(SUCCESS_500)
                    .rounded(RADIUS_FULL);
            });

        // Action text below
        ui.ch()
            .label(&action_text)
            .text_size(TEXT_XS)
            .text_color(TEXT_MUTED);
    });
}

// ── Agent Cards ───────────────────────────────────────────────────────────────

/// Render an agent selection card.
fn render_agent_card(
    ui: &mut Imm<CapsUi>,
    info: &CharacterInfo,
    skills: &Skills,
    action_state: &ActionState,
    is_selected: bool,
) {
    let char_id = info.id.clone();
    let level = compute_average_level(skills);

    let (bg_color, border_color) = if is_selected {
        (GRAY_900, PRIMARY_500)
    } else {
        (Color::srgba(0.15, 0.15, 0.15, 0.5), GRAY_600)
    };

    let border_w = if is_selected {
        BORDER_WIDTH_2
    } else {
        BORDER_WIDTH_DEFAULT
    };

    ui.ch()
        .button()
        .on_click_once(
            move |_trigger: On<Pointer<Click>>, mut inspector: ResMut<InspectorState>| {
                inspector.selected_character_id = Some(char_id.clone());
                inspector.active_tab = InspectorTab::Health;
            },
        )
        .style(|n: &mut Node| {
            n.min_width = Val::Px(180.0);
            n.flex_grow = 1.0;
            n.padding = UiRect::all(Val::Px(SPACE_3));
        })
        .bg(bg_color)
        .border(border_w)
        .border_color(border_color)
        .rounded(RADIUS_LG)
        .add(|ui| {
            ui.ch()
                .flex_row()
                .items_center()
                .column_gap(SPACE_3)
                .w_full()
                .add(|ui| {
                    // Avatar placeholder (colored circle with first letter)
                    let first_char = info
                        .name
                        .chars()
                        .next()
                        .unwrap_or('?')
                        .to_uppercase()
                        .to_string();
                    ui.ch()
                        .style(|n: &mut Node| {
                            n.width = Val::Px(48.0);
                            n.height = Val::Px(48.0);
                            n.justify_content = JustifyContent::Center;
                            n.align_items = AlignItems::Center;
                        })
                        .bg(if is_selected { PRIMARY_700 } else { GRAY_700 })
                        .border(BORDER_WIDTH_2)
                        .border_color(if is_selected { PRIMARY_500 } else { GRAY_600 })
                        .rounded(RADIUS_FULL)
                        .add(|ui| {
                            ui.ch()
                                .label(&first_char)
                                .text_size(TEXT_LG)
                                .font_bold()
                                .text_color(TEXT_PRIMARY);
                        });

                    // Name + level + status
                    ui.ch().flex_col().flex_1().add(|ui| {
                        // Name + level row
                        ui.ch()
                            .flex_row()
                            .items_center()
                            .column_gap(SPACE_2)
                            .add(|ui| {
                                ui.ch()
                                    .label(&info.name)
                                    .font_bold()
                                    .text_color(if is_selected { TEXT_PRIMARY } else { GRAY_300 });

                                // Level badge
                                let badge_bg = if is_selected {
                                    PRIMARY_500.with_alpha(0.2)
                                } else {
                                    GRAY_700
                                };
                                ui.ch()
                                    .px(Val::Px(SPACE_1_5))
                                    .py(Val::Px(SPACE_0_5))
                                    .bg(badge_bg)
                                    .rounded(RADIUS_DEFAULT)
                                    .add(|ui| {
                                        ui.ch()
                                            .label(format!("Lvl {}", level))
                                            .text_size(10.0)
                                            .text_color(if is_selected {
                                                PRIMARY_400
                                            } else {
                                                GRAY_400
                                            });
                                    });
                            });

                        // Status text
                        let (status_text, status_color) = action_status_display(action_state);
                        ui.ch()
                            .label(&status_text)
                            .text_size(TEXT_XS)
                            .text_color(status_color);
                    });
                });
        });
}

/// Compute average character level from all skills.
fn compute_average_level(skills: &Skills) -> u32 {
    let total: u32 = skills.iter().map(|(_, xp)| xp_to_level(xp)).sum();
    let count = skills.iter().count() as u32;
    if count == 0 {
        1
    } else {
        (total / count).max(1)
    }
}

/// Convert XP to level (same formula as inspector).
fn xp_to_level(xp: u32) -> u32 {
    let xp = xp as f64;
    ((xp * 4.0 / 5.0).cbrt().floor() as u32 + 1).min(100)
}

/// Get display text and color for a character's current action.
fn action_status_display(action_state: &ActionState) -> (String, Color) {
    match &action_state.current_action {
        None | Some(Action::Idle) => ("Idle".to_string(), TEXT_MUTED),
        Some(Action::Travel { destination }) => {
            (format!("Traveling to {}", destination), SUCCESS_400)
        }
        Some(Action::Gather { resource, .. }) => (format!("Gathering {}", resource), SUCCESS_400),
        Some(Action::Research { tech_id }) => (format!("Researching {}", tech_id), INFO_400),
        Some(Action::Build { building }) => (format!("Building {}", building), WARNING_400),
        Some(Action::Craft { recipe_id, .. }) => (format!("Crafting {}", recipe_id), INFO_400),
        Some(Action::Explore) => ("Exploring".to_string(), SUCCESS_400),
        Some(Action::Collect { item, .. }) => (format!("Collecting {}", item), SUCCESS_400),
        Some(Action::Deposit { item }) => (format!("Depositing {}", item), SUCCESS_400),
    }
}

/// Short format for action display in progress bar area.
fn format_action_short(action: &Option<Action>) -> String {
    match action {
        None | Some(Action::Idle) => "Idle".to_string(),
        Some(Action::Travel { destination }) => format!("Traveling to {}", destination),
        Some(Action::Gather { resource, location }) => {
            format!("Gathering {} at {}", resource, location)
        }
        Some(Action::Research { tech_id }) => format!("Researching {}", tech_id),
        Some(Action::Build { building }) => format!("Building {}", building),
        Some(Action::Craft { recipe_id, .. }) => format!("Crafting {}", recipe_id),
        Some(Action::Explore) => "Exploring".to_string(),
        Some(Action::Collect { item, .. }) => format!("Collecting {}", item),
        Some(Action::Deposit { item }) => format!("Depositing {}", item),
    }
}

// ── Job Queue Panel ───────────────────────────────────────────────────────────

/// Render the job queue panel for the selected character.
fn render_job_queue_panel(
    ui: &mut Imm<CapsUi>,
    entity_id: Entity,
    info: &CharacterInfo,
    action_state: &ActionState,
    gather_locations: &[(String, String, String)],
    picker_mode: JobPickerMode,
) {
    ui.ch()
        .flex_col()
        .w_full()
        .mt(Val::Px(SPACE_5))
        .bg(SURFACE_RAISED)
        .border(BORDER_WIDTH_DEFAULT)
        .border_color(BORDER_DEFAULT)
        .rounded(RADIUS_XL)
        .add(|ui| {
            // Header
            ui.ch()
                .flex_row()
                .w_full()
                .justify_between()
                .items_center()
                .p(Val::Px(SPACE_5))
                .style(|n: &mut Node| {
                    n.border = UiRect::bottom(Val::Px(BORDER_WIDTH_DEFAULT));
                })
                .border_color(BORDER_DEFAULT)
                .add(|ui| {
                    ui.ch().flex_col().add(|ui| {
                        ui.ch()
                            .label("Job Queue")
                            .text_size(TEXT_LG)
                            .font_bold()
                            .text_color(TEXT_PRIMARY);
                        ui.ch()
                            .flex_row()
                            .items_center()
                            .column_gap(SPACE_2)
                            .mt(Val::Px(SPACE_1))
                            .add(|ui| {
                                ui.ch()
                                    .label("Configuration for Agent:")
                                    .text_size(TEXT_SM)
                                    .text_color(TEXT_MUTED);
                                // Agent name badge
                                ui.ch()
                                    .px(Val::Px(SPACE_2))
                                    .py(Val::Px(SPACE_0_5))
                                    .bg(PRIMARY_500.with_alpha(0.1))
                                    .border(BORDER_WIDTH_DEFAULT)
                                    .border_color(PRIMARY_500.with_alpha(0.2))
                                    .rounded(RADIUS_DEFAULT)
                                    .add(|ui| {
                                        ui.ch()
                                            .label(&info.name)
                                            .text_size(TEXT_XS)
                                            .font_bold()
                                            .text_color(PRIMARY_400);
                                    });
                            });
                    });

                    // Action buttons
                    ui.ch().flex_row().column_gap(SPACE_2).add(|ui| {
                        // Clear Jobs button
                        let entity = entity_id;
                        ui.ch()
                            .button()
                            .style(|n: &mut Node| {
                                n.padding =
                                    UiRect::axes(Val::Px(SPACE_3), Val::Px(SPACE_1_5));
                            })
                            .bg(GRAY_800)
                            .border(BORDER_WIDTH_DEFAULT)
                            .border_color(GRAY_700)
                            .rounded(RADIUS_DEFAULT)
                            .on_click_once(
                                move |_: On<Pointer<Click>>,
                                      mut action_query: Query<&mut ActionState>| {
                                    if let Ok(mut state) = action_query.get_mut(entity) {
                                        state.clear_jobs();
                                    }
                                },
                            )
                            .add(|ui| {
                                ui.ch()
                                    .label("Clear Jobs")
                                    .text_size(TEXT_XS)
                                    .text_color(GRAY_300);
                            });

                        // Clear Actions button
                        let entity = entity_id;
                        ui.ch()
                            .button()
                            .style(|n: &mut Node| {
                                n.padding =
                                    UiRect::axes(Val::Px(SPACE_3), Val::Px(SPACE_1_5));
                            })
                            .bg(GRAY_800)
                            .border(BORDER_WIDTH_DEFAULT)
                            .border_color(GRAY_700)
                            .rounded(RADIUS_DEFAULT)
                            .on_click_once(
                                move |_: On<Pointer<Click>>,
                                      mut action_query: Query<&mut ActionState>| {
                                    if let Ok(mut state) = action_query.get_mut(entity) {
                                        state.clear();
                                    }
                                },
                            )
                            .add(|ui| {
                                ui.ch()
                                    .label("Clear Actions")
                                    .text_size(TEXT_XS)
                                    .text_color(GRAY_300);
                            });
                    });
                });

            // Table header
            ui.ch()
                .flex_row()
                .w_full()
                .px(Val::Px(SPACE_5))
                .py(Val::Px(SPACE_3))
                .bg(Color::srgba(0.1, 0.1, 0.1, 0.5))
                .style(|n: &mut Node| {
                    n.border = UiRect::bottom(Val::Px(BORDER_WIDTH_DEFAULT));
                })
                .border_color(BORDER_DEFAULT)
                .add(|ui| {
                    ui.ch()
                        .style(|n: &mut Node| {
                            n.width = Val::Px(40.0);
                        })
                        .add(|ui| {
                            ui.ch()
                                .label("#")
                                .text_size(TEXT_XS)
                                .font_bold()
                                .text_color(TEXT_MUTED);
                        });
                    ui.ch().flex_1().add(|ui| {
                        ui.ch()
                            .label("JOB NAME")
                            .text_size(TEXT_XS)
                            .font_bold()
                            .text_color(TEXT_MUTED);
                    });
                    ui.ch()
                        .style(|n: &mut Node| {
                            n.width = Val::Px(80.0);
                        })
                        .add(|ui| {
                            ui.ch()
                                .label("STEPS")
                                .text_size(TEXT_XS)
                                .font_bold()
                                .text_color(TEXT_MUTED);
                        });
                    ui.ch()
                        .style(|n: &mut Node| {
                            n.width = Val::Px(80.0);
                        })
                        .add(|ui| {
                            ui.ch()
                                .label("STATUS")
                                .text_size(TEXT_XS)
                                .font_bold()
                                .text_color(TEXT_MUTED);
                        });
                });

            // Job rows
            if action_state.job_queue.is_empty() {
                ui.ch().p(Val::Px(SPACE_5)).add(|ui| {
                    ui.ch()
                        .label("No jobs assigned")
                        .text_size(TEXT_SM)
                        .text_color(TEXT_MUTED);
                });
            } else {
                for (i, job) in action_state.job_queue.iter().enumerate() {
                    let is_current = action_state.current_job_index > 0
                        && (action_state.current_job_index - 1) % action_state.job_queue.len() == i;

                    let row_bg = if is_current {
                        Color::srgba(0.15, 0.15, 0.15, 0.5)
                    } else {
                        Color::NONE
                    };

                    ui.ch()
                        .flex_row()
                        .w_full()
                        .items_center()
                        .px(Val::Px(SPACE_5))
                        .py(Val::Px(SPACE_3))
                        .bg(row_bg)
                        .style(|n: &mut Node| {
                            n.border = UiRect::bottom(Val::Px(BORDER_WIDTH_DEFAULT));
                        })
                        .border_color(BORDER_MUTED)
                        .add(|ui| {
                            // Row number
                            ui.ch()
                                .style(|n: &mut Node| {
                                    n.width = Val::Px(40.0);
                                })
                                .add(|ui| {
                                    ui.ch()
                                        .label(format!("{}", i + 1))
                                        .text_size(TEXT_SM)
                                        .text_color(GRAY_400);
                                });

                            // Job name with colored badge
                            ui.ch()
                                .flex_1()
                                .flex_row()
                                .items_center()
                                .column_gap(SPACE_3)
                                .add(|ui| {
                                    let badge_color = job_type_color(&job.name);
                                    ui.ch()
                                        .px(Val::Px(SPACE_2))
                                        .py(Val::Px(SPACE_1))
                                        .bg(badge_color.with_alpha(0.1))
                                        .border(BORDER_WIDTH_DEFAULT)
                                        .border_color(badge_color.with_alpha(0.2))
                                        .rounded(RADIUS_DEFAULT)
                                        .add(|ui| {
                                            ui.ch()
                                                .label(&job.name)
                                                .text_size(TEXT_XS)
                                                .font_bold()
                                                .text_color(badge_color);
                                        });
                                });

                            // Steps count
                            ui.ch()
                                .style(|n: &mut Node| {
                                    n.width = Val::Px(80.0);
                                })
                                .add(|ui| {
                                    ui.ch()
                                        .label(format!("{}", job.actions.len()))
                                        .text_size(TEXT_SM)
                                        .text_color(GRAY_400);
                                });

                            // Active indicator
                            ui.ch()
                                .style(|n: &mut Node| {
                                    n.width = Val::Px(80.0);
                                })
                                .add(|ui| {
                                    if is_current {
                                        ui.ch()
                                            .label("Active")
                                            .text_size(TEXT_XS)
                                            .font_bold()
                                            .text_color(SUCCESS_400);
                                    } else {
                                        ui.ch()
                                            .label("Queued")
                                            .text_size(TEXT_XS)
                                            .text_color(GRAY_500);
                                    }
                                });
                        });
                }
            }

            // Add job buttons at bottom
            ui.ch()
                .flex_row()
                .w_full()
                .p(Val::Px(SPACE_5))
                .column_gap(SPACE_2)
                .add(|ui| {
                    match picker_mode {
                        JobPickerMode::None => {
                            if !gather_locations.is_empty() {
                                ui.ch()
                                    .button()
                                    .style(|n: &mut Node| {
                                        n.padding = UiRect::axes(
                                            Val::Px(SPACE_3),
                                            Val::Px(SPACE_1_5),
                                        );
                                    })
                                    .bg(PRIMARY_500.with_alpha(0.2))
                                    .border(BORDER_WIDTH_DEFAULT)
                                    .border_color(PRIMARY_500.with_alpha(0.4))
                                    .rounded(RADIUS_DEFAULT)
                                    .on_click_once(
                                        move |_: On<Pointer<Click>>,
                                              mut inspector: ResMut<InspectorState>| {
                                            inspector.job_picker_mode =
                                                JobPickerMode::GatherPicker;
                                        },
                                    )
                                    .add(|ui| {
                                        ui.ch()
                                            .label("+ Add Gather Job")
                                            .text_size(TEXT_XS)
                                            .text_color(PRIMARY_400);
                                    });
                            } else {
                                ui.ch()
                                    .label("No resource locations available")
                                    .text_size(TEXT_XS)
                                    .text_color(TEXT_MUTED);
                            }
                        }
                        JobPickerMode::GatherPicker => {
                            ui.ch().flex_col().w_full().row_gap(SPACE_1).add(|ui| {
                                // Back button
                                ui.ch()
                                    .button()
                                    .style(|n: &mut Node| {
                                        n.padding = UiRect::axes(
                                            Val::Px(SPACE_3),
                                            Val::Px(SPACE_1_5),
                                        );
                                    })
                                    .bg(GRAY_800)
                                    .border(BORDER_WIDTH_DEFAULT)
                                    .border_color(GRAY_700)
                                    .rounded(RADIUS_DEFAULT)
                                    .on_click_once(
                                        move |_: On<Pointer<Click>>,
                                              mut inspector: ResMut<InspectorState>| {
                                            inspector.job_picker_mode = JobPickerMode::None;
                                        },
                                    )
                                    .add(|ui| {
                                        ui.ch()
                                            .label("<- Back")
                                            .text_size(TEXT_XS)
                                            .text_color(GRAY_300);
                                    });

                                // Location buttons
                                for (loc_id, loc_name, resource) in gather_locations {
                                    let entity = entity_id;
                                    let location = loc_id.clone();
                                    let res = resource.clone();
                                    let label = format!("{} ({})", loc_name, resource);

                                    ui.ch()
                                        .button()
                                        .w_full()
                                        .style(|n: &mut Node| {
                                            n.padding = UiRect::axes(
                                                Val::Px(SPACE_3),
                                                Val::Px(SPACE_1_5),
                                            );
                                        })
                                        .bg(GRAY_800)
                                        .border(BORDER_WIDTH_DEFAULT)
                                        .border_color(GRAY_700)
                                        .rounded(RADIUS_DEFAULT)
                                        .on_click_once(
                                            move |_: On<Pointer<Click>>,
                                                  mut inspector: ResMut<InspectorState>,
                                                  mut action_query: Query<
                                                &mut ActionState,
                                            >| {
                                                if let Ok(mut action_state) =
                                                    action_query.get_mut(entity)
                                                {
                                                    let job = make_gather_job(&location, &res);
                                                    action_state.job_queue.push(job);
                                                }
                                                inspector.job_picker_mode = JobPickerMode::None;
                                            },
                                        )
                                        .add(|ui| {
                                            ui.ch()
                                                .label(&label)
                                                .text_size(TEXT_XS)
                                                .text_color(GRAY_100);
                                        });
                                }
                            });
                        }
                    }
                });
        });
}

/// Get a color for a job type based on its name.
fn job_type_color(job_name: &str) -> Color {
    let name_lower = job_name.to_lowercase();
    if name_lower.contains("gather") || name_lower.contains("mine") {
        WARNING_400
    } else if name_lower.contains("craft") {
        INFO_400
    } else if name_lower.contains("build") {
        SECONDARY_400
    } else if name_lower.contains("research") {
        ACCENT_400
    } else if name_lower.contains("explore") {
        SUCCESS_400
    } else {
        GRAY_400
    }
}

// ── Event Log Sidebar ─────────────────────────────────────────────────────────

/// Render the event log sidebar.
fn render_event_log_sidebar(
    ui: &mut Imm<CapsUi>,
    event_log: &EventLog,
    sim_state: &SimulationState,
) {
    ui.ch()
        .flex_col()
        .style(|n: &mut Node| {
            n.width = Val::Px(280.0);
            n.border = UiRect::left(Val::Px(BORDER_WIDTH_DEFAULT));
        })
        .h_full()
        .border_color(BORDER_DEFAULT)
        .bg(SURFACE_BASE)
        .add(|ui| {
            // Header
            ui.ch()
                .flex_row()
                .w_full()
                .justify_between()
                .items_center()
                .p(Val::Px(SPACE_4))
                .style(|n: &mut Node| {
                    n.border = UiRect::bottom(Val::Px(BORDER_WIDTH_DEFAULT));
                })
                .border_color(BORDER_DEFAULT)
                .bg(SURFACE_RAISED)
                .add(|ui| {
                    ui.ch()
                        .label("EVENT LOG")
                        .text_size(TEXT_SM)
                        .font_bold()
                        .text_color(TEXT_PRIMARY);
                });

            // Scrollable timeline
            ui.ch().flex_1().scrollarea(
                |n| {
                    n.flex_direction = FlexDirection::Column;
                    n.padding = UiRect::all(Val::Px(SPACE_4));
                    n.row_gap = Val::Px(SPACE_4);
                },
                |ui| {
                    if event_log.entries.is_empty() {
                        ui.ch()
                            .label("No events yet")
                            .text_size(TEXT_XS)
                            .text_color(TEXT_MUTED);
                        return;
                    }

                    for entry in &event_log.entries {
                        render_event_log_entry(ui, entry, sim_state);
                    }
                },
            );
        });
}

/// Render a single event log entry in the timeline.
fn render_event_log_entry(
    ui: &mut Imm<CapsUi>,
    entry: &crate::game::resources::EventLogEntry,
    sim_state: &SimulationState,
) {
    let dot_color = match entry.level {
        NotificationLevel::Success => SUCCESS_500,
        NotificationLevel::Error => ERROR_500,
        NotificationLevel::Info => INFO_500,
    };

    let text_color = match entry.level {
        NotificationLevel::Success => SUCCESS_400,
        NotificationLevel::Error => ERROR_400,
        NotificationLevel::Info => GRAY_300,
    };

    // Timeline entry with left border line
    ui.ch()
        .flex_row()
        .w_full()
        .relative()
        .style(|n: &mut Node| {
            n.border = UiRect::left(Val::Px(BORDER_WIDTH_DEFAULT));
            n.padding = UiRect::left(Val::Px(SPACE_4));
        })
        .border_color(GRAY_800)
        .add(|ui| {
            // Timeline dot (positioned to overlap the left border)
            ui.ch()
                .absolute()
                .style(|n: &mut Node| {
                    n.left = Val::Px(-5.0);
                    n.top = Val::Px(2.0);
                    n.width = Val::Px(10.0);
                    n.height = Val::Px(10.0);
                })
                .bg(dot_color)
                .rounded(RADIUS_FULL);

            // Content
            ui.ch().flex_col().row_gap(SPACE_0_5).add(|ui| {
                // Timestamp
                let ticks_ago = sim_state.game_time.saturating_sub(entry.game_tick);
                let time_text = if ticks_ago < 60 {
                    format!("{}t ago", ticks_ago)
                } else if ticks_ago < 3600 {
                    format!("{}m ago", ticks_ago / 60)
                } else {
                    format!("{}h ago", ticks_ago / 3600)
                };
                ui.ch()
                    .label(&time_text)
                    .text_size(10.0)
                    .text_color(TEXT_MUTED);

                // Message
                ui.ch()
                    .label(&entry.message)
                    .text_size(TEXT_XS)
                    .text_color(text_color);
            });
        });
}
