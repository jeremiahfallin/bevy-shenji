# Squads View Redesign Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Rebuild the Squads view to match the Command Dashboard Alpha mockup with squad card, agent selection grid, job queue table, and event log sidebar.

**Architecture:** Monolithic `SquadsView` component using `ImmediateAttach<CapsUi>`, matching existing view patterns (Dashboard, Locations). New `EventLog` resource for persistent log entries. `Squad` struct extended with `SquadStatus`. All rendering in one `construct()` with helper functions for each section.

**Tech Stack:** Rust, Bevy 0.17, bevy_immediate 0.4, existing theme system (palette, spacing, typography, borders)

---

### Task 1: Add EventLog Resource

**Files:**
- Modify: `src/game/resources.rs` (after `NotificationState` at line ~215)

**Step 1: Add EventLog types to resources.rs**

After the `NotificationState` impl block (line 215), add:

```rust
/// A persistent log entry for the event log sidebar.
#[derive(Debug, Clone, Reflect)]
pub struct EventLogEntry {
    pub message: String,
    pub level: NotificationLevel,
    pub game_tick: u64,
}

/// Persistent event log that keeps a history of game events.
#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct EventLog {
    pub entries: VecDeque<EventLogEntry>,
}

impl EventLog {
    pub const MAX_ENTRIES: usize = 100;

    pub fn push(&mut self, message: impl Into<String>, level: NotificationLevel, game_tick: u64) {
        self.entries.push_front(EventLogEntry {
            message: message.into(),
            level,
            game_tick,
        });
        if self.entries.len() > Self::MAX_ENTRIES {
            self.entries.pop_back();
        }
    }
}
```

**Step 2: Register EventLog in game module**

In `src/game/mod.rs`, add after line 22 (`app.init_resource::<resources::NotificationState>()`):

```rust
app.init_resource::<resources::EventLog>();
```

And add after line 31 (`app.register_type::<resources::NotificationState>()`):

```rust
app.register_type::<resources::EventLog>();
```

**Step 3: Build and verify**

Run: `cargo check`
Expected: Compiles with no errors.

**Step 4: Commit**

```bash
git add src/game/resources.rs src/game/mod.rs
git commit -m "feat: add EventLog resource for persistent game event history"
```

---

### Task 2: Add SquadStatus to Squad

**Files:**
- Modify: `src/game/resources.rs` (Squad struct at line ~42)

**Step 1: Add SquadStatus enum**

Before the `Squad` struct (line 42), add:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect, Serialize, Deserialize)]
pub enum SquadStatus {
    #[default]
    Idle,
    Active,
    Traveling,
}
```

**Step 2: Add status field to Squad**

Update the `Squad` struct to:

```rust
#[derive(Debug, Reflect, Clone, Serialize, Deserialize)]
pub struct Squad {
    pub name: String,
    pub members: Vec<String>,
    #[serde(default)]
    pub status: SquadStatus,
}
```

The `#[serde(default)]` ensures backward compatibility with existing save files.

**Step 3: Build and verify**

Run: `cargo check`
Expected: Compiles with no errors.

**Step 4: Commit**

```bash
git add src/game/resources.rs
git commit -m "feat: add SquadStatus enum to Squad struct"
```

---

### Task 3: Wire EventLog to Notification System

**Files:**
- Modify: `src/game/mod.rs` (tick_notifications system at line ~45)

**Step 1: Create a system to mirror notifications to event log**

The cleanest approach is to add a system that checks for new notifications each frame and copies them to the EventLog. Replace the `tick_notifications` function in `src/game/mod.rs` with:

```rust
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
```

**Step 2: Build and verify**

Run: `cargo check`
Expected: Compiles with no errors.

**Step 3: Commit**

```bash
git add src/game/mod.rs
git commit -m "feat: mirror new notifications to persistent EventLog"
```

---

### Task 4: Add Squad Status Derivation System

**Files:**
- Modify: `src/game/mod.rs`

**Step 1: Add system to derive squad status from member actions**

Add this function to `src/game/mod.rs`:

