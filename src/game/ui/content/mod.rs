use bevy::prelude::*;
use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};

use crate::game::resources::{GameView, UiState};
use crate::game::ui::inspector::{CharacterInspector, InspectorState};
use crate::theme::prelude::*;

pub mod buildings;
pub mod characters;
pub mod dashboard;
pub mod locations;
pub mod research;
pub mod squads;

pub use buildings::*;
pub use characters::*;
pub use dashboard::*;
pub use locations::*;
pub use research::*;
pub use squads::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<InspectorState>();
    app.add_plugins((
        BevyImmediateAttachPlugin::<CapsUi, Content>::new(),
        BevyImmediateAttachPlugin::<CapsUi, CharacterInspector>::new(),
        BevyImmediateAttachPlugin::<CapsUi, ResearchView>::new(),
        BevyImmediateAttachPlugin::<CapsUi, CharactersView>::new(),
        BevyImmediateAttachPlugin::<CapsUi, SquadsView>::new(),
        BevyImmediateAttachPlugin::<CapsUi, LocationsView>::new(),
        BevyImmediateAttachPlugin::<CapsUi, BuildingsView>::new(),
    ));
    app.add_plugins(dashboard::plugin);
}

#[derive(Component)]
pub struct Content;

impl ImmediateAttach<CapsUi> for Content {
    // Inject the game state resource ('static lifetime is required here)
    type Params = Res<'static, UiState>;

    fn construct(ui: &mut Imm<CapsUi>, ui_state: &mut Res<UiState>) {
        let active = ui_state.active_view;
        ui.ch().w_full().h_full().p(Val::Px(SPACE_5)).add(|ui| {
            // Render all views every frame but hide inactive ones via Display::None.
            // This keeps entities alive across view switches, preventing flicker
            // from entity destruction/recreation.
            //
            // We use a single .style() call per view to set all layout properties
            // atomically. This avoids a first-frame issue where chained .style()
            // calls each create a fresh Node::default() (overwriting prior
            // properties) before the entity is flushed to the world.
            let view_style = |is_active: bool| {
                move |n: &mut Node| {
                    n.width = Val::Percent(100.0);
                    n.height = Val::Percent(100.0);
                    n.display = if is_active {
                        Display::Flex
                    } else {
                        Display::None
                    };
                }
            };

            ui.ch_id("view_dashboard")
                .style(view_style(active == GameView::Dashboard))
                .on_spawn_insert(|| DashboardView);

            ui.ch_id("view_research")
                .style(view_style(active == GameView::Research))
                .on_spawn_insert(|| ResearchView);

            ui.ch_id("view_characters")
                .style(view_style(active == GameView::Characters))
                .on_spawn_insert(|| CharactersView);

            ui.ch_id("view_squads")
                .style(view_style(active == GameView::Squads))
                .on_spawn_insert(|| SquadsView);

            ui.ch_id("view_locations")
                .style(view_style(active == GameView::Locations))
                .on_spawn_insert(|| LocationsView);

            ui.ch_id("view_buildings")
                .style(view_style(active == GameView::Buildings))
                .on_spawn_insert(|| BuildingsView);
        });
    }
}
