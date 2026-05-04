//! The main menu (seen on the title screen).

use bevy::color::Alpha;
use bevy::prelude::*;

use crate::{
    UiRoot, asset_tracking::ResourceHandles, game::save::LoadGameMessage, menus::Menu,
    screens::Screen, ui::prelude::*,
};

use crate::screens::loading::TargetScreen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
    app.add_observer(attach_load_game_handler);
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
struct LoadGameButton;

fn menu_button(label: &str) -> Div {
    btn_primary(label).w(Val::Px(240.0))
}

fn spawn_main_menu(mut commands: Commands, ui_root: Res<UiRoot>, asset_server: Res<AssetServer>) {
    let title_font: Handle<Font> = asset_server.load("fonts/Kenney Space.ttf");

    let mut root = div()
        .col()
        .items_center()
        .justify_center()
        .gap_y(Val::Px(SPACE_2_5))
        .pad_x(Val::Px(SPACE_24))
        .pt(Val::Px(SPACE_6))
        .pb(Val::Px(SPACE_10))
        .insert((MainMenu, Name::new("Main Menu"), DespawnOnExit(Menu::Main)))
        // Title text: "SHENJI" in Kenney Space 72px gold
        .child(
            text("SHENJI")
                .font_size(72.0)
                .color(GOLD_400)
                .mb(Val::Px(SPACE_1))
                .insert((
                    TextFont {
                        font: title_font.clone(),
                        font_size: 72.0,
                        ..default()
                    },
                    TextLayout {
                        justify: Justify::Center,
                        ..default()
                    },
                    LINE_HEIGHT_NORMAL,
                )),
        )
        // Subtitle
        .child(
            text("A tale of strategy and survival")
                .font_size(14.0)
                .color(GRAY_500)
                .mb(Val::Px(SPACE_10))
                .insert(TextLayout {
                    justify: Justify::Center,
                    ..default()
                }),
        )
        // Decorative separator line
        .child(
            div()
                .w(Val::Px(200.0))
                .h(Val::Px(1.0))
                .mb(Val::Px(SPACE_8))
                .bg(GOLD_500.with_alpha(0.3)),
        )
        // Menu buttons
        .child(menu_button("New Game").on_click(on_new_game_button))
        .child(menu_button("Load Game").insert(LoadGameButton))
        .child(menu_button("Settings").on_click(open_settings_menu))
        .child(menu_button("Credits").on_click(open_credits_menu));

    // Conditional "Exit" button (Not needed on Web)
    #[cfg(not(target_family = "wasm"))]
    {
        root = root.child(menu_button("Exit").on_click(exit_app));
    }

    let menu = root.spawn(&mut commands).id();
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
    mut load_game_writer: MessageWriter<LoadGameMessage>,
) {
    load_game_writer.write(LoadGameMessage("autosave".to_string()));

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
