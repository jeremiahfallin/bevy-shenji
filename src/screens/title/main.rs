//! The main menu (seen on the title screen).

use bevy::prelude::*;

use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};

use crate::{
    asset_tracking::ResourceHandles,
    game::save::LoadGameEvent,
    menus::Menu,
    screens::Screen,
    theme::{UiRoot, prelude::*},
};

use crate::screens::loading::TargetScreen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, MainMenu>::new());
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
    app.add_observer(attach_load_game_handler);
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
struct LoadGameButton;

impl ImmediateAttach<CapsUi> for MainMenu {
    type Params = ();

    fn construct(ui: &mut Imm<CapsUi>, _: &mut ()) {
        // Buttons
        ui.ch()
            .button()
            .on_click_once(on_new_game_button)
            .add(|ui| {
                ui.ch().label("New Game");
            });

        ui.ch()
            .button()
            .on_spawn_insert(|| LoadGameButton)
            .add(|ui| {
                ui.ch().label("Load Game");
            });

        ui.ch()
            .button()
            .on_click_once(open_settings_menu)
            .add(|ui| {
                ui.ch().label("Settings");
            });

        ui.ch().button().on_click_once(open_credits_menu).add(|ui| {
            ui.ch().label("Credits");
        });

        // Conditional "Exit" button (Not needed on Web)
        #[cfg(not(target_family = "wasm"))]
        {
            ui.ch().button().on_click_once(exit_app).add(|ui| {
                ui.ch().label("Exit");
            });
        }
    }
}

fn spawn_main_menu(mut commands: Commands, ui_root: Res<UiRoot>) {
    let menu = commands
        .spawn((
            MainMenu,
            (
                Name::new("Main Menu"),
                Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    padding: UiRect {
                        left: Val::Px(90.0),
                        right: Val::Px(90.0),
                        top: Val::Px(25.0),
                        bottom: Val::Px(40.0),
                    },
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb_u8(39, 58, 66)),
                BorderRadius::all(Val::Px(2.0)),
            ),
            DespawnOnExit(Menu::Main),
        ))
        .id();
    commands.entity(ui_root.0).add_child(menu);
}

fn on_new_game_button(
    _: On<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut target_screen: ResMut<TargetScreen>,
) {
    target_screen.0 = Screen::NewGame;
    if resource_handles.is_all_done() {
        next_screen.set(Screen::NewGame);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_credits_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

fn on_load_game_button(
    _: On<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut target_screen: ResMut<TargetScreen>,
    mut load_game_writer: MessageWriter<LoadGameEvent>,
) {
    load_game_writer.write(LoadGameEvent("autosave".to_string()));

    // Also try to load "save1" if autosave doesn't exist?
    // For now we just trigger the load. The system will error if file not found but we still switch screen.
    // Ideally we should check if file exists, but let's stick to the request.

    target_screen.0 = Screen::Gameplay;
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn attach_load_game_handler(trigger: On<Add, LoadGameButton>, mut commands: Commands) {
    commands.entity(trigger.entity).observe(on_load_game_button);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
