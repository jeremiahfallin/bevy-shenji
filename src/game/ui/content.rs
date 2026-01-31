use bevy::prelude::*;
use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};

use crate::game::resources::SquadState;
use crate::theme::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, Content>::new());
}

#[derive(Component)]
pub struct Content;

impl ImmediateAttach<CapsUi> for Content {
    // Inject the game state resource ('static lifetime is required here)
    type Params = Res<'static, SquadState>;

    fn construct(ui: &mut Imm<CapsUi>, state: &mut Res<SquadState>) {
        ui.ch()
            .on_spawn_insert(|| (Name::new("Content"), Node::default()))
            .add(|ui| {
                ui.ch().header("Squads");

                // We use entity_commands().insert() to overwrite the Text component every frame/update.
                let money_text = format!("Number of characters: {}", state.characters.len());
                ui.ch().label(money_text);

                ui.ch().header("Squads");
                for squad in state.squads.values() {
                    ui.ch().label(squad.name.clone());
                    ui.ch().label(format!("Members: {}", squad.members.len()));
                    for member in squad.members.iter() {
                        ui.ch().label(format!("{}", state.characters[member].name));
                    }
                }
            });
    }
}
