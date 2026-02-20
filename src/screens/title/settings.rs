use bevy::audio::Volume;
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
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, SettingsMenu>::new());
    app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
    );
}

#[derive(Component)]
pub struct SettingsMenu;

impl ImmediateAttach<CapsUi> for SettingsMenu {
    // Note: 'static is required for the SystemParam definition
    type Params = Res<'static, GlobalVolume>;

    fn construct(ui: &mut Imm<CapsUi>, global_volume: &mut Res<GlobalVolume>) {
        // Child 1: Header
        ui.ch().header("Settings");

        // Child 2: Grid Container
        ui.ch().apply(style_grid_2col).add(|ui| {
            // Grid Item 1: Label
            ui.ch().label("Master Volume");

            // Grid Item 2: Controls
            ui.ch()
                .flex_row()
                .column_gap(SPACE_2_5)
                .items_center()
                .add(|ui| {
                    let mut btn = ui.ch().button();
                    btn.entity_commands().observe(lower_global_volume);
                    btn.add(|ui| {
                        ui.ch().label("-");
                    });

                    // Dynamic Text
                    let percent = global_volume.volume.to_linear() * 100.0;
                    let text = format!("{percent:3.0}%");

                    ui.ch().label(text);

                    let mut btn = ui.ch().button();
                    btn.entity_commands().observe(raise_global_volume);
                    btn.add(|ui| {
                        ui.ch().label("+");
                    });
                });
        });

        // Child 3: Back Button
        let mut btn = ui.ch().button();
        btn.entity_commands().observe(go_back_on_click);
        btn.add(|ui| {
            ui.ch().label("Back");
        });
    }
}

fn spawn_settings_menu(mut commands: Commands) {
    commands.spawn((
        SettingsMenu,
        // Important: This entity MUST be a Node for bevy_flair to handle its children
        (
            Name::new("Settings Menu"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ),
        DespawnOnExit(Menu::Settings),
    ));
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
    mut next_menu: ResMut<NextState<Menu>>,
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