```rust
fn update_squad_statuses(
    mut squad_state: ResMut<resources::SquadState>,
    action_query: Query<&crate::game::action::ActionState>,
) {
    for squad in squad_state.squads.values_mut() {
        let mut idle_count = 0u32;
        let mut traveling_count = 0u32;
        let mut active_count = 0u32;

        for member_id in &squad.members {
            if let Some(&entity) = squad_state.characters.get(member_id) {
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
```

**NOTE:** This system has a borrow conflict — it reads `squad_state.characters` while mutating `squad_state.squads`. To fix this, clone the character map before the loop:

```rust
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
```

**Step 2: Register the system**

In the `plugin` function of `src/game/mod.rs`, add after `app.add_systems(Update, tick_notifications);` (line 34):

```rust
app.add_systems(Update, update_squad_statuses);
```

**Step 3: Build and verify**

Run: `cargo check`
Expected: Compiles with no errors.

**Step 4: Commit**

```bash
git add src/game/mod.rs
git commit -m "feat: derive squad status from member action states"
```

---

### Task 5: Rewrite SquadsView — Squad Card Section

**Files:**
- Modify: `src/game/ui/content/squads.rs` (full rewrite)

This is the largest task. We'll rewrite the entire file. The new SquadsView has these sections:
1. Squad card with header, progress bar, agent grid (this task)
2. Job queue table (Task 6)
3. Event log sidebar (Task 7)

**Step 1: Rewrite squads.rs with the new structure**

Replace the entire contents of `src/game/ui/content/squads.rs` with the code below. This step covers the top-level layout and the squad card section:

```rust
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
        Query<'static, 'static, (&'static CharacterInfo, &'static Skills, &'static ActionState)>,
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
            &params.0,
            &params.1,
            &params.2,
            &params.3,
            &params.4,
            &params.5,
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
                                    squad_id,
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

/// Render a single squad card with header, progress, and agent grid.
fn render_squad_card(
    ui: &mut Imm<CapsUi>,
    squad_state: &SquadState,
    inspector_state: &InspectorState,
    char_query: &Query<(&CharacterInfo, &Skills, &ActionState)>,
    location_query: &Query<(&LocationInfo, Option<&LocationResources>)>,
    squad: &crate::game::resources::Squad,
    squad_id: u16,
) {
    // Squad card container
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

                                // Status badge
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

                        // Operation text (derived from most common action)
                        let operation = derive_squad_operation(squad, squad_state, char_query);
                        ui.ch()
                            .label(format!("Operation: {}", operation))
                            .text_size(TEXT_SM)
                            .text_color(TEXT_MUTED);
                    });

                    // Right: squad status text
                    ui.ch().flex_col().items_end().add(|ui| {
                        ui.ch()
                            .label("Squad Status")
                            .text_size(TEXT_XS)
                            .text_color(TEXT_MUTED)
                            .mb(Val::Px(SPACE_0_5));

                        let status_text = derive_squad_status_text(squad, squad_state, char_query);
                        let status_color = match squad.status {
                            SquadStatus::Active => SUCCESS_400,
                            SquadStatus::Traveling => SUCCESS_400,
                            SquadStatus::Idle => TEXT_MUTED,
                        };
                        ui.ch()
                            .label(&status_text)
                            .text_size(TEXT_SM)
                            .text_color(status_color);
                    });
                });

            // --- Selected Character Progress ---
            render_selected_progress(ui, squad_state, inspector_state, char_query);

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
                .gap(Val::Px(SPACE_4))
                .add(|ui| {
                    for member_id in &squad.members {
                        let Some(&entity) = squad_state.characters.get(member_id) else {
                            continue;
                        };
                        let Ok((info, skills, action_state)) = char_query.get(entity) else {
                            continue;
                        };

                        let is_selected = inspector_state.selected_character_id.as_deref()
                            == Some(&info.id);
                        render_agent_card(ui, info, skills, action_state, is_selected);
                    }
                });

            // --- Job Queue for Selected Character ---
            if let Some(selected_id) = &inspector_state.selected_character_id {
                // Check if selected character is in this squad
                if squad.members.contains(selected_id) {
                    if let Some(&entity) = squad_state.characters.get(selected_id) {
                        if let Ok((info, _skills, action_state)) = char_query.get(entity) {
                            // Collect gather location data for job picker
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

/// Render the progress bar for the currently selected character.
fn render_selected_progress(
    ui: &mut Imm<CapsUi>,
    squad_state: &SquadState,
    inspector_state: &InspectorState,
    char_query: &Query<(&CharacterInfo, &Skills, &ActionState)>,
) {
    let Some(selected_id) = &inspector_state.selected_character_id else {
        return;
    };
    let Some(&entity) = squad_state.characters.get(selected_id) else {
        return;
    };
    let Ok((_info, _skills, action_state)) = char_query.get(entity) else {
        return;
    };

    // Only show if there's a non-idle action with progress
    if !action_state.current_action.is_some()
        || matches!(action_state.current_action, Some(Action::Idle))
        || action_state.progress.required == 0
    {
        return;
    }

    let fraction = action_state.progress.fraction();
    let pct = (fraction * 100.0) as u32;
    let action_text = format_action_short(&action_state.current_action);

    ui.ch()
        .flex_col()
        .w_full()
        .mb(Val::Px(SPACE_6))
        .add(|ui| {
            // Header row
            ui.ch()
                .flex_row()
                .w_full()
                .justify_between()
                .items_end()
                .mb(Val::Px(SPACE_2))
                .add(|ui| {
                    ui.ch()
                        .label("Mission Progress")
                        .text_size(TEXT_XS)
                        .font_bold()
                        .text_color(TEXT_MUTED);
                    ui.ch()
                        .label(format!("{}%", pct))
                        .text_size(TEXT_SM)
                        .text_color(SUCCESS_400);
                });

            // Progress bar
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

    // Card border and background based on selection
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
                                    .text_color(if is_selected {
                                        TEXT_PRIMARY
                                    } else {
                                        GRAY_300
                                    });

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

/// Compute average character level from skills.
fn compute_average_level(skills: &Skills) -> u32 {
    let total: u32 = skills.iter().map(|(_, xp)| xp_to_level(xp)).sum();
    let count = skills.iter().count() as u32;
    if count == 0 { 1 } else { (total / count).max(1) }
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
        Some(Action::Gather { resource, .. }) => {
            (format!("Gathering {}", resource), SUCCESS_400)
        }
        Some(Action::Research { tech_id }) => (format!("Researching {}", tech_id), INFO_400),
        Some(Action::Build { building }) => (format!("Building {}", building), WARNING_400),
        Some(Action::Craft { recipe_id, .. }) => {
            (format!("Crafting {}", recipe_id), INFO_400)
        }
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

// === TASK 6: Job Queue Panel (below) ===
// === TASK 7: Event Log Sidebar (below) ===
```

