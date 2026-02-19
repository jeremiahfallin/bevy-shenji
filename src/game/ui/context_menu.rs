use crate::game::action::{Action, ActionState};
use crate::game::character::CharacterInfo;
use crate::game::data::GameData;
use crate::game::location::{LocationInfo, LocationRegistry, LocationResources};
use crate::game::research::ResearchState;
use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::{
    CapSet, Imm, ImmEntity, ImplCap,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, ContextMenuOverlay>::new());
    app.add_systems(Update, close_context_menu_on_click);
}

/// Which sub-menu the context menu is currently showing.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextMenuMode {
    #[default]
    Main,
    Travel,
    Gather,
    Research,
}

#[derive(Resource, Default)]
pub struct ContextMenuState {
    pub is_open: bool,
    pub position: Vec2,
    pub target: Option<Entity>,
    pub context_type: ContextMenuType,
    pub mode: ContextMenuMode,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum ContextMenuType {
    #[default]
    None,
    InventoryItem,
    Unit,
    Character,
}

#[derive(Component)]
pub struct WithContextMenu;

pub trait ImmUiContextMenuExt {
    fn context_menu(self, context_type: ContextMenuType, target: Entity) -> Self;
}

impl<Cap> ImmUiContextMenuExt for ImmEntity<'_, '_, '_, Cap>
where
    Cap: CapSet + ImplCap<CapabilityObserver>,
{
    fn context_menu(mut self, context_type: ContextMenuType, target: Entity) -> Self {
        let has_menu = self
            .cap_get_component::<WithContextMenu>()
            .ok()
            .flatten()
            .is_some();

        if !has_menu {
            self.entity_commands().insert(WithContextMenu).observe(
                move |trigger: On<Pointer<Click>>, mut state: ResMut<ContextMenuState>| {
                    if trigger.event().button == PointerButton::Secondary {
                        state.is_open = true;
                        state.position = trigger.event().pointer_location.position;
                        state.context_type = context_type;
                        state.target = Some(target);
                        state.mode = ContextMenuMode::Main;
                    }
                },
            );
        }
        self
    }
}

// --- Context Menu Overlay Rendering ---

#[derive(Component)]
pub struct ContextMenuOverlay;

impl ImmediateAttach<CapsUi> for ContextMenuOverlay {
    type Params = (
        ResMut<'static, ContextMenuState>,
        Query<'static, 'static, &'static CharacterInfo>,
        Query<'static, 'static, (&'static LocationInfo, Option<&'static LocationResources>)>,
        Res<'static, GameData>,
        Res<'static, ResearchState>,
        Res<'static, LocationRegistry>,
    );

