//! Character inspector — tabbed view (Health / Equipment / Skills / Inventory
//! / Jobs) for the currently selected character. Spawned as a child of the
//! `CharacterInspector` marker entity created by the layout.
//!
//! Spawned once on `On<Add, CharacterInspector>`; rebuilt whenever any source
//! changes. All click handlers (tab buttons, clear buttons, job picker)
//! preserved via marker components + observer systems.

use bevy::prelude::*;
use bevy_declarative::prelude::px;

use crate::game::action::{Action, ActionState, make_gather_job};
use crate::game::character::{Equipment, Health, Inventory, Skills};
use crate::game::location::{LocationInfo, LocationResources};
use crate::game::resources::SquadState;
use crate::screens::Screen;
use crate::ui::prelude::*;

#[derive(Resource, Default)]
pub struct InspectorState {
    pub selected_character_id: Option<String>,
    pub active_tab: InspectorTab,
    pub job_picker_mode: JobPickerMode,
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum InspectorTab {
    #[default]
    Health,
    Equipment,
    Skills,
    Inventory,
    Jobs,
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum JobPickerMode {
    #[default]
    None,
    GatherPicker,
}

#[derive(Component)]
pub struct CharacterInspector;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(populate_inspector_on_add);
    app.add_systems(
        Update,
        refresh_inspector.run_if(in_state(Screen::Gameplay)),
    );
}

// --- Click marker components ------------------------------------------------

#[derive(Component)]
struct TabButton(InspectorTab);

#[derive(Component)]
struct InspectorClearActionsButton(Entity);

#[derive(Component)]
struct InspectorClearJobsButton(Entity);

#[derive(Component)]
struct InspectorAddGatherButton;

#[derive(Component)]
struct InspectorBackPickerButton;

#[derive(Component)]
struct InspectorLocationGatherButton {
    entity: Entity,
    location_id: String,
    resource: String,
}

// --- Click observer systems -------------------------------------------------

fn tab_on_click(
    click: On<Pointer<Click>>,
    q: Query<&TabButton>,
    mut inspector: ResMut<InspectorState>,
) {
    if let Ok(marker) = q.get(click.entity) {
        inspector.active_tab = marker.0;
        inspector.job_picker_mode = JobPickerMode::None;
    }
}

fn clear_actions_on_click(
    click: On<Pointer<Click>>,
    q: Query<&InspectorClearActionsButton>,
    mut action_query: Query<&mut ActionState>,
) {
    if let Ok(marker) = q.get(click.entity) {
        if let Ok(mut state) = action_query.get_mut(marker.0) {
            state.clear();
        }
    }
}

fn clear_jobs_on_click(
    click: On<Pointer<Click>>,
    q: Query<&InspectorClearJobsButton>,
    mut action_query: Query<&mut ActionState>,
) {
    if let Ok(marker) = q.get(click.entity) {
        if let Ok(mut state) = action_query.get_mut(marker.0) {
            state.clear_jobs();
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
    q: Query<&InspectorLocationGatherButton>,
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
fn populate_inspector_on_add(
    add: On<Add, CharacterInspector>,
    mut commands: Commands,
    squad_state: Res<SquadState>,
    inspector_state: Res<InspectorState>,
    char_query: Query<(&Health, &Skills, &Equipment, &Inventory, &ActionState)>,
    location_query: Query<(&LocationInfo, Option<&LocationResources>)>,
) {
    spawn_inspector_children(
        &mut commands,
        add.entity,
        &squad_state,
        &inspector_state,
        &char_query,
        &location_query,
    );
}

#[allow(clippy::too_many_arguments)]
fn refresh_inspector(
    mut commands: Commands,
    view_q: Query<Entity, With<CharacterInspector>>,
    squad_state: Res<SquadState>,
    inspector_state: Res<InspectorState>,
    char_query: Query<(&Health, &Skills, &Equipment, &Inventory, &ActionState)>,
    location_query: Query<(&LocationInfo, Option<&LocationResources>)>,
    changed_chars: Query<
        (),
        Or<(
            Changed<Health>,
            Changed<Skills>,
            Changed<Equipment>,
            Changed<Inventory>,
            Changed<ActionState>,
        )>,
    >,
    changed_locs: Query<(), Or<(Changed<LocationInfo>, Changed<LocationResources>)>>,
) {
    let any_changed = squad_state.is_changed()
        || inspector_state.is_changed()
        || !changed_chars.is_empty()
        || !changed_locs.is_empty();
    if !any_changed {
        return;
    }
    for view in &view_q {
        commands.entity(view).despawn_related::<Children>();
        spawn_inspector_children(
            &mut commands,
            view,
            &squad_state,
            &inspector_state,
            &char_query,
            &location_query,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_inspector_children(
    commands: &mut Commands,
    view: Entity,
    squad_state: &SquadState,
    inspector_state: &InspectorState,
    char_query: &Query<(&Health, &Skills, &Equipment, &Inventory, &ActionState)>,
    location_query: &Query<(&LocationInfo, Option<&LocationResources>)>,
) {
    // Validation gates: produce an early-return placeholder tree.
    let placeholder = |msg: &str| -> Div {
        div()
            .col()
            .w_full()
            .h_full()
            .items_center()
            .justify_center()
            .child(label(msg.to_string()).color(Color::srgb(0.6, 0.6, 0.6)))
    };

    let Some(char_id) = &inspector_state.selected_character_id else {
        let entity = placeholder("Select a character to inspect").spawn(commands).id();
        commands.entity(view).add_child(entity);
        return;
    };
    let Some(&entity) = squad_state.characters.get(char_id) else {
        let entity = placeholder("Character not found").spawn(commands).id();
        commands.entity(view).add_child(entity);
        return;
    };
    let Ok((health, skills, equipment, inventory, action_state)) = char_query.get(entity) else {
        let entity = placeholder("Character missing data").spawn(commands).id();
        commands.entity(view).add_child(entity);
        return;
    };

    // Collect gather location data for job picker.
    let gather_locations: Vec<(String, String, String)> = location_query
        .iter()
        .filter(|(info, res)| info.discovered && res.is_some())
        .filter_map(|(info, res)| {
            let res = res?;
            if res.resource_type.is_empty() || res.current_amount == 0 {
                return None;
            }
            Some((info.id.clone(), info.name.clone(), res.resource_type.clone()))
        })
        .collect();

    let active = inspector_state.active_tab;

    let tab_bar = div()
        .flex()
        .row()
        .w_full()
        .mb(px(SPACE_2_5))
        .gap_x(px(SPACE_1))
        .child(tab_button("Health", InspectorTab::Health, active))
        .child(tab_button("Equipment", InspectorTab::Equipment, active))
        .child(tab_button("Skills", InspectorTab::Skills, active))
        .child(tab_button("Inventory", InspectorTab::Inventory, active))
        .child(tab_button("Jobs", InspectorTab::Jobs, active));

    let body = match active {
        InspectorTab::Health => health_tab(health),
        InspectorTab::Equipment => equipment_tab(equipment),
        InspectorTab::Skills => skills_tab(skills),
        InspectorTab::Inventory => inventory_tab(inventory),
        InspectorTab::Jobs => jobs_tab(
            entity,
            action_state,
            &gather_locations,
            inspector_state.job_picker_mode,
        ),
    };

    let root = div()
        .col()
        .flex_grow(1.0)
        .p(px(SPACE_4))
        .child(action_status_header(entity, action_state))
        .child(divider())
        .child(tab_bar)
        .child(body);

    let root_entity = root.spawn(commands).id();
    commands.entity(view).add_child(root_entity);
}

// --- Tab content builders ---------------------------------------------------

fn health_tab(health: &Health) -> Div {
    let mut col = div().col().w_full();
    for (part, hp) in health.iter() {
        let color = if hp > 80 {
            Color::srgb(0.0, 1.0, 0.0)
        } else if hp > 40 {
            Color::srgb(1.0, 1.0, 0.0)
        } else {
            Color::srgb(1.0, 0.0, 0.0)
        };
        col = col.child(
            div()
                .flex()
                .row()
                .justify_between()
                .w_full()
                .mb(px(SPACE_1))
                .child(label(part.to_string()).color(Color::srgb(0.8, 0.8, 0.8)))
                .child(text(format!("{}", hp)).color(color)),
        );
    }
    col.child(label("Status").mt(px(SPACE_2_5)).mb(px(SPACE_1)))
        .child(label(format!("Hunger: {}", health.hunger)))
}

fn equipment_tab(eq: &Equipment) -> Div {
    div()
        .col()
        .w_full()
        .child(equip_slot_row("Head", &eq.head))
        .child(equip_slot_row("Chest", &eq.chest))
        .child(equip_slot_row("Legs", &eq.legs))
        .child(equip_slot_row("Feet", &eq.feet))
        .child(equip_slot_row("Hands", &eq.hands))
        .child(equip_slot_row("Main Hand", &eq.main_hand))
}

fn equip_slot_row(slot_name: &str, item: &Option<String>) -> Div {
    let value: TextEl = match item {
        Some(name) => text(name.clone()).color(Color::WHITE),
        None => text("Empty").color(Color::srgb(0.3, 0.3, 0.3)),
    };
    div()
        .flex()
        .row()
        .justify_between()
        .w_full()
        .mb(px(SPACE_1))
        .child(label(slot_name.to_string()).color(Color::srgb(0.7, 0.7, 0.7)))
        .child(value)
}

fn skills_tab(skills: &Skills) -> Div {
    let mut col = div().col().w_full();
    for (skill, xp) in skills.iter() {
        col = col.child(
            div()
                .flex()
                .row()
                .justify_between()
                .w_full()
                .mb(px(SPACE_0_5))
                .child(label(skill.to_string()).color(Color::srgb(0.8, 0.8, 0.8)))
                .child(text(format!("{}", xp_to_level(xp))).color(Color::WHITE)),
        );
    }
    div().w_full().h_full().child(scroll_view().vertical(true).child(col))
}

fn inventory_tab(inventory: &Inventory) -> Div {
    if inventory.items.is_empty() {
        return div()
            .col()
            .w_full()
            .child(label("Empty").color(Color::srgb(0.5, 0.5, 0.5)));
    }
    let mut col = div().col().w_full();
    for (item, count) in &inventory.items {
        col = col.child(
            div()
                .flex()
                .row()
                .justify_between()
                .w_full()
                .child(label(format!("{}: {}", item, count)))
                .child(
                    div()
                        .pad_x(px(SPACE_2))
                        .py(px(SPACE_0_5))
                        .bg(GRAY_700)
                        .rounded(Val::Px(2.0))
                        .child(text("Drop").color(Color::WHITE)),
                ),
        );
    }
    col
}

fn jobs_tab(
    entity_id: Entity,
    action_state: &ActionState,
    gather_locations: &[(String, String, String)],
    picker_mode: JobPickerMode,
) -> Div {
    let mut col = div().col().w_full().h_full();

    if action_state.job_queue.is_empty() {
        col = col.child(
            label("No jobs assigned")
                .color(Color::srgb(0.5, 0.5, 0.5))
                .mb(px(SPACE_2)),
        );
    } else {
        col = col.child(
            text("Job Queue:")
                .font_size(13.0)
                .color(Color::srgb(0.8, 0.8, 0.8))
                .mb(px(SPACE_1)),
        );
        for (i, job) in action_state.job_queue.iter().enumerate() {
            let is_current = action_state.current_job_index > 0
                && (action_state.current_job_index - 1) % action_state.job_queue.len() == i;
            let prefix = if is_current { "> " } else { "  " };
            let color = if is_current {
                Color::WHITE
            } else {
                Color::srgb(0.7, 0.7, 0.7)
            };
            col = col.child(
                div()
                    .flex()
                    .row()
                    .w_full()
                    .justify_between()
                    .mb(px(SPACE_0_5))
                    .child(
                        text(format!(
                            "{}{}. {} ({} steps)",
                            prefix,
                            i + 1,
                            job.name,
                            job.actions.len()
                        ))
                        .font_size(12.0)
                        .color(color),
                    ),
            );
        }
    }

    col = col.child(divider());

    match picker_mode {
        JobPickerMode::None => {
            if !gather_locations.is_empty() {
                col = col.child(
                    div()
                        .w_full()
                        .pad_x(px(SPACE_2))
                        .py(px(SPACE_1))
                        .bg(GRAY_700)
                        .rounded(Val::Px(2.0))
                        .insert(InspectorAddGatherButton)
                        .on_click(add_gather_on_click)
                        .child(text("+ Add Gather Job").font_size(12.0).color(GRAY_100)),
                );
            } else {
                col = col.child(
                    text("No resource locations available")
                        .font_size(11.0)
                        .color(Color::srgb(0.5, 0.5, 0.5)),
                );
            }
        }
        JobPickerMode::GatherPicker => {
            col = col.child(
                text("Select resource to gather:")
                    .font_size(12.0)
                    .color(Color::srgb(0.8, 0.8, 0.8))
                    .mb(px(SPACE_1)),
            );
            col = col.child(
                div()
                    .w_full()
                    .pad_x(px(SPACE_2))
                    .py(px(SPACE_1))
                    .bg(GRAY_700)
                    .rounded(Val::Px(2.0))
                    .mb(px(SPACE_0_5))
                    .insert(InspectorBackPickerButton)
                    .on_click(back_picker_on_click)
                    .child(text("<- Back").font_size(12.0).color(GRAY_100)),
            );
            for (loc_id, loc_name, resource) in gather_locations {
                let label_text = format!("{} ({})", loc_name, resource);
                col = col.child(
                    div()
                        .w_full()
                        .pad_x(px(SPACE_2))
                        .py(px(SPACE_1))
                        .bg(GRAY_700)
                        .rounded(Val::Px(2.0))
                        .mb(px(SPACE_0_5))
                        .insert(InspectorLocationGatherButton {
                            entity: entity_id,
                            location_id: loc_id.clone(),
                            resource: resource.clone(),
                        })
                        .on_click(location_gather_on_click)
                        .child(text(label_text).font_size(12.0).color(GRAY_100)),
                );
            }
        }
    }
    col
}

// --- Action status header ---------------------------------------------------

fn action_status_header(entity_id: Entity, action_state: &ActionState) -> Div {
    let action_text = match &action_state.current_action {
        Some(action) => format_action(action),
        None => "Idle".to_string(),
    };

    let mut col = div().col().w_full().mb(px(SPACE_1)).child(
        div()
            .flex()
            .row()
            .w_full()
            .mb(px(SPACE_0_5))
            .child(
                text("Action: ")
                    .font_size(12.0)
                    .color(Color::srgb(0.6, 0.6, 0.6)),
            )
            .child(text(action_text).font_size(12.0).color(Color::WHITE)),
    );

    if action_state.current_action.is_some()
        && !matches!(action_state.current_action, Some(Action::Idle))
        && action_state.progress.required > 0
    {
        let fraction = action_state.progress.fraction();
        let pct = (fraction * 100.0) as u32;
        col = col.child(
            div()
                .flex()
                .row()
                .w_full()
                .mb(px(SPACE_0_5))
                .child(
                    div()
                        .w(Val::Percent(70.0))
                        .h(Val::Px(8.0))
                        .bg(GRAY_700)
                        .rounded(Val::Px(2.0))
                        .child(
                            div()
                                .w(Val::Percent(fraction * 100.0))
                                .h(Val::Percent(100.0))
                                .bg(PRIMARY_500)
                                .rounded(Val::Px(2.0)),
                        ),
                )
                .child(
                    text(format!(" {}%", pct))
                        .font_size(11.0)
                        .color(Color::srgb(0.7, 0.7, 0.7)),
                ),
        );
    }

    let queue_count = action_state.action_queue.len();
    let job_count = action_state.job_queue.len();
    col = col.child(
        div()
            .flex()
            .row()
            .w_full()
            .gap_x(px(SPACE_3))
            .child(
                text(format!("Queued: {}", queue_count))
                    .font_size(11.0)
                    .color(Color::srgb(0.6, 0.6, 0.6)),
            )
            .child(
                text(format!("Jobs: {}", job_count))
                    .font_size(11.0)
                    .color(Color::srgb(0.6, 0.6, 0.6)),
            ),
    );

    col.child(
        div()
            .flex()
            .row()
            .w_full()
            .mt(px(SPACE_1))
            .gap_x(px(SPACE_1))
            .child(
                div()
                    .pad_x(px(SPACE_1_5))
                    .py(px(SPACE_0_5))
                    .bg(GRAY_700)
                    .rounded(Val::Px(2.0))
                    .insert(InspectorClearActionsButton(entity_id))
                    .on_click(clear_actions_on_click)
                    .child(
                        text("Clear Actions")
                            .font_size(11.0)
                            .color(Color::srgb(0.8, 0.8, 0.8)),
                    ),
            )
            .child(
                div()
                    .pad_x(px(SPACE_1_5))
                    .py(px(SPACE_0_5))
                    .bg(GRAY_700)
                    .rounded(Val::Px(2.0))
                    .insert(InspectorClearJobsButton(entity_id))
                    .on_click(clear_jobs_on_click)
                    .child(
                        text("Clear Jobs")
                            .font_size(11.0)
                            .color(Color::srgb(0.8, 0.8, 0.8)),
                    ),
            ),
    )
}

// --- Helpers ---------------------------------------------------------------

fn tab_button(label_text: &str, tab: InspectorTab, active: InspectorTab) -> Div {
    let is_active = tab == active;
    let color = if is_active {
        Color::WHITE
    } else {
        Color::srgb(0.6, 0.6, 0.6)
    };
    let mut btn = div()
        .pad_x(px(SPACE_2))
        .py(px(SPACE_1))
        .bg(Color::NONE)
        .insert(TabButton(tab))
        .on_click(tab_on_click)
        .child(text(label_text).color(color));
    if is_active {
        btn = btn.bg(GRAY_800);
    }
    btn
}

fn divider() -> Div {
    div()
        .w_full()
        .h(Val::Px(1.0))
        .my(px(SPACE_1_5))
        .bg(GRAY_700)
}

fn format_action(action: &Action) -> String {
    match action {
        Action::Idle => "Idle".to_string(),
        Action::Explore => "Exploring".to_string(),
        Action::Travel { destination } => format!("Traveling to {}", destination),
        Action::Gather { location, resource } => {
            format!("Gathering {} at {}", resource, location)
        }
        Action::Collect { location, item } => format!("Collecting {} at {}", item, location),
        Action::Deposit { item } => format!("Depositing {}", item),
        Action::Research { tech_id } => format!("Researching {}", tech_id),
        Action::Craft {
            recipe_id,
            workstation,
        } => format!("Crafting {} at {}", recipe_id, workstation),
        Action::Build { building } => format!("Building {}", building),
    }
}

fn xp_to_level(xp: u32) -> u32 {
    let xp = xp as f64;
    ((xp * 4.0 / 5.0).cbrt().floor() as u32 + 1).min(100)
}
