use crate::game::action::{Action, ActionState};
use crate::game::character::{Equipment, Health, Inventory, Skills};
use crate::game::resources::SquadState;
use crate::theme::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_immediate::{Imm, attach::ImmediateAttach, ui::CapsUi};

// FIX: Derive 'Resource' so it can be used in Res<...>
#[derive(Resource, Default)]
pub struct InspectorState {
    pub selected_character_id: Option<String>,
    pub active_tab: InspectorTab,
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum InspectorTab {
    #[default]
    Health,
    Equipment,
    Skills,
    Inventory,
}

// The UI Widget Component
#[derive(Component)]
pub struct CharacterInspector;

// Custom SystemParam to handle multiple resources with correct lifetimes
#[derive(SystemParam)]
pub struct InspectorParams<'w> {
    pub squad_state: Res<'w, SquadState>,
    pub inspector_state: Res<'w, InspectorState>,
}

impl ImmediateAttach<CapsUi> for CharacterInspector {
    // We inject the Global Game State (SquadState), Local UI State (InspectorState), and Character Components
    type Params = (
        Res<'static, SquadState>,
        Res<'static, InspectorState>,
        Query<
            'static,
            'static,
            (
                &'static Health,
                &'static Skills,
                &'static Equipment,
                &'static Inventory,
                &'static ActionState,
            ),
        >,
    );

    fn construct(
        ui: &mut Imm<CapsUi>,
        params: &mut (
            Res<SquadState>,
            Res<InspectorState>,
            Query<(&Health, &Skills, &Equipment, &Inventory, &ActionState)>,
        ),
    ) {
        let (squad_state, inspector_state, char_query) = (&params.0, &params.1, &params.2);

        // 1. Validation: Do we have a selected character?
        let Some(char_entity) = &inspector_state.selected_character_id else {
            ui.ch().label("Select a character to inspect").style(|n| {
                n.align_self = AlignSelf::Center;
                n.margin = UiRect::all(Val::Auto);
            });
            return;
        };

        // 2. Fetch Data: Does that character exist?
        let Some(entity) = squad_state.characters.get(char_entity) else {
            ui.ch().label("Character not found");
            return;
        };

        let Ok((health, skills, equipment, inventory, action_state)) = char_query.get(*entity)
        else {
            ui.ch().label("Character missing data");
            return;
        };

        let entity_id = *entity;

        // 3. Render Inspector
        ui.ch()
            .flex_col()
            .flex_grow()
            .apply(style_panel_central)
            .p(Val::Px(15.0))
            .add(|ui| {
                // --- Action Status Header ---
                render_action_status(ui, entity_id, action_state);

                // Divider
                ui.ch().on_spawn_insert(|| {
                    (
                        Node {
                            height: Val::Px(1.0),
                            width: Val::Percent(100.0),
                            margin: UiRect::axes(Val::Px(0.0), Val::Px(6.0)),
                            ..default()
                        },
                        BackgroundColor(GRAY_700),
                    )
                });

                // Tab bar
                let active = inspector_state.active_tab;
                ui.ch()
                    .flex_row()
                    .w_full()
                    .mb(Val::Px(10.0))
                    .column_gap(4.0)
                    .add(|ui| {
                        tab_button(ui, "Health", InspectorTab::Health, active);
                        tab_button(ui, "Equipment", InspectorTab::Equipment, active);
                        tab_button(ui, "Skills", InspectorTab::Skills, active);
                        tab_button(ui, "Inventory", InspectorTab::Inventory, active);
                    });

                // Tab content
                match inspector_state.active_tab {
                InspectorTab::Health => {
                    // FIX: Iterate over all limbs
                    for (part, hp) in health.iter() {
                        ui.ch()
                            .flex_row()
                            .justify_between()
                            .w_full()
                            .mb(Val::Px(5.0))
                            .add(|ui| {
                                ui.ch().label(part).text_color(Color::srgb(0.8, 0.8, 0.8));
                                // Color scaling
                                let color = if hp > 80 {
                                    Color::srgb(0.0, 1.0, 0.0)
                                } else if hp > 40 {
                                    Color::srgb(1.0, 1.0, 0.0)
                                } else {
                                    Color::srgb(1.0, 0.0, 0.0)
                                };
                                ui.ch().label(format!("{}", hp)).text_color(color);
                            });
                    }
                    ui.ch().label("Status").mt(Val::Px(10.0)).mb(Val::Px(5.0));
                    ui.ch().label(format!("Hunger: {}", health.hunger));
                }
                InspectorTab::Equipment => {
                    // FIX: Use the display_equip_slot helper
                    let eq = equipment;
                    display_equip_slot(ui, "Head", &eq.head);
                    display_equip_slot(ui, "Chest", &eq.chest);
                    display_equip_slot(ui, "Legs", &eq.legs);
                    display_equip_slot(ui, "Feet", &eq.feet);
                    display_equip_slot(ui, "Hands", &eq.hands);
                    display_equip_slot(ui, "Main Hand", &eq.main_hand);
                }
                InspectorTab::Skills => {
                    ui.ch().scrollarea(
                        |n| {
                            n.flex_direction = FlexDirection::Column;
                        },
                        |ui| {
                            for (skill, xp) in skills.iter() {
                                ui.ch()
                                    .flex_row()
                                    .justify_between()
                                    .w_full()
                                    .mb(Val::Px(2.0))
                                    .add(|ui| {
                                        ui.ch().label(skill).text_color(Color::srgb(0.8, 0.8, 0.8));
                                        ui.ch()
                                            .label(format!("{}", xp_to_level(xp)))
                                            .text_color(Color::WHITE);
                                    });
                            }
                        },
                    );
                }
                InspectorTab::Inventory => {
                    if inventory.items.is_empty() {
                        ui.ch()
                            .label("Empty")
                            .text_color(Color::srgb(0.5, 0.5, 0.5));
                    } else {
                        for (item, count) in &inventory.items {
                            ui.ch().flex_row().justify_between().w_full().add(|ui| {
                                ui.ch().label(format!("{}: {}", item, count));
                                ui.ch().button().add(|ui| {
                                    ui.ch().label("Drop");
                                });
                            });
                        }
                    }
                }
            }});
    }
}