Note: This file references `render_job_queue_panel` and `render_event_log_sidebar` which will be added in Tasks 6 and 7. For now, add stub functions at the bottom so it compiles:

```rust
/// Render the job queue panel for the selected character.
fn render_job_queue_panel(
    ui: &mut Imm<CapsUi>,
    _entity: Entity,
    _info: &CharacterInfo,
    _action_state: &ActionState,
    _gather_locations: &[(String, String, String)],
    _picker_mode: JobPickerMode,
) {
    ui.ch()
        .mt(Val::Px(SPACE_5))
        .label("Job Queue (coming soon)")
        .text_color(TEXT_MUTED);
}

/// Render the event log sidebar.
fn render_event_log_sidebar(
    ui: &mut Imm<CapsUi>,
    _event_log: &EventLog,
    _sim_state: &SimulationState,
) {
    ui.ch()
        .flex_col()
        .style(|n: &mut Node| {
            n.width = Val::Px(280.0);
        })
        .h_full()
        .border(BORDER_WIDTH_DEFAULT)
        .border_color(BORDER_DEFAULT)
        .bg(SURFACE_BASE)
        .add(|ui| {
            ui.ch().p(Val::Px(SPACE_4)).add(|ui| {
                ui.ch()
                    .label("EVENT LOG")
                    .text_size(TEXT_SM)
                    .font_bold()
                    .text_color(TEXT_PRIMARY);
            });
        });
}
```

**Step 2: Update content/mod.rs if needed**

