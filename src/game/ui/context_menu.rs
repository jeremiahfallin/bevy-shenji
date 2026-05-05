//! Right-click context menu overlay.
//!
//! Architecture:
//! - `ContextMenuState` is the public API. Game code mutates it (sets
//!   `is_open = true`, `position`, `target`, `context_type`, `mode`) to
//!   open the menu; `close_context_menu_on_click` and the menu items
//!   themselves close it again.
//! - `ContextMenuOverlay` is a marker entity created by the gameplay
//!   layout. An `On<Add>` observer attaches an empty container; a
//!   refresh system tears down and rebuilds the menu's children whenever
//!   `ContextMenuState` changes.
//!
//! The previous `ImmUiContextMenuExt` trait + `WithContextMenu` marker
//! component were dropped — they were unreferenced outside this file.
//! Future widgets that want to open the context menu can mutate
//! `ContextMenuState` directly via their own observer system.

use bevy::prelude::*;
use bevy_declarative::prelude::px;

use crate::game::action::{Action, ActionState};
use crate::game::character::CharacterInfo;
use crate::game::data::GameData;
use crate::game::location::{LocationInfo, LocationRegistry, LocationResources};
use crate::game::research::ResearchState;
use crate::screens::Screen;
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(populate_overlay_on_add);
    app.add_systems(
        Update,
        (
            refresh_context_menu,
            close_context_menu_on_click,
        )
            .run_if(in_state(Screen::Gameplay)),
    );
}

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
pub struct ContextMenuOverlay;

// --- Click-handler marker components ----------------------------------------

#[derive(Component)]
struct ActionItemButton {
    target: Entity,
    action: Action,
}

#[derive(Component)]
struct ModeSwitchButton(ContextMenuMode);

#[derive(Component)]
struct ClearActionsItemButton(Entity);

#[derive(Component)]
struct ClearJobsItemButton(Entity);

#[derive(Component)]
struct CloseOnlyButton;

// --- Observer systems for click handlers ------------------------------------

fn action_item_on_click(
    click: On<Pointer<Click>>,
    q: Query<&ActionItemButton>,
    mut action_query: Query<&mut ActionState>,
    mut state: ResMut<ContextMenuState>,
) {
    if let Ok(marker) = q.get(click.entity) {
        if let Ok(mut action_state) = action_query.get_mut(marker.target) {
            action_state.queue_action(marker.action.clone());
        }
        state.is_open = false;
    }
}

fn mode_switch_on_click(
    click: On<Pointer<Click>>,
    q: Query<&ModeSwitchButton>,
    mut state: ResMut<ContextMenuState>,
) {
    if let Ok(marker) = q.get(click.entity) {
        state.mode = marker.0;
    }
}

fn clear_actions_item_on_click(
    click: On<Pointer<Click>>,
    q: Query<&ClearActionsItemButton>,
    mut action_query: Query<&mut ActionState>,
    mut state: ResMut<ContextMenuState>,
) {
    if let Ok(marker) = q.get(click.entity) {
        if let Ok(mut action_state) = action_query.get_mut(marker.0) {
            action_state.clear();
        }
        state.is_open = false;
    }
}

fn clear_jobs_item_on_click(
    click: On<Pointer<Click>>,
    q: Query<&ClearJobsItemButton>,
    mut action_query: Query<&mut ActionState>,
    mut state: ResMut<ContextMenuState>,
) {
    if let Ok(marker) = q.get(click.entity) {
        if let Ok(mut action_state) = action_query.get_mut(marker.0) {
            action_state.clear_jobs();
        }
        state.is_open = false;
    }
}

fn close_only_on_click(_: On<Pointer<Click>>, mut state: ResMut<ContextMenuState>) {
    state.is_open = false;
}

// --- Lifecycle: spawn / refresh ---------------------------------------------

fn populate_overlay_on_add(
    add: On<Add, ContextMenuOverlay>,
    mut commands: Commands,
    state: Res<ContextMenuState>,
    characters: Query<&CharacterInfo>,
    locations: Query<(&LocationInfo, Option<&LocationResources>)>,
    game_data: Res<GameData>,
    research_state: Res<ResearchState>,
) {
    spawn_overlay_children(
        &mut commands,
        add.entity,
        &state,
        &characters,
        &locations,
        &game_data,
        &research_state,
    );
}

