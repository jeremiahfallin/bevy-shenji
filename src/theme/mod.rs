#![allow(dead_code)]
pub mod behaviors;
pub mod components;
pub mod extensions;
pub mod primitives;
pub mod resources;
pub mod scroll;
pub mod styles;
pub mod widgets;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::behaviors::*;
    pub use super::components::*;
    pub use super::extensions::*;
    pub use super::primitives::*;
    pub use super::resources::*;
    pub use super::scroll::*;
    pub use super::styles::buttons::*;
    pub use super::styles::containers::*;
    pub use super::styles::grids::*;
    pub use super::styles::palette::*;
    pub use super::styles::typography::*;
    pub use super::widgets::*;
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(behaviors::plugin);
    app.add_plugins(scroll::ScrollWidgetPlugin);
}

#[derive(Resource)]
pub struct UiRoot(pub Entity);
