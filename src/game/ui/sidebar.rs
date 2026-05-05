//! Gameplay sidebar — nav rail + sim/base/resource info.
//!
//! Spawned once when the `Sidebar` marker entity is added (the layout
//! creates that entity). Dynamic fields (sim time/days/speed, base zeni/power,
//! resource counts) carry marker components and are kept in sync by per-field
//! systems gated on `Res::is_changed()` — the same pattern Settings uses.
//! Nav-button highlight follows `UiState::active_view` via a similar system.

use bevy::prelude::*;
use bevy_declarative::prelude::px;
use lucide_icons::Icon;

use crate::game::resources::{BaseInventory, BaseState, GameView, UiState};
use crate::game::simulation::SimulationState;
use crate::screens::Screen;
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(populate_sidebar_on_add);
    app.add_systems(
        Update,
        (
            update_nav_active,
            update_sim_text,
            update_base_text,
            update_resource_counts,
        )
            .run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component)]
pub struct Sidebar;

// --- Dynamic-field markers --------------------------------------------------

#[derive(Component)]
struct NavButton(GameView);

#[derive(Component)]
struct SimTimeText;

#[derive(Component)]
struct SimDaysText;

#[derive(Component)]
struct SimSpeedText;

#[derive(Component)]
struct BaseZeniText;

#[derive(Component)]
struct BasePowerText;

#[derive(Component)]
struct ResourceCountText {
    id: &'static str,
}

// --- Initial spawn ----------------------------------------------------------

fn populate_sidebar_on_add(
    add: On<Add, Sidebar>,
    mut commands: Commands,
    ui_state: Res<UiState>,
    sim_state: Res<SimulationState>,
    base_state: Res<BaseState>,
    base_inv: Res<BaseInventory>,
) {
    let sidebar = add.entity;

    let nav_section = div()
        .col()
        .gap_y(px(SPACE_0_5))
        .p(px(SPACE_2))
        .child(nav_button(
            "Dashboard",
            Icon::LayoutDashboard,
            GameView::Dashboard,
            ui_state.active_view,
        ))
        .child(nav_button(
            "Research",
            Icon::Book,
            GameView::Research,
            ui_state.active_view,
        ))
        .child(nav_button(
            "Squads",
            Icon::Group,
            GameView::Squads,
            ui_state.active_view,
        ))
        .child(nav_button(
            "Characters",
            Icon::User,
            GameView::Characters,
            ui_state.active_view,
        ))
        .child(nav_button(
            "Locations",
            Icon::Map,
            GameView::Locations,
            ui_state.active_view,
        ))
        .child(nav_button(
            "Buildings",
            Icon::Hammer,
            GameView::Buildings,
            ui_state.active_view,
        ));

    let sim_section = div()
        .col()
        .gap_y(px(SPACE_0_5))
        .p(px(SPACE_2))
        .w_full()
        .child(heading_3("Simulation"))
        .child(label(format!("Game Time: {}", sim_state.game_time)).insert(SimTimeText))
        .child(label(format!("Days: {}", sim_state.game_days)).insert(SimDaysText))
        .child(label(sim_speed_text(&sim_state)).insert(SimSpeedText));

    let base_section = div()
        .col()
        .gap_y(px(SPACE_0_5))
        .p(px(SPACE_2))
        .w_full()
        .child(heading_3("Base"))
        .child(label(format!("Zeni: {}", base_state.value.zeni)).insert(BaseZeniText))
        .child(
            label(format!(
                "Power: {}/{}",
                base_state.power.generation, base_state.power.consumption
            ))
            .insert(BasePowerText),
        );

    let resources_section = div()
        .col()
        .gap_y(px(SPACE_0_5))
        .p(px(SPACE_2))
        .w_full()
        .child(heading_3("Resources"))
        .child(resource_row("Lumber", "lumber", &base_inv))
        .child(resource_row("Stone", "stone", &base_inv))
        .child(resource_row("Iron Ore", "iron_ore", &base_inv))
        .child(resource_row("Copper Ore", "copper_ore", &base_inv));

    let root = div()
        .col()
        .w_full()
        .h_full()
        .min_w(Val::Px(250.0))
        .flex_shrink(0.0)
        .justify_between()
        .child(nav_section)
        .child(sim_section)
        .child(base_section)
        .child(resources_section);

    let root_entity = root.spawn(&mut commands).id();
    commands.entity(sidebar).add_child(root_entity);
}