The existing `mod.rs` already registers `SquadsView` with `BevyImmediateAttachPlugin`. No changes needed since the component name stays the same.

**Step 3: Build and verify**

Run: `cargo check`
Expected: Compiles with no errors.

**Step 4: Run the game and visually verify**

Run: `bevy run`
Expected: Squads view shows squad cards with header, status badge, agent cards in a grid.

**Step 5: Commit**

```bash
git add src/game/ui/content/squads.rs
git commit -m "feat: rewrite SquadsView with squad card, agent grid, and progress bar"
```

---

### Task 6: Implement Job Queue Panel

**Files:**
- Modify: `src/game/ui/content/squads.rs` (replace `render_job_queue_panel` stub)

**Step 1: Replace the job queue stub**

Replace the stub `render_job_queue_panel` function with:

```rust
/// Render the job queue panel for the selected character.
fn render_job_queue_panel(
    ui: &mut Imm<CapsUi>,
    entity_id: Entity,
    info: &CharacterInfo,
    action_state: &ActionState,
    gather_locations: &[(String, String, String)],
    picker_mode: JobPickerMode,
) {
    // Job queue card
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
                    ui.ch()
                        .flex_row()
                        .column_gap(SPACE_2)
                        .add(|ui| {
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
                ui.ch()
                    .p(Val::Px(SPACE_5))
                    .add(|ui| {
                        ui.ch()
                            .label("No jobs assigned")
                            .text_size(TEXT_SM)
                            .text_color(TEXT_MUTED);
                    });
            } else {
                for (i, job) in action_state.job_queue.iter().enumerate() {
                    let is_current = action_state.current_job_index > 0
                        && (action_state.current_job_index - 1) % action_state.job_queue.len()
                            == i;

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
                                                  mut action_query: Query<&mut ActionState>| {
                                                if let Ok(mut action_state) =
                                                    action_query.get_mut(entity)
                                                {
                                                    let job =
                                                        make_gather_job(&location, &res);
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
        WARNING_400  // amber/yellow
    } else if name_lower.contains("craft") {
        INFO_400  // blue
    } else if name_lower.contains("build") {
        SECONDARY_400  // purple
    } else if name_lower.contains("research") {
        ACCENT_400  // teal
    } else if name_lower.contains("explore") {
        SUCCESS_400  // green
    } else {
        GRAY_400
    }
}
```

**Step 2: Build and verify**

Run: `cargo check`
Expected: Compiles with no errors.

**Step 3: Commit**

```bash
git add src/game/ui/content/squads.rs
git commit -m "feat: implement job queue panel with table layout and action buttons"
```

---

### Task 7: Implement Event Log Sidebar

**Files:**
- Modify: `src/game/ui/content/squads.rs` (replace `render_event_log_sidebar` stub)

**Step 1: Replace the event log stub**

Replace the stub `render_event_log_sidebar` function with:

```rust
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
```

**Step 2: Build and verify**

Run: `cargo check`
Expected: Compiles with no errors.

**Step 3: Run the game and visually verify the full view**

Run: `bevy run`
Expected: Squads view shows the complete layout: squad cards with agent grid on the left, event log sidebar on the right. Events appear in the log as the game runs.

**Step 4: Commit**

```bash
git add src/game/ui/content/squads.rs
git commit -m "feat: implement event log sidebar with timeline display"
```

---

### Task 8: Final Cleanup and Formatting

**Files:**
- All modified files

**Step 1: Format code**

Run: `cargo fmt --all`

**Step 2: Run clippy**

Run: `cargo clippy`
Expected: No new warnings (existing `too_many_arguments` and `type_complexity` allows should cover Bevy patterns).

**Step 3: Build release to verify**

Run: `cargo check`
Expected: Clean compile.

**Step 4: Run the game end-to-end**

Run: `bevy run`
Expected:
- Navigate to Squads view
- See squad cards with name, status badge, operation text
- See agent selection cards with level badges
- Click an agent to select them — card highlights, progress bar appears if active
- See job queue table for selected agent
- Event log sidebar shows entries as game progresses
- Add/clear jobs still works

**Step 5: Commit any formatting changes**

```bash
git add -A
git commit -m "chore: format and lint cleanup for squads view redesign"
```