/// Render the action status header showing current action, progress, queue counts.
fn render_action_status(ui: &mut Imm<CapsUi>, entity_id: Entity, action_state: &ActionState) {
    ui.ch().flex_col().w_full().mb(Val::Px(4.0)).add(|ui| {
        // Current action line
        let action_text = match &action_state.current_action {
            Some(action) => format_action(action),
            None => "Idle".to_string(),
        };

        ui.ch()
            .flex_row()
            .w_full()
            .mb(Val::Px(2.0))
            .add(|ui| {
                ui.ch()
                    .label("Action: ")
                    .text_size(12.0)
                    .text_color(Color::srgb(0.6, 0.6, 0.6));
                ui.ch()
                    .label(&action_text)
                    .text_size(12.0)
                    .text_color(Color::WHITE);
            });

        // Progress bar (if there's a current non-idle action with progress)
        if action_state.current_action.is_some()
            && !matches!(action_state.current_action, Some(Action::Idle))
            && action_state.progress.required > 0
        {
            let fraction = action_state.progress.fraction();
            let pct = (fraction * 100.0) as u32;

            ui.ch()
                .flex_row()
                .w_full()
                .mb(Val::Px(2.0))
                .add(|ui| {
                    // Progress bar background
                    ui.ch()
                        .style(|n: &mut Node| {
                            n.width = Val::Percent(70.0);
                            n.height = Val::Px(8.0);
                        })
                        .bg(GRAY_700)
                        .rounded(2.0)
                        .add(move |ui| {
                            // Progress bar fill
                            ui.ch()
                                .style(move |n: &mut Node| {
                                    n.width = Val::Percent(fraction * 100.0);
                                    n.height = Val::Percent(100.0);
                                })
                                .bg(PRIMARY_500)
                                .rounded(2.0);
                        });

                    ui.ch()
                        .label(format!(" {}%", pct))
                        .text_size(11.0)
                        .text_color(Color::srgb(0.7, 0.7, 0.7));
                });
        }

        // Queue counts
        let queue_count = action_state.action_queue.len();
        let job_count = action_state.job_queue.len();
        ui.ch()
            .flex_row()
            .w_full()
            .column_gap(12.0)
            .add(|ui| {
                ui.ch()
                    .label(format!("Queued: {}", queue_count))
                    .text_size(11.0)
                    .text_color(Color::srgb(0.6, 0.6, 0.6));
                ui.ch()
                    .label(format!("Jobs: {}", job_count))
                    .text_size(11.0)
                    .text_color(Color::srgb(0.6, 0.6, 0.6));
            });

        // Action buttons row
        ui.ch()
            .flex_row()
            .w_full()
            .mt(Val::Px(4.0))
            .column_gap(4.0)
            .add(|ui| {
                // Clear Actions button
                {
                    let entity = entity_id;
                    ui.ch()
                        .button()
                        .style(|n: &mut Node| {
                            n.padding = UiRect::axes(Val::Px(6.0), Val::Px(2.0));
                        })
                        .bg(GRAY_700)
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
                                .text_size(11.0)
                                .text_color(Color::srgb(0.8, 0.8, 0.8));
                        });
                }

                // Clear Jobs button
                {
                    let entity = entity_id;
                    ui.ch()
                        .button()
                        .style(|n: &mut Node| {
                            n.padding = UiRect::axes(Val::Px(6.0), Val::Px(2.0));
                        })
                        .bg(GRAY_700)
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
                                .text_size(11.0)
                                .text_color(Color::srgb(0.8, 0.8, 0.8));
                        });
                }
            });
    });
}

/// Format an action for display.
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

// Helper: Tab Button
fn tab_button(ui: &mut Imm<CapsUi>, text: &str, tab: InspectorTab, active: InspectorTab) {
    let is_active = tab == active;
    ui.ch()
        .button()
        .on_click_once(
            move |_trigger: On<Pointer<Click>>, mut state: ResMut<InspectorState>| {
                state.active_tab = tab;
            },
        )
        .style(move |n| {
            // Underline effect for active tab
            n.border = UiRect::bottom(Val::Px(if is_active { 2.0 } else { 0.0 }));
        })
        .bg(Color::NONE) // Transparent background like a tab
        .add(|ui| {
            ui.ch().label(text).text_color(if is_active {
                Color::WHITE
            } else {
                Color::srgb(0.6, 0.6, 0.6)
            });
        });
}

// Helper: Math
fn xp_to_level(xp: u32) -> u32 {
    let xp = xp as f64;
    ((xp * 4.0 / 5.0).cbrt().floor() as u32 + 1).min(100)
}

fn display_equip_slot(ui: &mut Imm<CapsUi>, slot_name: &str, item: &Option<String>) {
    ui.ch()
        .flex_row()
        .justify_between()
        .w_full()
        .mb(Val::Px(5.0))
        .add(|ui| {
            ui.ch()
                .label(slot_name)
                .text_color(Color::srgb(0.7, 0.7, 0.7));
            match item {
                Some(name) => {
                    ui.ch().label(name).text_color(Color::WHITE);
                }
                None => {
                    ui.ch()
                        .label("Empty")
                        .text_color(Color::srgb(0.3, 0.3, 0.3));
                }
            }
        });
}
