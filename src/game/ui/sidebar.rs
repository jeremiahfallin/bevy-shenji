use bevy::prelude::*;

use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};
use lucide_icons::Icon;

use crate::game::resources::{GameState, GameView, UiState};
use crate::theme::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, Sidebar>::new());
}

#[derive(Component)]
pub struct Sidebar;

impl ImmediateAttach<CapsUi> for Sidebar {
    // Inject the game state resource ('static lifetime is required here)
    type Params = (Res<'static, GameState>, ResMut<'static, UiState>);

    fn construct(
        ui: &mut Imm<CapsUi>,
        (game_state, ui_state): &mut (Res<GameState>, ResMut<UiState>),
    ) {
        ui.ch()
            .flex_col()
            .w_full()
            .min_w(Val::Px(250.0))
            .flex_shrink_0()
            .justify_between()
            .add(|ui| {
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
                });

                ui.ch()
                    .flex_col()
                    .row_gap(2.0)
                    .p(Val::Px(8.0))
                    .w_full()
                    .add(|ui| {
                        ui.ch().header("Resources");

                        let money_text = format!("Current Level: ${:.2}", game_state.current_level);
                        ui.ch().label(money_text);

                        let wood_text = format!("Game Time: {:.0}", game_state.game_time);
                        ui.ch().label(wood_text);

                        let rate_text = format!("({:.1}/sec)", game_state.is_paused);
                        ui.ch().label(rate_text);
                    });
            });
    }
}
