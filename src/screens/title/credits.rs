//! The Credits menu (shown over Title).

use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_declarative::prelude::px;

use crate::ui::prelude::*;
use crate::{UiRoot, asset_tracking::LoadResource, audio::music, menus::Menu};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and(input_just_pressed(KeyCode::Escape))),
    );

    app.load_resource::<CreditsAssets>();
    app.add_systems(OnEnter(Menu::Credits), start_credits_music);
}

#[derive(Component)]
pub struct CreditsMenu;

const KEY_COL_WIDTH: f32 = 220.0;
const VAL_COL_WIDTH: f32 = 320.0;

fn credit_row(key: &str, value: &str) -> Div {
    div()
        .flex()
        .row()
        .gap_x(px(SPACE_4))
        .child(
            div()
                .flex()
                .row()
                .justify_end()
                .w(Val::Px(KEY_COL_WIDTH))
                .child(label(key).color(GRAY_400)),
        )
        .child(
            div()
                .flex()
                .row()
                .justify_start()
                .w(Val::Px(VAL_COL_WIDTH))
                .child(label(value)),
        )
}

fn spawn_credits_menu(mut commands: Commands, ui_root: Res<UiRoot>) {
    let root = div()
        .col()
        .items_center()
        .justify_center()
        .w(Val::Percent(100.0))
        .h(Val::Percent(100.0))
        .gap_y(px(SPACE_4))
        .insert((
            CreditsMenu,
            Name::new("Credits Menu"),
            DespawnOnExit(Menu::Credits),
        ))
        // Created by
        .child(heading_2("Created by"))
        .child(
            div()
                .col()
                .gap_y(px(SPACE_2))
                .child(credit_row(
                    "Joe Shmoe",
                    "Implemented alligator wrestling AI",
                ))
                .child(credit_row(
                    "Jane Doe",
                    "Made the music for the alien invasion",
                )),
        )
        // Assets
        .child(heading_2("Assets"))
        .child(
            div()
                .col()
                .gap_y(px(SPACE_2))
                .child(credit_row("Ducky sprite", "CC0 by Caz Creates Games"))
                .child(credit_row("Button SFX", "CC0 by Jaszunio15"))
                .child(credit_row("Music", "CC BY 3.0 by Kevin MacLeod"))
                .child(credit_row(
                    "Bevy logo",
                    "All rights reserved by the Bevy Foundation...",
                )),
        )
        // Back
        .child(btn_primary("Back").on_click(go_back_on_click));

    let menu = root.spawn(&mut commands).id();
    commands.entity(ui_root.0).add_child(menu);
}

// --- Audio & Navigation ---

fn go_back_on_click(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct CreditsAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for CreditsAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Monkeys Spinning Monkeys.ogg"),
        }
    }
}

fn start_credits_music(mut commands: Commands, credits_music: Res<CreditsAssets>) {
    commands.spawn((
        Name::new("Credits Music"),
        DespawnOnExit(Menu::Credits),
        music(credits_music.music.clone()),
    ));
}
