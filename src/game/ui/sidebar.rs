use bevy::prelude::*;

use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};
use lucide_icons::Icon;

// Import your widget helpers
use crate::game::resources::GameState;
use crate::theme::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, Sidebar>::new());
}

#[derive(Component)]
pub struct Sidebar;

impl ImmediateAttach<CapsUi> for Sidebar {
    // Inject the game state resource ('static lifetime is required here)
    type Params = Res<'static, GameState>;

    fn construct(ui: &mut Imm<CapsUi>, state: &mut Res<GameState>) {
        ui.ch().flex_col().w_full().justify_between().add(|ui| {
            ui.ch().flex_col().row_gap(2.0).p(Val::Px(8.0)).add(|ui| {
                ui.ch()
                    .icon_button()
                    .with_icon(Icon::LayoutDashboard)
                    .with_label("Dashboard");
                ui.ch()
                    .icon_button()
                    .with_icon(Icon::BookOpen)
                    .with_label("Research");
                ui.ch()
                    .icon_button()
                    .with_icon(Icon::Users)
                    .with_label("Squads");
                ui.ch()
                    .icon_button()
                    .with_icon(Icon::User)
                    .with_label("Characters");
            });

            ui.ch()
                .flex_col()
                .row_gap(2.0)
                .p(Val::Px(8.0))
                .w_full()
                .add(|ui| {
                    ui.ch().header("Resources");

                    let money_text = format!("Current Level: ${:.2}", state.current_level);
                    ui.ch().label(money_text);

                    let wood_text = format!("Game Time: {:.0}", state.game_time);
                    ui.ch().label(wood_text);

                    let rate_text = format!("({:.1}/sec)", state.is_paused);
                    ui.ch().label(rate_text);
                });
        });
    }
}
