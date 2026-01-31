use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};

use crate::theme::prelude::*;
use crate::{asset_tracking::LoadResource, audio::music, menus::Menu};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, CreditsMenu>::new());
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

impl ImmediateAttach<CapsUi> for CreditsMenu {
    type Params = ();

    fn construct(ui: &mut Imm<CapsUi>, _: &mut ()) {
        // 1. Header: Created By
        ui.ch().header("Created by");

        // 2. Grid: Created By
        ui.ch().apply(style_grid_2col).add(|ui| {
            add_credit_row(ui, "Joe Shmoe", "Implemented alligator wrestling AI");
            add_credit_row(ui, "Jane Doe", "Made the music for the alien invasion");
        });

        // 3. Header: Assets
        ui.ch().header("Assets");

        // 4. Grid: Assets
        ui.ch().apply(style_grid_2col).add(|ui| {
            add_credit_row(ui, "Ducky sprite", "CC0 by Caz Creates Games");
            add_credit_row(ui, "Button SFX", "CC0 by Jaszunio15");
            add_credit_row(ui, "Music", "CC BY 3.0 by Kevin MacLeod");
            add_credit_row(
                ui,
                "Bevy logo",
                "All rights reserved by the Bevy Foundation...",
            );
        });

        // 5. Back Button
        let mut btn = ui.ch().button();
        btn.entity_commands().observe(go_back_on_click);
        btn.add(|ui| {
            ui.ch().label("Back");
        });
    }
}

// Helper to spawn a row in the grid
fn add_credit_row(ui: &mut Imm<CapsUi>, key: &str, value: &str) {
    // Left Column (Key)
    ui.ch()
        .label(key)
        .justify_self(JustifySelf::End)
        .color(Color::srgb(0.66, 0.66, 0.66)); // Approx #aaaaaa

    // Right Column (Value)
    ui.ch().label(value).justify_self(JustifySelf::Start);
}

fn spawn_credits_menu(mut commands: Commands) {
    commands.spawn((
        CreditsMenu,
        (
            Name::new("Credits Menu"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ),
        DespawnOnExit(Menu::Credits),
    ));
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
