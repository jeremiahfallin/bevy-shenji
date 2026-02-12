//! The gameplay screen.

mod pause;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};

use crate::{
    Pause,
    game::ui::spawn_game_layout,
    menus::Menu,
    screens::Screen,
    theme::{UiRoot, prelude::*},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        pause::plugin,
        BevyImmediateAttachPlugin::<CapsUi, PauseOverlay>::new(),
    ));

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

impl ImmediateAttach<CapsUi> for PauseOverlay {
    type Params = ();

    fn construct(ui: &mut Imm<CapsUi>, _: &mut ()) {
        ui.ch()
            .w_full()
            .h_full()
            .bg(Color::srgba(0.0, 0.0, 0.0, 0.8));
    }
}

fn spawn_pause_overlay(mut commands: Commands, ui_root: Res<UiRoot>) {
    let overlay = commands
        .spawn((
            PauseOverlay,
            Name::new("Pause Overlay"),
            Node::default(),
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            DespawnOnExit(Pause(true)),
        ))
        .id();
    commands.entity(ui_root.0).add_child(overlay);
}

fn unpause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(false));
}

fn pause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(true));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