#[allow(clippy::too_many_arguments)]
fn refresh_context_menu(
    mut commands: Commands,
    overlay_q: Query<Entity, With<ContextMenuOverlay>>,
    state: Res<ContextMenuState>,
    characters: Query<&CharacterInfo>,
    locations: Query<(&LocationInfo, Option<&LocationResources>)>,
    game_data: Res<GameData>,
    research_state: Res<ResearchState>,
    _location_registry: Res<LocationRegistry>,
) {
    if !state.is_changed() && !game_data.is_changed() && !research_state.is_changed() {
        return;
    }
    for overlay in &overlay_q {
        commands.entity(overlay).despawn_related::<Children>();
        spawn_overlay_children(
            &mut commands,
            overlay,
            &state,
            &characters,
            &locations,
            &game_data,
            &research_state,
        );
    }
}

fn spawn_overlay_children(
    commands: &mut Commands,
    overlay: Entity,
    state: &ContextMenuState,
    characters: &Query<&CharacterInfo>,
    locations: &Query<(&LocationInfo, Option<&LocationResources>)>,
    game_data: &GameData,
    research_state: &ResearchState,
) {
    if !state.is_open {
        return;
    }

    let pos = state.position;
    let context_type = state.context_type;
    let target = state.target;
    let mode = state.mode;

    let header_text = match (context_type, target) {
        (ContextMenuType::Character, Some(entity)) => characters
            .get(entity)
            .map(|info| info.name.clone())
            .unwrap_or_else(|_| "Character".to_string()),
        (ContextMenuType::Unit, _) => "Unit".to_string(),
        (ContextMenuType::InventoryItem, _) => "Item".to_string(),
        _ => "Menu".to_string(),
    };

    let discovered_locations: Vec<(String, String)> = locations
        .iter()
        .filter(|(info, _)| info.discovered)
        .map(|(info, _)| (info.id.clone(), info.name.clone()))
        .collect();

    let gather_locations: Vec<(String, String, String)> = locations
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

    let researchable_techs: Vec<(String, String)> = game_data
        .research
        .values()
        .filter(|def| research_state.can_research(&def.id, game_data))
        .map(|def| (def.id.clone(), def.name.clone()))
        .collect();

    let mut panel = div()
        .col()
        .min_w(Val::Px(180.0))
        .h(Val::Px(0.0)) // overridden by max_height below via Node insert
        .p(px(SPACE_1))
        .bg(GRAY_800)
        .rounded(Val::Px(4.0))
        .insert((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(pos.x),
                top: Val::Px(pos.y),
                min_width: Val::Px(180.0),
                max_height: Val::Px(400.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(SPACE_1)),
                ..default()
            },
            BorderColor::all(GRAY_700),
        ))
        .child(text(header_text).font_size(13.0).color(GRAY_100))
        .child(menu_divider());

    panel = match context_type {
        ContextMenuType::Character => {
            if let Some(entity) = target {
                add_character_menu_items(
                    panel,
                    entity,
                    mode,
                    &discovered_locations,
                    &gather_locations,
                    &researchable_techs,
                )
            } else {
                panel
            }
        }
        ContextMenuType::Unit => panel
            .child(close_only_item("Move"))
            .child(close_only_item("Attack")),
        ContextMenuType::InventoryItem => panel
            .child(close_only_item("Use"))
            .child(close_only_item("Drop")),
        ContextMenuType::None => panel,
    };

    let panel_entity = panel.spawn(commands).id();
    commands.entity(overlay).add_child(panel_entity);
}

// --- Menu builders ----------------------------------------------------------

