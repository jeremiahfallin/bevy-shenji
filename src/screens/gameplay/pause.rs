use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};

use crate::theme::prelude::*;
use crate::{menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, PauseMenu>::new());
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}

#[derive(Component)]
pub struct PauseMenu;

impl ImmediateAttach<CapsUi> for PauseMenu {
    type Params = ();

    fn construct(ui: &mut Imm<CapsUi>, _: &mut ()) {
        ui.ch().header("Game Paused");

        // Continue
        let mut btn = ui.ch().button();
        btn.entity_commands().observe(close_menu);
        btn.add(|ui| {
            ui.ch().label("Continue");
        });

        // Settings
        let mut btn = ui.ch().button();
        btn.entity_commands().observe(open_settings_menu);
        btn.add(|ui| {
            ui.ch().label("Settings");
        });

        // Quit
        let mut btn = ui.ch().button();
        btn.entity_commands().observe(quit_to_title);
        btn.add(|ui| {
            ui.ch().label("Quit to Title");
        });
    }
}

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn((
        PauseMenu,
        (
            Name::new("Pause Menu"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ),
        DespawnOnExit(Menu::Pause),
    ));
}

// --- Actions (Unchanged) ---

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
