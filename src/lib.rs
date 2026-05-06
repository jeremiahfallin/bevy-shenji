//! `shenji` library crate — exposes the app surface for the binary
//! (`src/main.rs`) and integration tests under `tests/`.

#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]

pub mod asset_tracking;
pub mod audio;
#[cfg(feature = "dev")]
pub mod brp;
#[cfg(feature = "dev")]
pub mod dev_tools;
pub mod game;
pub mod menus;
pub mod screens;
pub mod theme;
pub mod ui;

use crate::theme::prelude::*;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_declarative::BevyDeclarativePlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Shenji".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        )
        .add_plugins(BevyDeclarativePlugin);

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            audio::plugin,
            game::plugin,
            #[cfg(feature = "dev")]
            brp::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
            ui::plugin,
        ));

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

        // Initialize UiRoot immediately to ensure it exists for all screens.
        let root = app
            .world_mut()
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(SPACE_5),
                    ..default()
                },
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
                Name::new("UiRoot"),
            ))
            .id();
        app.insert_resource(UiRoot(root));

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PausableSystems;

/// The root UI entity that all screens attach their UI to.
#[derive(Resource)]
pub struct UiRoot(pub Entity);

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}