    fn construct(
        ui: &mut Imm<CapsUi>,
        params: &mut (
            ResMut<ContextMenuState>,
            Query<&CharacterInfo>,
            Query<(&LocationInfo, Option<&LocationResources>)>,
            Res<GameData>,
            Res<ResearchState>,
            Res<LocationRegistry>,
        ),
    ) {
        let (menu_state, characters, locations, game_data, research_state, _location_registry) = (
            &*params.0, &params.1, &params.2, &params.3, &params.4, &params.5,
        );

        if !menu_state.is_open {
            return;
        }

        let pos = menu_state.position;
        let context_type = menu_state.context_type;
        let target = menu_state.target;
        let mode = menu_state.mode;

        // Get target name for header
        let header_text = match (context_type, target) {
            (ContextMenuType::Character, Some(entity)) => {
                if let Ok(info) = characters.get(entity) {
                    info.name.clone()
                } else {
                    "Character".to_string()
                }
            }
            (ContextMenuType::Unit, _) => "Unit".to_string(),
            (ContextMenuType::InventoryItem, _) => "Item".to_string(),
            _ => "Menu".to_string(),
        };

        // Collect data needed for sub-menus before entering the closure
        // Discovered locations for Travel
        let discovered_locations: Vec<(String, String)> = locations
            .iter()
            .filter(|(info, _)| info.discovered)
            .map(|(info, _)| (info.id.clone(), info.name.clone()))
            .collect();

        // Resource locations for Gather (discovered locations that have resources)
        let gather_locations: Vec<(String, String, String)> = locations
            .iter()
            .filter(|(info, res)| info.discovered && res.is_some())
            .filter_map(|(info, res)| {
                let res = res?;
                if res.resource_type.is_empty() || res.current_amount == 0 {
                    return None;
                }
                Some((
                    info.id.clone(),
                    info.name.clone(),
                    res.resource_type.clone(),
                ))
            })
            .collect();

        // Researchable techs
        let researchable_techs: Vec<(String, String)> = game_data
            .research
            .values()
            .filter(|def| research_state.can_research(&def.id, game_data))
            .map(|def| (def.id.clone(), def.name.clone()))
            .collect();

        // Render the menu panel at the cursor position
        ui.ch_id("context_menu")
            .style(move |n: &mut Node| {
                n.position_type = PositionType::Absolute;
                n.left = Val::Px(pos.x);
                n.top = Val::Px(pos.y);
                n.min_width = Val::Px(180.0);
                n.max_height = Val::Px(400.0);
                n.flex_direction = FlexDirection::Column;
                n.padding = UiRect::all(Val::Px(4.0));
            })
            .bg(GRAY_800)
            .border(1.0)
            .border_color(GRAY_700)
            .rounded(4.0)
            .add(|ui| {
                // Header
                ui.ch()
                    .label(&header_text)
                    .text_size(13.0)
                    .text_color(GRAY_100);

                // Divider
                menu_divider(ui);

                // Menu items based on context type and mode
                match context_type {
                    ContextMenuType::Character => {
                        if let Some(entity) = target {
                            render_character_menu(
                                ui,
                                entity,
                                &mode,
                                &discovered_locations,
                                &gather_locations,
                                &researchable_techs,
                            );
                        }
                    }
                    ContextMenuType::Unit => {
                        menu_item_close(ui, "Move");
                        menu_item_close(ui, "Attack");
                    }
                    ContextMenuType::InventoryItem => {
                        menu_item_close(ui, "Use");
                        menu_item_close(ui, "Drop");
                    }
                    ContextMenuType::None => {}
                }
            });
    }
}

fn render_character_menu(
    ui: &mut Imm<CapsUi>,
    target_entity: Entity,
    mode: &ContextMenuMode,
    discovered_locations: &[(String, String)],
    gather_locations: &[(String, String, String)],
    researchable_techs: &[(String, String)],
) {
    match mode {
        ContextMenuMode::Main => {
            // Explore action
            menu_item_action(ui, "Explore", target_entity, Action::Explore);

            // Travel to... (opens sub-menu)
            if !discovered_locations.is_empty() {
                menu_item_mode(ui, "Travel to...", ContextMenuMode::Travel);
            }

            // Gather at... (opens sub-menu)
            if !gather_locations.is_empty() {
                menu_item_mode(ui, "Gather at...", ContextMenuMode::Gather);
            }

            // Research... (opens sub-menu)
            if !researchable_techs.is_empty() {
                menu_item_mode(ui, "Research...", ContextMenuMode::Research);
            }

            // Divider before destructive actions
            menu_divider(ui);

            // Clear Actions
            {
                let entity = target_entity;
                ui.ch()
                    .button()
                    .w_full()
                    .style(|n: &mut Node| {
                        n.padding = UiRect::axes(Val::Px(8.0), Val::Px(4.0));
                    })
                    .on_click_once(
                        move |_: On<Pointer<Click>>,
                              mut state: ResMut<ContextMenuState>,
                              mut action_query: Query<&mut ActionState>| {
                            if let Ok(mut action_state) = action_query.get_mut(entity) {
                                action_state.clear();
                            }
                            state.is_open = false;
                        },
                    )
                    .add(|ui| {
                        ui.ch()
                            .label("Clear Actions")
                            .text_size(13.0)
                            .text_color(GRAY_100);
                    });
            }

            // Clear Jobs
            {
                let entity = target_entity;
                ui.ch()
                    .button()
                    .w_full()
                    .style(|n: &mut Node| {
                        n.padding = UiRect::axes(Val::Px(8.0), Val::Px(4.0));
                    })
                    .on_click_once(
                        move |_: On<Pointer<Click>>,
                              mut state: ResMut<ContextMenuState>,
                              mut action_query: Query<&mut ActionState>| {
                            if let Ok(mut action_state) = action_query.get_mut(entity) {
                                action_state.clear_jobs();
                            }
                            state.is_open = false;
                        },
                    )
                    .add(|ui| {
                        ui.ch()
                            .label("Clear Jobs")
                            .text_size(13.0)
                            .text_color(GRAY_100);
                    });
            }
        }

        ContextMenuMode::Travel => {
            // Back button
            menu_item_mode(ui, "<- Back", ContextMenuMode::Main);
            menu_divider(ui);

            // List all discovered locations
            for (loc_id, loc_name) in discovered_locations {
                let entity = target_entity;
                let destination = loc_id.clone();
                let label = loc_name.clone();
                ui.ch()
                    .button()
                    .w_full()
                    .style(|n: &mut Node| {
                        n.padding = UiRect::axes(Val::Px(8.0), Val::Px(4.0));
                    })
                    .on_click_once(
                        move |_: On<Pointer<Click>>,
                              mut state: ResMut<ContextMenuState>,
                              mut action_query: Query<&mut ActionState>| {
                            if let Ok(mut action_state) = action_query.get_mut(entity) {
                                action_state.queue_action(Action::Travel {
                                    destination: destination.clone(),
                                });
                            }
                            state.is_open = false;
                        },
                    )
                    .add(|ui| {
                        ui.ch().label(&label).text_size(13.0).text_color(GRAY_100);
                    });
            }
        }

        ContextMenuMode::Gather => {
            // Back button
            menu_item_mode(ui, "<- Back", ContextMenuMode::Main);
            menu_divider(ui);

            // List all gatherable resource locations
            for (loc_id, loc_name, resource) in gather_locations {
                let entity = target_entity;
                let location = loc_id.clone();
                let res = resource.clone();
                let label = format!("{} ({})", loc_name, resource);
                ui.ch()
                    .button()
                    .w_full()
                    .style(|n: &mut Node| {
                        n.padding = UiRect::axes(Val::Px(8.0), Val::Px(4.0));
                    })
                    .on_click_once(
                        move |_: On<Pointer<Click>>,
                              mut state: ResMut<ContextMenuState>,
                              mut action_query: Query<&mut ActionState>| {
                            if let Ok(mut action_state) = action_query.get_mut(entity) {
                                action_state.queue_action(Action::Gather {
                                    location: location.clone(),
                                    resource: res.clone(),
                                });
                            }
                            state.is_open = false;
                        },
                    )
                    .add(|ui| {
                        ui.ch().label(&label).text_size(13.0).text_color(GRAY_100);
                    });
            }
        }

        ContextMenuMode::Research => {
            // Back button
            menu_item_mode(ui, "<- Back", ContextMenuMode::Main);
            menu_divider(ui);

            // List all researchable techs
            for (tech_id, tech_name) in researchable_techs {
                let entity = target_entity;
                let tid = tech_id.clone();
                let label = tech_name.clone();
                ui.ch()
                    .button()
                    .w_full()
                    .style(|n: &mut Node| {
                        n.padding = UiRect::axes(Val::Px(8.0), Val::Px(4.0));
                    })
                    .on_click_once(
                        move |_: On<Pointer<Click>>,
                              mut state: ResMut<ContextMenuState>,
                              mut action_query: Query<&mut ActionState>| {
                            if let Ok(mut action_state) = action_query.get_mut(entity) {
                                action_state.queue_action(Action::Research {
                                    tech_id: tid.clone(),
                                });
                            }
                            state.is_open = false;
                        },
                    )
                    .add(|ui| {
                        ui.ch().label(&label).text_size(13.0).text_color(GRAY_100);
                    });
            }
        }
    }
}

/// Menu item that queues an action on the target entity and closes the menu.
fn menu_item_action(ui: &mut Imm<CapsUi>, label: &str, target_entity: Entity, action: Action) {
    let label_owned = label.to_string();
    ui.ch()
        .button()
        .w_full()
        .style(|n: &mut Node| {
            n.padding = UiRect::axes(Val::Px(8.0), Val::Px(4.0));
        })
        .on_click_once(
            move |_: On<Pointer<Click>>,
                  mut state: ResMut<ContextMenuState>,
                  mut action_query: Query<&mut ActionState>| {
                if let Ok(mut action_state) = action_query.get_mut(target_entity) {
                    action_state.queue_action(action.clone());
                }
                state.is_open = false;
            },
        )
        .add(|ui| {
            ui.ch()
                .label(&label_owned)
                .text_size(13.0)
                .text_color(GRAY_100);
        });
}

/// Menu item that switches the context menu mode (for sub-menus).
fn menu_item_mode(ui: &mut Imm<CapsUi>, label: &str, target_mode: ContextMenuMode) {
    let label_owned = label.to_string();
    ui.ch()
        .button()
        .w_full()
        .style(|n: &mut Node| {
            n.padding = UiRect::axes(Val::Px(8.0), Val::Px(4.0));
        })
        .on_click_once(
            move |_: On<Pointer<Click>>, mut state: ResMut<ContextMenuState>| {
                state.mode = target_mode;
            },
        )
        .add(|ui| {
            ui.ch()
                .label(&label_owned)
                .text_size(13.0)
                .text_color(GRAY_100);
        });
}

/// A simple close-only menu item (for non-character menus).
fn menu_item_close(ui: &mut Imm<CapsUi>, label: &str) {
    let label_owned = label.to_string();
    ui.ch()
        .button()
        .w_full()
        .style(|n: &mut Node| {
            n.padding = UiRect::axes(Val::Px(8.0), Val::Px(4.0));
        })
        .on_click_once(
            |_: On<Pointer<Click>>, mut state: ResMut<ContextMenuState>| {
                state.is_open = false;
            },
        )
        .add(|ui| {
            ui.ch()
                .label(&label_owned)
                .text_size(13.0)
                .text_color(GRAY_100);
        });
}

/// Renders a horizontal divider line.
fn menu_divider(ui: &mut Imm<CapsUi>) {
    ui.ch().on_spawn_insert(|| {
        (
            Node {
                height: Val::Px(1.0),
                width: Val::Percent(100.0),
                margin: UiRect::axes(Val::Px(0.0), Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(GRAY_700),
        )
    });
}

/// Close the context menu when the user left-clicks anywhere else.
fn close_context_menu_on_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut state: ResMut<ContextMenuState>,
) {
    if state.is_open && mouse.just_pressed(MouseButton::Left) {
        state.is_open = false;
    }
}
