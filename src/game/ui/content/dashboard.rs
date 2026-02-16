use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::{Imm, attach::ImmediateAttach, ui::CapsUi};

#[derive(Component)]
pub struct DashboardView;

impl ImmediateAttach<CapsUi> for DashboardView {
    type Params = ();

    fn construct(ui: &mut Imm<CapsUi>, _: &mut ()) {
        ui.ch().header("Dashboard");
    }
}
