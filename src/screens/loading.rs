use bevy::prelude::*;

use crate::ui::prelude::*;
use crate::{UiRoot, asset_tracking::ResourceHandles, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<TargetScreen>();
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);
    app.add_systems(
        Update,
        enter_target_screen.run_if(in_state(Screen::Loading).and(all_assets_loaded)),
    );
}

#[derive(Component)]
pub struct LoadingScreen;

fn spawn_loading_screen(mut commands: Commands, ui_root: Res<UiRoot>) {
    let root = div()
        .col()
        .items_center()
        .justify_center()
        .w(Val::Percent(100.0))
        .h(Val::Percent(100.0))
        .insert((
            LoadingScreen,
            Name::new("Loading Screen"),
            DespawnOnExit(Screen::Loading),
        ))
        .child(label("Loading...").color(Color::WHITE));

    let loading = root.spawn(&mut commands).id();
    commands.entity(ui_root.0).add_child(loading);
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TargetScreen(pub Screen);

fn enter_target_screen(mut next_screen: ResMut<NextState<Screen>>, target: Res<TargetScreen>) {
    next_screen.set(target.0);
}

fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
}