fn nav_button(label_text: &str, ic: Icon, view: GameView, active: GameView) -> Div {
    let mut btn = div()
        .flex()
        .row()
        .items_center()
        .gap_x(px(SPACE_2))
        .pad_x(px(SPACE_3))
        .py(px(SPACE_2))
        .rounded(px(SPACE_1))
        .insert(NavButton(view))
        .on_click(nav_button_on_click)
        .child(icon(ic).color(Color::WHITE))
        .child(text(label_text).color(Color::WHITE));
    if view == active {
        btn = btn.bg(GRAY_700);
    }
    btn
}

fn resource_row(name: &str, id: &'static str, base_inv: &BaseInventory) -> Div {
    div()
        .flex()
        .row()
        .justify_between()
        .w_full()
        .child(label(name).color(Color::srgb(0.8, 0.8, 0.8)))
        .child(
            text(format!("{}", base_inv.count(id)))
                .color(Color::WHITE)
                .insert(ResourceCountText { id }),
        )
}

fn sim_speed_text(sim_state: &SimulationState) -> String {
    if sim_state.is_paused() {
        "Paused".to_string()
    } else {
        format!("Speed: {}x", sim_state.speed)
    }
}

// --- Click handler ----------------------------------------------------------

fn nav_button_on_click(
    click: On<Pointer<Click>>,
    q: Query<&NavButton>,
    mut ui_state: ResMut<UiState>,
) {
    if let Ok(nav) = q.get(click.entity) {
        ui_state.active_view = nav.0;
    }
}

// --- Reactive update systems ------------------------------------------------

fn update_nav_active(
    ui_state: Res<UiState>,
    mut q: Query<(&NavButton, &mut BackgroundColor)>,
) {
    if !ui_state.is_changed() {
        return;
    }
    for (nav, mut bg) in &mut q {
        if nav.0 == ui_state.active_view {
            *bg = BackgroundColor(GRAY_700);
        } else {
            *bg = BackgroundColor(Color::NONE);
        }
    }
}

fn update_sim_text(
    sim_state: Res<SimulationState>,
    mut q_time: Query<&mut Text, (With<SimTimeText>, Without<SimDaysText>, Without<SimSpeedText>)>,
    mut q_days: Query<&mut Text, (With<SimDaysText>, Without<SimTimeText>, Without<SimSpeedText>)>,
    mut q_speed: Query<&mut Text, (With<SimSpeedText>, Without<SimTimeText>, Without<SimDaysText>)>,
) {
    if !sim_state.is_changed() {
        return;
    }
    for mut t in &mut q_time {
        t.0 = format!("Game Time: {}", sim_state.game_time);
    }
    for mut t in &mut q_days {
        t.0 = format!("Days: {}", sim_state.game_days);
    }
    for mut t in &mut q_speed {
        t.0 = sim_speed_text(&sim_state);
    }
}

fn update_base_text(
    base_state: Res<BaseState>,
    mut q_zeni: Query<&mut Text, (With<BaseZeniText>, Without<BasePowerText>)>,
    mut q_power: Query<&mut Text, (With<BasePowerText>, Without<BaseZeniText>)>,
) {
    if !base_state.is_changed() {
        return;
    }
    for mut t in &mut q_zeni {
        t.0 = format!("Zeni: {}", base_state.value.zeni);
    }
    for mut t in &mut q_power {
        t.0 = format!(
            "Power: {}/{}",
            base_state.power.generation, base_state.power.consumption
        );
    }
}

fn update_resource_counts(
    base_inv: Res<BaseInventory>,
    mut q: Query<(&ResourceCountText, &mut Text)>,
) {
    if !base_inv.is_changed() {
        return;
    }
    for (rc, mut t) in &mut q {
        t.0 = format!("{}", base_inv.count(rc.id));
    }
}
