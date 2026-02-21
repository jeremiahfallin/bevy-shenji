use bevy::prelude::*;

use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};

use crate::{
    asset_tracking::ResourceHandles,
    screens::Screen,
    UiRoot,
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, LoadingScreen>::new());
    app.init_resource::<TargetScreen>();
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);
    app.add_systems(
        Update,
        enter_target_screen.run_if(in_state(Screen::Loading).and(all_assets_loaded)),
    );
}

#[derive(Component)]
pub struct LoadingScreen;

impl ImmediateAttach<CapsUi> for LoadingScreen {
    type Params = ();

    fn construct(ui: &mut Imm<CapsUi>, _: &mut ()) {
        // Simple label
        ui.ch().label("Loading...").text_color(Color::WHITE);
    }
}

fn spawn_loading_screen(mut commands: Commands, ui_root: Res<UiRoot>) {
    let loading = commands
        .spawn((
            LoadingScreen,
            (
                Name::new("Loading Screen"),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            ),
            DespawnOnExit(Screen::Loading),
        ))
        .id();
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
