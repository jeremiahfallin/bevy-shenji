use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::{Imm, attach::ImmediateAttach, ui::CapsUi};

#[derive(Component)]
pub struct CharactersView;

impl ImmediateAttach<CapsUi> for CharactersView {
    type Params = ();

    fn construct(ui: &mut Imm<CapsUi>, _: &mut ()) {
        ui.ch().header("Characters");
    }
}
