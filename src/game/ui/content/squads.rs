//! Squads view — column of squad cards with header / progress / agent grid /
//! job queue, plus an event log sidebar.
//!
//! Spawned once on `On<Add, SquadsView>`. A refresh system gated on any of
//! the source resources/queries having changed despawns the view's children
//! and rebuilds the tree — the master design's stated update policy. Click
//! handlers use marker components on buttons so observer systems can recover
//! their context (target entity, scenario id, etc.) at click time without
//! capturing in a closure.
//!
//! Visual note: the pre-migration code used `font_bold()` on several labels
//! (theme widget). The new UI builders don't expose a bold font handle; bold
//! styling is dropped here. Layout/colors/sizes are preserved.

use bevy::prelude::*;
use bevy_declarative::prelude::px;

use crate::game::action::{Action, ActionState, make_gather_job};
use crate::game::character::{CharacterInfo, Skills};
use crate::game::location::{LocationInfo, LocationResources};
use crate::game::resources::{
    EventLog, EventLogEntry, NotificationLevel, Squad as GameSquad, SquadState, SquadStatus,
};
use crate::game::simulation::SimulationState;
use crate::game::ui::inspector::{InspectorState, InspectorTab, JobPickerMode};
use crate::screens::Screen;
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(populate_squads_on_add);
    app.add_systems(Update, refresh_squads.run_if(in_state(Screen::Gameplay)));
}

#[derive(Component)]
pub struct SquadsView;

// --- Click marker components ------------------------------------------------

#[derive(Component)]
struct AgentCardButton(String);

#[derive(Component)]
struct ClearJobsButton(Entity);

#[derive(Component)]
struct ClearActionsButton(Entity);

#[derive(Component)]
struct AddGatherJobButton;

#[derive(Component)]
struct BackPickerButton;

#[derive(Component)]
struct LocationGatherButton {
    entity: Entity,
    location_id: String,
    resource: String,
}

// --- Click observer systems -------------------------------------------------

fn agent_card_on_click(
    click: On<Pointer<Click>>,
    q: Query<&AgentCardButton>,
    mut inspector: ResMut<InspectorState>,
) {
    if let Ok(marker) = q.get(click.entity) {
        inspector.selected_character_id = Some(marker.0.clone());
        inspector.active_tab = InspectorTab::Health;
    }
}

fn clear_jobs_on_click(
    click: On<Pointer<Click>>,
    q: Query<&ClearJobsButton>,
    mut action_query: Query<&mut ActionState>,
) {
    if let Ok(marker) = q.get(click.entity) {
        if let Ok(mut state) = action_query.get_mut(marker.0) {
            state.clear_jobs();
        }
    }
}

fn clear_actions_on_click(
    click: On<Pointer<Click>>,
    q: Query<&ClearActionsButton>,
    mut action_query: Query<&mut ActionState>,
) {
    if let Ok(marker) = q.get(click.entity) {
        if let Ok(mut state) = action_query.get_mut(marker.0) {
            state.clear();
        }
    }
}

fn add_gather_on_click(_: On<Pointer<Click>>, mut inspector: ResMut<InspectorState>) {
    inspector.job_picker_mode = JobPickerMode::GatherPicker;
}

fn back_picker_on_click(_: On<Pointer<Click>>, mut inspector: ResMut<InspectorState>) {
    inspector.job_picker_mode = JobPickerMode::None;
}

fn location_gather_on_click(
    click: On<Pointer<Click>>,
    q: Query<&LocationGatherButton>,
    mut inspector: ResMut<InspectorState>,
    mut action_query: Query<&mut ActionState>,
) {
    if let Ok(marker) = q.get(click.entity) {
        if let Ok(mut action_state) = action_query.get_mut(marker.entity) {
            let job = make_gather_job(&marker.location_id, &marker.resource);
            action_state.job_queue.push(job);
        }
        inspector.job_picker_mode = JobPickerMode::None;
    }
}

// --- Lifecycle: spawn / refresh ---------------------------------------------

#[allow(clippy::too_many_arguments)]
fn populate_squads_on_add(
    add: On<Add, SquadsView>,
    mut commands: Commands,
    squad_state: Res<SquadState>,
    inspector_state: Res<InspectorState>,
    event_log: Res<EventLog>,
    sim_state: Res<SimulationState>,
    char_query: Query<(&CharacterInfo, &Skills, &ActionState)>,
    location_query: Query<(&LocationInfo, Option<&LocationResources>)>,
) {
    spawn_squads_children(
        &mut commands,
        add.entity,
        &squad_state,
        &inspector_state,
        &event_log,
        &sim_state,
        &char_query,
        &location_query,
    );
}

