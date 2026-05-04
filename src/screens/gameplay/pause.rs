//! The Pause menu (shown over Gameplay).

use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_declarative::prelude::px;

use crate::ui::prelude::*;
use crate::{UiRoot, menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}

#[derive(Component)]
pub struct PauseMenu;

fn spawn_pause_menu(mut commands: Commands, ui_root: Res<UiRoot>) {
    let root = div()
        .col()
        .items_center()
        .justify_center()
        .w(Val::Percent(100.0))
        .h(Val::Percent(100.0))
        .gap_y(px(SPACE_4))
        .insert((
            PauseMenu,
            Name::new("Pause Menu"),
            DespawnOnExit(Menu::Pause),
        ))
        .child(heading_2("Game Paused"))
        .child(btn_primary("Continue").on_click(close_menu))
        .child(btn_primary("Settings").on_click(open_settings_menu))
        .child(btn_primary("Quit to Title").on_click(quit_to_title));

    let menu = root.spawn(&mut commands).id();
    commands.entity(ui_root.0).add_child(menu);
}

// --- Actions ---

fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn close_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn quit_to_title(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
