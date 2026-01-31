use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, BottomBar>::new());
}

#[derive(Component)]
pub struct BottomBar;

impl ImmediateAttach<CapsUi> for BottomBar {
    type Params = ();

    fn construct(ui: &mut Imm<CapsUi>, _: &mut ()) {
        ui.ch().label("Bottom Bar Area");
    }
}