#[allow(clippy::too_many_arguments)]
fn refresh_squads(
    mut commands: Commands,
    view_q: Query<Entity, With<SquadsView>>,
    squad_state: Res<SquadState>,
    inspector_state: Res<InspectorState>,
    event_log: Res<EventLog>,
    sim_state: Res<SimulationState>,
    char_query: Query<(&CharacterInfo, &Skills, &ActionState)>,
    location_query: Query<(&LocationInfo, Option<&LocationResources>)>,
    changed_actions: Query<(), Changed<ActionState>>,
    changed_chars: Query<(), Changed<CharacterInfo>>,
    changed_locs: Query<
        (),
        Or<(
            Changed<LocationInfo>,
            Changed<LocationResources>,
        )>,
    >,
) {
    let any_changed = squad_state.is_changed()
        || inspector_state.is_changed()
        || event_log.is_changed()
        || sim_state.is_changed()
        || !changed_actions.is_empty()
        || !changed_chars.is_empty()
        || !changed_locs.is_empty();
    if !any_changed {
        return;
    }
    for view in &view_q {
        commands.entity(view).despawn_related::<Children>();
        spawn_squads_children(
            &mut commands,
            view,
            &squad_state,
            &inspector_state,
            &event_log,
            &sim_state,
            &char_query,
            &location_query,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_squads_children(
    commands: &mut Commands,
    view: Entity,
    squad_state: &SquadState,
    inspector_state: &InspectorState,
    event_log: &EventLog,
    sim_state: &SimulationState,
    char_query: &Query<(&CharacterInfo, &Skills, &ActionState)>,
    location_query: &Query<(&LocationInfo, Option<&LocationResources>)>,
) {
    // Main scrollable column of squad cards.
    let mut main_scroll = scroll_view().vertical(true);
    let mut main_inner = div()
        .col()
        .gap_y(px(SPACE_5))
        .p(px(SPACE_2_5))
        .w_full();
    for &squad_id in &squad_state.squad_order {
        let Some(squad) = squad_state.squads.get(&squad_id) else {
            continue;
        };
        main_inner = main_inner.child(squad_card(
            squad,
            squad_state,
            inspector_state,
            char_query,
            location_query,
        ));
    }
    main_scroll = main_scroll.child(main_inner);

    let main_pane = div()
        .col()
        .h_full()
        .flex_grow(1.0)
        .min_w(Val::Px(0.0))
        .child(main_scroll);

    // Top-level row: main + event log sidebar.
    let root = div()
        .flex()
        .row()
        .w_full()
        .h_full()
        .child(main_pane)
        .child(event_log_sidebar(event_log, sim_state));

    let root_entity = root.spawn(commands).id();
    commands.entity(view).add_child(root_entity);
}

// --- Squad card -------------------------------------------------------------

fn squad_card(
    squad: &GameSquad,
    squad_state: &SquadState,
    inspector_state: &InspectorState,
    char_query: &Query<(&CharacterInfo, &Skills, &ActionState)>,
    location_query: &Query<(&LocationInfo, Option<&LocationResources>)>,
) -> Div {
    let (badge_text, badge_color) = match squad.status {
        SquadStatus::Active => ("ACTIVE", PRIMARY_500),
        SquadStatus::Traveling => ("TRAVELING", WARNING_500),
        SquadStatus::Idle => ("IDLE", GRAY_500),
    };
    let status_color = match squad.status {
        SquadStatus::Active | SquadStatus::Traveling => SUCCESS_400,
        SquadStatus::Idle => TEXT_MUTED,
    };
    let operation = derive_squad_operation(squad, squad_state, char_query);
    let status_text = derive_squad_status_text(squad, squad_state, char_query);

    let header = div()
        .flex()
        .row()
        .w_full()
        .justify_between()
        .items_start()
        .mb(px(SPACE_5))
        .child(
            div()
                .col()
                .child(
                    div()
                        .flex()
                        .row()
                        .items_center()
                        .gap_x(px(SPACE_2))
                        .mb(px(SPACE_1))
                        .child(text(squad.name.clone()).font_size(TEXT_XL).color(TEXT_PRIMARY))
                        .child(
                            div()
                                .pad_x(px(SPACE_2))
                                .py(px(SPACE_0_5))
                                .bg(badge_color.with_alpha(0.15))
                                .rounded(px(RADIUS_DEFAULT))
                                .child(text(badge_text).font_size(TEXT_XS).color(badge_color)),
                        ),
                )
                .child(
                    text(format!("Operation: {}", operation))
                        .font_size(TEXT_SM)
                        .color(TEXT_MUTED),
                ),
        )
        .child(
            div()
                .col()
                .items_end()
                .child(
                    text("Squad Status")
                        .font_size(TEXT_XS)
                        .color(TEXT_MUTED)
                        .mb(px(SPACE_0_5)),
                )
                .child(text(status_text).font_size(TEXT_SM).color(status_color)),
        );

    // Progress bar (only if a member of this squad is selected and active).
    let progress = selected_progress(squad, squad_state, inspector_state, char_query);

    // Agent grid label + grid.
    let agent_grid_label = text("Select Agent to Configure")
        .font_size(TEXT_XS)
        .color(TEXT_MUTED)
        .mb(px(SPACE_3));

    let mut agent_grid = div()
        .flex()
        .row()
        .w_full()
        .flex_wrap()
        .gap_x(px(SPACE_4))
        .gap_y(px(SPACE_4));
    for member_id in &squad.members {
        let Some(&entity) = squad_state.characters.get(member_id) else {
            continue;
        };
        let Ok((info, skills, action_state)) = char_query.get(entity) else {
            continue;
        };
        let is_selected = inspector_state.selected_character_id.as_deref() == Some(&info.id);
        agent_grid = agent_grid.child(agent_card(info, skills, action_state, is_selected));
    }

    // Job queue (only if a member of this squad is selected).
    let selected_member = inspector_state
        .selected_character_id
        .as_ref()
        .filter(|id| squad.members.contains(id));
    let mut card = div()
        .col()
        .w_full()
        .bg(SURFACE_RAISED)
        .rounded(px(RADIUS_XL))
        .p(px(SPACE_5))
        .child(header);
    if let Some(p) = progress {
        card = card.child(p);
    }
    card = card.child(agent_grid_label).child(agent_grid);
    if let Some(selected_id) = selected_member {
        if let Some(&entity) = squad_state.characters.get(selected_id) {
            if let Ok((info, _, action_state)) = char_query.get(entity) {
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
                card = card.child(job_queue_panel(
                    entity,
                    info,
                    action_state,
                    &gather_locations,
                    inspector_state.job_picker_mode,
                ));
            }
        }
    }
    card
}

// --- Selected character progress bar ----------------------------------------

fn selected_progress(
    squad: &GameSquad,
    squad_state: &SquadState,
    inspector_state: &InspectorState,
    char_query: &Query<(&CharacterInfo, &Skills, &ActionState)>,
) -> Option<Div> {
    let selected_id = inspector_state.selected_character_id.as_ref()?;
    if !squad.members.contains(selected_id) {
        return None;
    }
    let &entity = squad_state.characters.get(selected_id)?;
    let (_, _, action_state) = char_query.get(entity).ok()?;
    if action_state.current_action.is_none()
        || matches!(action_state.current_action, Some(Action::Idle))
        || action_state.progress.required == 0
    {
        return None;
    }
    let fraction = action_state.progress.fraction();
    let pct = (fraction * 100.0) as u32;
    let action_text = format_action_short(&action_state.current_action);

    let header = div()
        .flex()
        .row()
        .w_full()
        .justify_between()
        .items_end()
        .mb(px(SPACE_2))
        .child(
            text("MISSION PROGRESS")
                .font_size(TEXT_XS)
                .color(TEXT_MUTED),
        )
        .child(text(format!("{}%", pct)).font_size(TEXT_SM).color(SUCCESS_400));

    let track = div()
        .w_full()
        .h(Val::Px(10.0))
        .bg(GRAY_800)
        .rounded(px(RADIUS_FULL))
        .mb(px(SPACE_1))
        .child(
            div()
                .w(Val::Percent(fraction * 100.0))
                .h(Val::Percent(100.0))
                .bg(SUCCESS_500)
                .rounded(px(RADIUS_FULL)),
        );

    Some(
        div()
            .col()
            .w_full()
            .mb(px(SPACE_6))
            .child(header)
            .child(track)
            .child(text(action_text).font_size(TEXT_XS).color(TEXT_MUTED)),
    )
}

// --- Agent card -------------------------------------------------------------

fn agent_card(
    info: &CharacterInfo,
    skills: &Skills,
    action_state: &ActionState,
    is_selected: bool,
) -> Div {
    let level = compute_average_level(skills);
    let (bg_color, _border_color) = if is_selected {
        (GRAY_900, PRIMARY_500)
    } else {
        (Color::srgba(0.15, 0.15, 0.15, 0.5), GRAY_600)
    };
    let avatar_bg = if is_selected { PRIMARY_700 } else { GRAY_700 };
    let badge_bg = if is_selected {
        PRIMARY_500.with_alpha(0.2)
    } else {
        GRAY_700
    };
    let badge_text_color = if is_selected { PRIMARY_400 } else { GRAY_400 };
    let name_color = if is_selected { TEXT_PRIMARY } else { GRAY_300 };

    let first_char = info
        .name
        .chars()
        .next()
        .unwrap_or('?')
        .to_uppercase()
        .to_string();

    let avatar = div()
        .w(Val::Px(48.0))
        .h(Val::Px(48.0))
        .items_center()
        .justify_center()
        .bg(avatar_bg)
        .rounded(px(RADIUS_FULL))
        .child(text(first_char).font_size(TEXT_LG).color(TEXT_PRIMARY));

    let level_badge = div()
        .pad_x(px(SPACE_1_5))
        .py(px(SPACE_0_5))
        .bg(badge_bg)
        .rounded(px(RADIUS_DEFAULT))
        .child(text(format!("Lvl {}", level)).font_size(10.0).color(badge_text_color));

    let (status_text, status_color) = action_status_display(action_state);

    let body = div()
        .col()
        .flex_grow(1.0)
        .child(
            div()
                .flex()
                .row()
                .items_center()
                .gap_x(px(SPACE_2))
                .child(text(info.name.clone()).color(name_color))
                .child(level_badge),
        )
        .child(text(status_text).font_size(TEXT_XS).color(status_color));

    div()
        .min_w(Val::Px(180.0))
        .flex_grow(1.0)
        .p(px(SPACE_3))
        .bg(bg_color)
        .rounded(px(RADIUS_LG))
        .insert(AgentCardButton(info.id.clone()))
        .on_click(agent_card_on_click)
        .child(
            div()
                .flex()
                .row()
                .items_center()
                .gap_x(px(SPACE_3))
                .w_full()
                .child(avatar)
                .child(body),
        )
}

// --- Job queue panel --------------------------------------------------------

fn job_queue_panel(
    entity_id: Entity,
    info: &CharacterInfo,
    action_state: &ActionState,
    gather_locations: &[(String, String, String)],
    picker_mode: JobPickerMode,
) -> Div {
    let header = div()
        .flex()
        .row()
        .w_full()
        .justify_between()
        .items_center()
        .p(px(SPACE_5))
        .child(
            div()
                .col()
                .child(text("Job Queue").font_size(TEXT_LG).color(TEXT_PRIMARY))
                .child(
                    div()
                        .flex()
                        .row()
                        .items_center()
                        .gap_x(px(SPACE_2))
                        .mt(px(SPACE_1))
                        .child(
                            text("Configuration for Agent:")
                                .font_size(TEXT_SM)
                                .color(TEXT_MUTED),
                        )
                        .child(
                            div()
                                .pad_x(px(SPACE_2))
                                .py(px(SPACE_0_5))
                                .bg(PRIMARY_500.with_alpha(0.1))
                                .rounded(px(RADIUS_DEFAULT))
                                .child(text(info.name.clone()).font_size(TEXT_XS).color(PRIMARY_400)),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .row()
                .gap_x(px(SPACE_2))
                .child(
                    div()
                        .pad_x(px(SPACE_3))
                        .py(px(SPACE_1_5))
                        .bg(GRAY_800)
                        .rounded(px(RADIUS_DEFAULT))
                        .insert(ClearJobsButton(entity_id))
                        .on_click(clear_jobs_on_click)
                        .child(text("Clear Jobs").font_size(TEXT_XS).color(GRAY_300)),
                )
                .child(
                    div()
                        .pad_x(px(SPACE_3))
                        .py(px(SPACE_1_5))
                        .bg(GRAY_800)
                        .rounded(px(RADIUS_DEFAULT))
                        .insert(ClearActionsButton(entity_id))
                        .on_click(clear_actions_on_click)
                        .child(text("Clear Actions").font_size(TEXT_XS).color(GRAY_300)),
                ),
        );

    let table_header = div()
        .flex()
        .row()
        .w_full()
        .pad_x(px(SPACE_5))
        .py(px(SPACE_3))
        .bg(Color::srgba(0.1, 0.1, 0.1, 0.5))
        .child(
            div()
                .w(Val::Px(40.0))
                .child(text("#").font_size(TEXT_XS).color(TEXT_MUTED)),
        )
        .child(
            div()
                .flex_grow(1.0)
                .child(text("JOB NAME").font_size(TEXT_XS).color(TEXT_MUTED)),
        )
        .child(
            div()
                .w(Val::Px(80.0))
                .child(text("STEPS").font_size(TEXT_XS).color(TEXT_MUTED)),
        )
        .child(
            div()
                .w(Val::Px(80.0))
                .child(text("STATUS").font_size(TEXT_XS).color(TEXT_MUTED)),
        );

    let mut body = div().col().w_full();
    if action_state.job_queue.is_empty() {
        body = body.child(
            div().p(px(SPACE_5)).child(
                text("No jobs assigned")
                    .font_size(TEXT_SM)
                    .color(TEXT_MUTED),
            ),
        );
    } else {
        for (i, job) in action_state.job_queue.iter().enumerate() {
            let is_current = action_state.current_job_index > 0
                && (action_state.current_job_index - 1) % action_state.job_queue.len() == i;
            body = body.child(job_row(i, job, is_current));
        }
    }

    let mut panel = div()
        .col()
        .w_full()
        .mt(px(SPACE_5))
        .bg(SURFACE_RAISED)
        .rounded(px(RADIUS_XL))
        .child(header)
        .child(table_header)
        .child(body);

    // Add-job UI: either the "+ Add Gather Job" button or the picker.
    panel = match picker_mode {
        JobPickerMode::None => panel.child(add_job_buttons(gather_locations.is_empty())),
        JobPickerMode::GatherPicker => {
            panel.child(gather_picker(entity_id, gather_locations))
        }
    };

    panel
}

fn job_row(i: usize, job: &crate::game::action::Job, is_current: bool) -> Div {
    let row_bg = if is_current {
        Color::srgba(0.15, 0.15, 0.15, 0.5)
    } else {
        Color::NONE
    };
    let badge_color = job_type_color(&job.name);

    div()
        .flex()
        .row()
        .w_full()
        .items_center()
        .pad_x(px(SPACE_5))
        .py(px(SPACE_3))
        .bg(row_bg)
        .child(
            div()
                .w(Val::Px(40.0))
                .child(text(format!("{}", i + 1)).font_size(TEXT_SM).color(GRAY_400)),
        )
        .child(
            div()
                .flex_grow(1.0)
                .flex()
                .row()
                .items_center()
                .gap_x(px(SPACE_3))
                .child(
                    div()
                        .pad_x(px(SPACE_2))
                        .py(px(SPACE_1))
                        .bg(badge_color.with_alpha(0.1))
                        .rounded(px(RADIUS_DEFAULT))
                        .child(text(job.name.clone()).font_size(TEXT_XS).color(badge_color)),
                ),
        )
        .child(
            div()
                .w(Val::Px(80.0))
                .child(
                    text(format!("{}", job.actions.len()))
                        .font_size(TEXT_SM)
                        .color(GRAY_400),
                ),
        )
        .child(div().w(Val::Px(80.0)).child(if is_current {
            text("Active").font_size(TEXT_XS).color(SUCCESS_400)
        } else {
            text("Queued").font_size(TEXT_XS).color(GRAY_500)
        }))
}

fn add_job_buttons(no_locations: bool) -> Div {
    let row = div()
        .flex()
        .row()
        .w_full()
        .p(px(SPACE_5))
        .gap_x(px(SPACE_2));
    if no_locations {
        row.child(
            text("No resource locations available")
                .font_size(TEXT_XS)
                .color(TEXT_MUTED),
        )
    } else {
        row.child(
            div()
                .pad_x(px(SPACE_3))
                .py(px(SPACE_1_5))
                .bg(PRIMARY_500.with_alpha(0.2))
                .rounded(px(RADIUS_DEFAULT))
                .insert(AddGatherJobButton)
                .on_click(add_gather_on_click)
                .child(
                    text("+ Add Gather Job")
                        .font_size(TEXT_XS)
                        .color(PRIMARY_400),
                ),
        )
    }
}

fn gather_picker(entity_id: Entity, gather_locations: &[(String, String, String)]) -> Div {
    let mut col = div()
        .col()
        .w_full()
        .p(px(SPACE_5))
        .gap_y(px(SPACE_1))
        .child(
            div()
                .pad_x(px(SPACE_3))
                .py(px(SPACE_1_5))
                .bg(GRAY_800)
                .rounded(px(RADIUS_DEFAULT))
                .insert(BackPickerButton)
                .on_click(back_picker_on_click)
                .child(text("<- Back").font_size(TEXT_XS).color(GRAY_300)),
        );
    for (loc_id, loc_name, resource) in gather_locations {
        let label_text = format!("{} ({})", loc_name, resource);
        col = col.child(
            div()
                .w_full()
                .pad_x(px(SPACE_3))
                .py(px(SPACE_1_5))
                .bg(GRAY_800)
                .rounded(px(RADIUS_DEFAULT))
                .insert(LocationGatherButton {
                    entity: entity_id,
                    location_id: loc_id.clone(),
                    resource: resource.clone(),
                })
                .on_click(location_gather_on_click)
                .child(text(label_text).font_size(TEXT_XS).color(GRAY_100)),
        );
    }
    col
}

// --- Event log sidebar ------------------------------------------------------

fn event_log_sidebar(event_log: &EventLog, sim_state: &SimulationState) -> Div {
    let header = div()
        .flex()
        .row()
        .w_full()
        .justify_between()
        .items_center()
        .p(px(SPACE_4))
        .bg(SURFACE_RAISED)
        .child(text("EVENT LOG").font_size(TEXT_SM).color(TEXT_PRIMARY));

    let mut entries_col = div().col().w_full().gap_y(px(SPACE_4));
    if event_log.entries.is_empty() {
        entries_col = entries_col.child(
            text("No events yet")
                .font_size(TEXT_XS)
                .color(TEXT_MUTED),
        );
    } else {
        for entry in &event_log.entries {
            entries_col = entries_col.child(event_log_entry(entry, sim_state));
        }
    }

    let scroll = scroll_view()
        .vertical(true)
        .child(div().col().p(px(SPACE_4)).child(entries_col));

    div()
        .col()
        .w(Val::Px(280.0))
        .h_full()
        .bg(SURFACE_BASE)
        .child(header)
        .child(scroll)
}

fn event_log_entry(entry: &EventLogEntry, sim_state: &SimulationState) -> Div {
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
    let ticks_ago = sim_state.game_time.saturating_sub(entry.game_tick);
    let time_text = if ticks_ago < 60 {
        format!("{}t ago", ticks_ago)
    } else if ticks_ago < 3600 {
        format!("{}m ago", ticks_ago / 60)
    } else {
        format!("{}h ago", ticks_ago / 3600)
    };

    div()
        .flex()
        .row()
        .w_full()
        .pl(px(SPACE_4))
        .child(
            div()
                .w(Val::Px(10.0))
                .h(Val::Px(10.0))
                .bg(dot_color)
                .rounded(px(RADIUS_FULL))
                .mt(px(SPACE_0_5)),
        )
        .child(
            div()
                .col()
                .gap_y(px(SPACE_0_5))
                .pl(px(SPACE_2))
                .child(text(time_text).font_size(10.0).color(TEXT_MUTED))
                .child(text(entry.message.clone()).font_size(TEXT_XS).color(text_color)),
        )
}

// --- Pure helpers (preserved verbatim from pre-migration) -------------------

fn derive_squad_operation(
    squad: &GameSquad,
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

fn derive_squad_status_text(
    squad: &GameSquad,
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

fn compute_average_level(skills: &Skills) -> u32 {
    let total: u32 = skills.iter().map(|(_, xp)| xp_to_level(xp)).sum();
    let count = skills.iter().count() as u32;
    if count == 0 {
        1
    } else {
        (total / count).max(1)
    }
}

fn xp_to_level(xp: u32) -> u32 {
    let xp = xp as f64;
    ((xp * 4.0 / 5.0).cbrt().floor() as u32 + 1).min(100)
}

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
