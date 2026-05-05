//! The gameplay screen.

mod pause;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    Pause, UiRoot, game::simulation::SimulationState, game::ui::spawn_game_layout, menus::Menu,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(pause::plugin);

    app.add_systems(OnEnter(Screen::Gameplay), spawn_game_layout);

    // Toggle pause on key press.
    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(
                in_state(Screen::Gameplay)
                    .and(in_state(Menu::None))
                    .and(input_just_pressed(KeyCode::KeyP).or(input_just_pressed(KeyCode::Escape))),
            ),
            close_menu.run_if(
                in_state(Screen::Gameplay)
                    .and(not(in_state(Menu::None)))
                    .and(input_just_pressed(KeyCode::KeyP)),
            ),
        ),
    );
    app.add_systems(OnExit(Screen::Gameplay), (close_menu, unpause));
    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component)]
pub struct PauseOverlay;

fn spawn_pause_overlay(mut commands: Commands, ui_root: Res<UiRoot>) {
    let overlay = commands
        .spawn((
            PauseOverlay,
            Name::new("Pause Overlay"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            DespawnOnExit(Pause(true)),
        ))
        .id();
    commands.entity(ui_root.0).add_child(overlay);
}

fn unpause(mut next_pause: ResMut<NextState<Pause>>, mut sim: ResMut<SimulationState>) {
    NextState::set_if_neq(&mut next_pause, Pause(false));
    // Restore simulation to the speed it was running at before pause.
    if sim.is_paused() {
        let prev = sim.previous_speed.max(1);
        sim.set_speed(prev);
    }
}

fn pause(mut next_pause: ResMut<NextState<Pause>>, mut sim: ResMut<SimulationState>) {
    NextState::set_if_neq(&mut next_pause, Pause(true));
    sim.pause();
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    NextState::set_if_neq(&mut next_menu, Menu::None);
}
