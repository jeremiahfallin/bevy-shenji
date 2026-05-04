//! The Settings menu (shown over Title or Pause).

use bevy::audio::Volume;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_declarative::prelude::px;

use crate::ui::prelude::*;
use crate::{UiRoot, menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    app.add_systems(
        Update,
        (
            update_volume_percent.run_if(in_state(Menu::Settings)),
            go_back.run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
        ),
    );
}

#[derive(Component)]
pub struct SettingsMenu;

/// Marker for the dynamic percent label inside the volume row.
#[derive(Component)]
struct VolumePercentText;

fn format_percent(global_volume: &GlobalVolume) -> String {
    let percent = global_volume.volume.to_linear() * 100.0;
    format!("{percent:3.0}%")
}

fn spawn_settings_menu(
    mut commands: Commands,
    ui_root: Res<UiRoot>,
    global_volume: Res<GlobalVolume>,
) {
    let initial_percent = format_percent(&global_volume);

    let root = div()
        .col()
        .items_center()
        .justify_center()
        .w(Val::Percent(100.0))
        .h(Val::Percent(100.0))
        .gap_y(px(SPACE_4))
        .insert((
            SettingsMenu,
            Name::new("Settings Menu"),
            DespawnOnExit(Menu::Settings),
        ))
        // Header
        .child(heading_2("Settings"))
        // Volume row: label + [- nn% +]
        .child(
            div()
                .flex()
                .row()
                .items_center()
                .gap_x(px(SPACE_6))
                .child(label("Master Volume"))
                .child(
                    div()
                        .flex()
                        .row()
                        .items_center()
                        .gap_x(px(SPACE_2_5))
                        .child(btn_ghost("-").on_click(lower_global_volume))
                        .child(label(initial_percent).insert(VolumePercentText))
                        .child(btn_ghost("+").on_click(raise_global_volume)),
                ),
        )
        // Back button
        .child(btn_primary("Back").on_click(go_back_on_click));

    let menu = root.spawn(&mut commands).id();
    commands.entity(ui_root.0).add_child(menu);
}

fn update_volume_percent(
    mut q: Query<&mut Text, With<VolumePercentText>>,
    global_volume: Res<GlobalVolume>,
) {
    if !global_volume.is_changed() {
        return;
    }
    let text = format_percent(&global_volume);
    for mut t in &mut q {
        t.0 = text.clone();
    }
}

// --- Action Handlers ---

const MIN_VOLUME: f32 = 0.0;
const MAX_VOLUME: f32 = 3.0;

fn lower_global_volume(_: On<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let linear = (global_volume.volume.to_linear() - 0.1).max(MIN_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

fn raise_global_volume(_: On<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let linear = (global_volume.volume.to_linear() + 0.1).min(MAX_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

fn go_back_on_click(
    _: On<Pointer<Click>>,
    screen: Res<State<Screen>>,
    next_menu: ResMut<NextState<Menu>>,
) {
    go_back(screen, next_menu);
}

fn go_back(screen: Res<State<Screen>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}
