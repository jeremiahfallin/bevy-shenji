//! Main view container. Spawns the six view marker entities once when the
//! `Content` marker is added and toggles their `Display` based on
//! `UiState::active_view`.
//!
//! Each view (Dashboard / Research / Characters / Squads / Locations /
//! Buildings) carries its own `On<Add, …>` observer (in its module) that
//! populates the entity. We keep entities alive across view switches and
//! only flip `Display::Flex` ↔ `Display::None`, so per-view scroll positions
//! and other transient UI state survive.

use bevy::prelude::*;

use crate::game::resources::{GameView, UiState};
use crate::game::ui::inspector::InspectorState;
use crate::screens::Screen;

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
    app.add_observer(populate_content_on_add);
    app.add_systems(Update, update_active_view.run_if(in_state(Screen::Gameplay)));
    app.add_plugins((
        dashboard::plugin,
        characters::plugin,
        squads::plugin,
        research::plugin,
        locations::plugin,
        buildings::plugin,
    ));
}

#[derive(Component)]
pub struct Content;

/// Marker on each view child entity identifying which `GameView` it represents.
/// Lives alongside the view's own marker (e.g. `DashboardView`).
#[derive(Component)]
struct ContentViewKind(GameView);

fn populate_content_on_add(
    add: On<Add, Content>,
    mut commands: Commands,
    ui_state: Res<UiState>,
) {
    let parent = add.entity;
    let active = ui_state.active_view;
    let display_for = |kind: GameView| {
        if kind == active {
            Display::Flex
        } else {
            Display::None
        }
    };
    let view_node = |kind: GameView| Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        display: display_for(kind),
        ..default()
    };

    let dashboard = commands
        .spawn((
            view_node(GameView::Dashboard),
            ContentViewKind(GameView::Dashboard),
            DashboardView,
        ))
        .id();
    let research = commands
        .spawn((
            view_node(GameView::Research),
            ContentViewKind(GameView::Research),
            ResearchView,
        ))
        .id();
    let characters = commands
        .spawn((
            view_node(GameView::Characters),
            ContentViewKind(GameView::Characters),
            CharactersView,
        ))
        .id();
    let squads = commands
        .spawn((
            view_node(GameView::Squads),
            ContentViewKind(GameView::Squads),
            SquadsView,
        ))
        .id();
    let locations = commands
        .spawn((
            view_node(GameView::Locations),
            ContentViewKind(GameView::Locations),
            LocationsView,
        ))
        .id();
    let buildings = commands
        .spawn((
            view_node(GameView::Buildings),
            ContentViewKind(GameView::Buildings),
            BuildingsView,
        ))
        .id();
    commands.entity(parent).add_children(&[
        dashboard, research, characters, squads, locations, buildings,
    ]);
}

fn update_active_view(
    ui_state: Res<UiState>,
    mut q: Query<(&ContentViewKind, &mut Node)>,
) {
    if !ui_state.is_changed() {
        return;
    }
    for (kind, mut node) in &mut q {
        node.display = if kind.0 == ui_state.active_view {
            Display::Flex
        } else {
            Display::None
        };
    }
}