fn add_character_menu_items(
    mut panel: Div,
    target_entity: Entity,
    mode: ContextMenuMode,
    discovered_locations: &[(String, String)],
    gather_locations: &[(String, String, String)],
    researchable_techs: &[(String, String)],
) -> Div {
    match mode {
        ContextMenuMode::Main => {
            panel = panel.child(action_item("Explore", target_entity, Action::Explore));
            if !discovered_locations.is_empty() {
                panel = panel.child(mode_switch_item("Travel to...", ContextMenuMode::Travel));
            }
            if !gather_locations.is_empty() {
                panel = panel.child(mode_switch_item("Gather at...", ContextMenuMode::Gather));
            }
            if !researchable_techs.is_empty() {
                panel = panel.child(mode_switch_item("Research...", ContextMenuMode::Research));
            }
            panel = panel
                .child(menu_divider())
                .child(clear_actions_item(target_entity))
                .child(clear_jobs_item(target_entity));
        }
        ContextMenuMode::Travel => {
            panel = panel
                .child(mode_switch_item("<- Back", ContextMenuMode::Main))
                .child(menu_divider());
            for (loc_id, loc_name) in discovered_locations {
                panel = panel.child(action_item(
                    loc_name,
                    target_entity,
                    Action::Travel {
                        destination: loc_id.clone(),
                    },
                ));
            }
        }
        ContextMenuMode::Gather => {
            panel = panel
                .child(mode_switch_item("<- Back", ContextMenuMode::Main))
                .child(menu_divider());
            for (loc_id, loc_name, resource) in gather_locations {
                let label_text = format!("{} ({})", loc_name, resource);
                panel = panel.child(action_item(
                    &label_text,
                    target_entity,
                    Action::Gather {
                        location: loc_id.clone(),
                        resource: resource.clone(),
                    },
                ));
            }
        }
        ContextMenuMode::Research => {
            panel = panel
                .child(mode_switch_item("<- Back", ContextMenuMode::Main))
                .child(menu_divider());
            for (tech_id, tech_name) in researchable_techs {
                panel = panel.child(action_item(
                    tech_name,
                    target_entity,
                    Action::Research {
                        tech_id: tech_id.clone(),
                    },
                ));
            }
        }
    }
    panel
}

fn action_item(label: &str, target: Entity, action: Action) -> Div {
    div()
        .w_full()
        .pad_x(px(SPACE_2))
        .py(px(SPACE_1))
        .insert(ActionItemButton { target, action })
        .on_click(action_item_on_click)
        .child(text(label.to_string()).font_size(13.0).color(GRAY_100))
}

fn mode_switch_item(label: &str, target_mode: ContextMenuMode) -> Div {
    div()
        .w_full()
        .pad_x(px(SPACE_2))
        .py(px(SPACE_1))
        .insert(ModeSwitchButton(target_mode))
        .on_click(mode_switch_on_click)
        .child(text(label.to_string()).font_size(13.0).color(GRAY_100))
}

fn clear_actions_item(entity: Entity) -> Div {
    div()
        .w_full()
        .pad_x(px(SPACE_2))
        .py(px(SPACE_1))
        .insert(ClearActionsItemButton(entity))
        .on_click(clear_actions_item_on_click)
        .child(text("Clear Actions").font_size(13.0).color(GRAY_100))
}

fn clear_jobs_item(entity: Entity) -> Div {
    div()
        .w_full()
        .pad_x(px(SPACE_2))
        .py(px(SPACE_1))
        .insert(ClearJobsItemButton(entity))
        .on_click(clear_jobs_item_on_click)
        .child(text("Clear Jobs").font_size(13.0).color(GRAY_100))
}

fn close_only_item(label: &str) -> Div {
    div()
        .w_full()
        .pad_x(px(SPACE_2))
        .py(px(SPACE_1))
        .insert(CloseOnlyButton)
        .on_click(close_only_on_click)
        .child(text(label.to_string()).font_size(13.0).color(GRAY_100))
}

fn menu_divider() -> Div {
    div()
        .w_full()
        .h(Val::Px(1.0))
        .my(px(SPACE_1))
        .bg(GRAY_700)
}

// --- Close-on-left-click anywhere -------------------------------------------

fn close_context_menu_on_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut state: ResMut<ContextMenuState>,
) {
    if state.is_open && mouse.just_pressed(MouseButton::Left) {
        state.is_open = false;
    }
}
