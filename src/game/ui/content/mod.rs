use bevy::prelude::*;
use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};

use crate::game::resources::{GameView, UiState};
use crate::game::ui::inspector::{CharacterInspector, InspectorState};
use crate::theme::prelude::*;

pub mod characters;
pub mod dashboard;
pub mod research;
pub mod squads;

pub use characters::*;
pub use dashboard::*;
pub use research::*;
pub use squads::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<InspectorState>();
    app.add_plugins((
        BevyImmediateAttachPlugin::<CapsUi, Content>::new(),
        BevyImmediateAttachPlugin::<CapsUi, CharacterInspector>::new(),
        BevyImmediateAttachPlugin::<CapsUi, DashboardView>::new(),
        BevyImmediateAttachPlugin::<CapsUi, ResearchView>::new(),
        BevyImmediateAttachPlugin::<CapsUi, CharactersView>::new(),
        BevyImmediateAttachPlugin::<CapsUi, SquadsView>::new(),
    ));
}

#[derive(Component)]
pub struct Content;

impl ImmediateAttach<CapsUi> for Content {
    // Inject the game state resource ('static lifetime is required here)
    type Params = Res<'static, UiState>;

    fn construct(ui: &mut Imm<CapsUi>, ui_state: &mut Res<UiState>) {
        ui.ch()
            .w_full()
            .h_full()
            .p(Val::Px(20.0))
            .add(|ui| match ui_state.active_view {
                GameView::Dashboard => {
                    ui.ch_id("view_dashboard")
                        .w_full()
                        .h_full()
                        .on_spawn_insert(|| DashboardView);
                }
                GameView::Research => {
                    ui.ch_id("view_research")
                        .w_full()
                        .h_full()
                        .on_spawn_insert(|| ResearchView);
                }
                GameView::Characters => {
                    ui.ch_id("view_characters")
                        .w_full()
                        .h_full()
                        .on_spawn_insert(|| CharactersView);
                }
                GameView::Squads => {
                    ui.ch_id("view_squads")
                        .w_full()
                        .h_full()
                        .on_spawn_insert(|| SquadsView);
                }
            });
    }
}
