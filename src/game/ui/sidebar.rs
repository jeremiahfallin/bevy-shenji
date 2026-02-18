use bevy::prelude::*;

use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};
use lucide_icons::Icon;

use crate::game::resources::{BaseInventory, BaseState, GameView, UiState};
use crate::game::simulation::SimulationState;
use crate::theme::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, Sidebar>::new());
}

#[derive(Component)]
pub struct Sidebar;

impl ImmediateAttach<CapsUi> for Sidebar {
    type Params = (
        ResMut<'static, UiState>,
        Res<'static, SimulationState>,
        Res<'static, BaseState>,
        Res<'static, BaseInventory>,
    );

    fn construct(
        ui: &mut Imm<CapsUi>,
        (ui_state, sim_state, base_state, base_inv): &mut (
            ResMut<UiState>,
            Res<SimulationState>,
            Res<BaseState>,
            Res<BaseInventory>,
        ),
    ) {
        ui.ch()
            .flex_col()
            .w_full()
            .min_w(Val::Px(250.0))
            .flex_shrink_0()
            .justify_between()
            .add(|ui| {
                // Navigation buttons
                ui.ch().flex_col().row_gap(2.0).p(Val::Px(8.0)).add(|ui| {
                    let mut nav_btn = |label: &str, icon: Icon, view: GameView| {
                        let is_active = ui_state.active_view == view;
                        let target_view = view;

                        ui.ch()
                            .icon_button()
                            .with_icon(icon)
                            .with_label(label)
                            .apply(|e| if is_active { e.bg(GRAY_700) } else { e })
                            .on_click_once(
                                move |_: On<Pointer<Click>>, mut state: ResMut<UiState>| {
                                    state.active_view = target_view;
                                },
                            );
                    };

                    nav_btn("Dashboard", Icon::LayoutDashboard, GameView::Dashboard);
                    nav_btn("Research", Icon::Book, GameView::Research);
                    nav_btn("Squads", Icon::Group, GameView::Squads);
                    nav_btn("Characters", Icon::User, GameView::Characters);
                    nav_btn("Locations", Icon::Map, GameView::Locations);
                });

                // Simulation info
                ui.ch()
                    .flex_col()
                    .row_gap(2.0)
                    .p(Val::Px(8.0))
                    .w_full()
                    .add(|ui| {
                        ui.ch().header("Simulation");

                        let time_text = format!("Game Time: {}", sim_state.game_time);
                        ui.ch().label(time_text);

                        let days_text = format!("Days: {}", sim_state.game_days);
                        ui.ch().label(days_text);

                        let speed_text = if sim_state.is_paused() {
                            "Paused".to_string()
                        } else {
                            format!("Speed: {}x", sim_state.speed)
                        };
                        ui.ch().label(speed_text);
                    });

                // Currency and power
                ui.ch()
                    .flex_col()
                    .row_gap(2.0)
                    .p(Val::Px(8.0))
                    .w_full()
                    .add(|ui| {
                        ui.ch().header("Base");

                        ui.ch().label(format!("Zeni: {}", base_state.value.zeni));

                        ui.ch().label(format!(
                            "Power: {}/{}",
                            base_state.power.generation, base_state.power.consumption
                        ));
                    });

                // Key resources from inventory
                ui.ch()
                    .flex_col()
                    .row_gap(2.0)
                    .p(Val::Px(8.0))
                    .w_full()
                    .add(|ui| {
                        ui.ch().header("Resources");

                        let resource_row =
                            |ui: &mut Imm<CapsUi>, label: &str, count: u32| {
                                ui.ch()
                                    .flex_row()
                                    .justify_between()
                                    .w_full()
                                    .add(|ui| {
                                        ui.ch()
                                            .label(label)
                                            .text_color(Color::srgb(0.8, 0.8, 0.8));
                                        ui.ch()
                                            .label(format!("{}", count))
                                            .text_color(Color::WHITE);
                                    });
                            };

                        resource_row(ui, "Lumber", base_inv.count("lumber"));
                        resource_row(ui, "Stone", base_inv.count("stone"));
                        resource_row(ui, "Iron Ore", base_inv.count("iron_ore"));
                        resource_row(ui, "Copper Ore", base_inv.count("copper_ore"));
                    });
            });
    }
}
