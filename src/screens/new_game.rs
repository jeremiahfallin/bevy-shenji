use bevy::prelude::*;
use bevy_declarative::prelude::px;

use crate::ui::prelude::*;
use crate::{
    UiRoot,
    game::{
        building::Building,
        character::CharacterInfo,
        data::GameData,
        location::{LocationInfo, LocationRegistry},
        research::ResearchState,
        resources::{
            BaseInventory, BaseState, ExplorationState, GameState, PlayerState, SquadState,
        },
        scenarios::{apply_scenario, get_all_scenarios},
        simulation::SimulationState,
    },
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::NewGame), spawn_screen);
}

#[derive(Component)]
pub struct NewGameScreen;

/// Marker on each scenario's Start button. Holds the scenario id so the
/// click observer can look the scenario up at click time.
#[derive(Component)]
struct ScenarioStartButton(String);

fn spawn_screen(mut commands: Commands, ui_root: Res<UiRoot>) {
    let scenarios = get_all_scenarios();

    let mut list = scroll_view().vertical(true);
    for scenario in scenarios {
        let id = scenario.id.clone();
        let stats = div()
            .flex()
            .row()
            .gap_x(px(SPACE_8))
            .my(px(SPACE_2_5))
            .child(label(format!("Gold: {}", scenario.starting_gold)))
            .child(label(format!("Lvl: {}", scenario.starting_level)))
            .child(label(format!(
                "Chars: {}",
                scenario.starting_characters.len()
            )));

        let card_el = card()
            .gap_y(px(SPACE_2))
            .child(
                text(scenario.name.clone())
                    .font_size(28.0)
                    .color(Color::srgb(0.88, 0.67, 0.41)),
            )
            .child(
                text(scenario.description.clone())
                    .font_size(18.0)
                    .color(Color::srgb(0.8, 0.8, 0.8)),
            )
            .child(stats)
            .child(
                btn_primary("Start")
                    .insert(ScenarioStartButton(id))
                    .on_click(start_scenario_on_click),
            );
        list = list.child(card_el);
    }

    let root = div()
        .col()
        .items_center()
        .justify_center()
        .gap_y(px(SPACE_4))
        .w(Val::Percent(100.0))
        .h(Val::Percent(100.0))
        .p(px(SPACE_5))
        .insert((
            NewGameScreen,
            Name::new("New Game Screen"),
            DespawnOnExit(Screen::NewGame),
        ))
        .child(text("Choose how you want to begin your journey:").color(Color::WHITE))
        .child(list)
        .child(btn_primary("Back to Main Menu").on_click(go_back));

    let entity = root.spawn(&mut commands).id();
    commands.entity(ui_root.0).add_child(entity);
}

#[allow(clippy::too_many_arguments)]
fn start_scenario_on_click(
    click: On<Pointer<Click>>,
    q: Query<&ScenarioStartButton>,
    mut commands: Commands,
    mut game: ResMut<GameState>,
    mut player: ResMut<PlayerState>,
    mut squad: ResMut<SquadState>,
    mut base: ResMut<BaseState>,
    mut research: ResMut<ResearchState>,
    mut sim_state: ResMut<SimulationState>,
    mut base_inv: ResMut<BaseInventory>,
    mut exploration_state: ResMut<ExplorationState>,
    mut screen: ResMut<NextState<Screen>>,
    game_data: Res<GameData>,
    mut loc_registry: ResMut<LocationRegistry>,
    old_chars: Query<Entity, With<CharacterInfo>>,
    old_locs: Query<Entity, With<LocationInfo>>,
    old_buildings: Query<Entity, With<Building>>,
) {
    let Ok(marker) = q.get(click.entity) else {
        return;
    };
    let Some(scenario) = get_all_scenarios().into_iter().find(|s| s.id == marker.0) else {
        return;
    };
    let old: Vec<Entity> = old_chars.iter().collect();
    let old_loc: Vec<Entity> = old_locs.iter().collect();
    let old_bldg: Vec<Entity> = old_buildings.iter().collect();
    apply_scenario(
        &mut commands,
        &scenario,
        &mut game,
        &mut player,
        &mut squad,
        &mut base,
        &mut research,
        &mut sim_state,
        &mut base_inv,
        &mut exploration_state,
        &old,
        &game_data,
        &mut loc_registry,
        &old_loc,
        &old_bldg,
    );
    screen.set(Screen::Gameplay);
}

fn go_back(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
