use bevy::prelude::*;

use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};

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
        // We add children directly to the 'Sidebar' entity (which is the root Node)

        // Static Header
        ui.ch().header("Resources");

        // DYNAMIC LABEL: Money
        // We use entity_commands().insert() to overwrite the Text component every frame/update.
        let money_text = format!("Current Level: ${:.2}", state.current_level);
        ui.ch().label(money_text);

        // DYNAMIC LABEL: Wood
        let wood_text = format!("Game Time: {:.0}", state.game_time);
        ui.ch().label(wood_text);

        // DYNAMIC LABEL: Wood Rate
        let rate_text = format!("({:.1}/sec)", state.is_paused);
        ui.ch().label(rate_text);
    }
}
