use bevy::prelude::*;

use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};

use crate::{
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
    theme::{UiRoot, prelude::*, scroll::ImmUiScrollExt},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, NewGameScreen>::new());
    app.add_systems(OnEnter(Screen::NewGame), spawn_screen);
}

#[derive(Component)]
pub struct NewGameScreen;

impl ImmediateAttach<CapsUi> for NewGameScreen {
    // We request Commands to spawn the Location, and ResMut to update state
    type Params = (
        Commands<'static, 'static>,
        ResMut<'static, NextState<Screen>>,
        ResMut<'static, GameState>,
        ResMut<'static, PlayerState>,
        ResMut<'static, SquadState>,
    );

    fn construct(
        ui: &mut Imm<CapsUi>,
        (_commands, _next_screen, _game_state, _player_state, _squad_state): &mut <Self::Params as bevy::ecs::system::SystemParam>::Item<'_, '_>,
    ) {
        // Main Container (Central Panel)
        ui.ch()
            .apply(style_panel_central)
            .add(|ui| {
                ui.ch().label("Choose how you want to begin your journey:").text_color(Color::WHITE);

                // Scenarios List
                ui.ch()
                    .flex_col().w_full().flex_grow().min_h(Val::Px(0.0)).gap(40.0).p(Val::Px(10.0)) // scenario-list styles
                    .add(|ui| {
                        ui.ch().scrollarea(
                            |n| {
                                n.width = Val::Percent(100.0);
                                n.flex_grow = 1.0;
                                n.flex_direction = FlexDirection::Column;
                                n.align_items = AlignItems::Center;
                                n.row_gap = Val::Px(10.0);
                                n.overflow = Overflow {
                                    y: OverflowAxis::Scroll,
                                    ..default()
                                };
                            },
                            |scroll_ui| {
                                for scenario in get_all_scenarios() {
                                    // Scenario Card
                                    scroll_ui.ch()
                                        .apply(style_card)
                                        .add(|ui| {
                                            ui.ch().label(scenario.name.clone()).text_size(28.0).color(Color::srgb(0.88, 0.67, 0.41)); // scenario-title
                                            ui.ch().label(scenario.description.clone()).text_size(18.0).color(Color::srgb(0.8, 0.8, 0.8)); // scenario-description


                                    // Stats Row
                                    ui.ch()
                                        .flex_row().gap(30.0).my(Val::Px(10.0)) // horizontal
                                        .add(|ui| {
                                            ui.ch().label(format!(
                                                    "Gold: {}",
                                                    scenario.starting_gold
                                                ));
                                            ui.ch().label(format!(
                                                    "Lvl: {}",
                                                    scenario.starting_level
                                                ));
                                            ui.ch().label(format!(
                                                    "Chars: {}",
                                                    scenario.starting_characters.len()
                                                ));
                                        });

                                    // Start Button
                                    let s = scenario.clone();

                                    ui.ch().button()
                                        .on_click_once(move |_: On<Pointer<Click>>,
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
                                                      old_buildings: Query<Entity, With<Building>>| {
                                                    let old: Vec<Entity> = old_chars.iter().collect();
                                                    let old_loc: Vec<Entity> = old_locs.iter().collect();
                                                    let old_bldg: Vec<Entity> = old_buildings.iter().collect();
                                                    apply_scenario(&mut commands, &s, &mut game, &mut player, &mut squad, &mut base, &mut research, &mut sim_state, &mut base_inv, &mut exploration_state, &old, &game_data, &mut loc_registry, &old_loc, &old_bldg);
                                                    screen.set(Screen::Gameplay);
                                                })
                                        .add(|ui| { ui.ch().label("Start"); });
                                });
                        }
                    });

                    ui.ch().flex_col().w_full().items_center().button()
                        .w(Val::Px(200.0))
                        .on_click_once(go_back)
                        .add(|ui| { ui.ch().label("Back to Main Menu").text_color(Color::WHITE); });
            });
        });
    }
}

fn spawn_screen(mut commands: Commands, ui_root: Res<UiRoot>) {
    let entity = commands
        .spawn((
            NewGameScreen,
            (
                Name::new("New Game Screen"),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ),
            DespawnOnExit(Screen::NewGame),
        ))
        .id();
    commands.entity(ui_root.0).add_child(entity);
}

fn go_back(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
