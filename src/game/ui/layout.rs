use super::{bottom_bar::BottomBar, content::Content, context_menu::ContextMenuOverlay, sidebar::Sidebar};
use crate::screens::Screen;
use crate::theme::UiRoot;
use crate::theme::prelude::*;
use bevy::prelude::*;

use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, GameLayout>::new());
}

#[derive(Component)]
pub struct GameLayout;

impl ImmediateAttach<CapsUi> for GameLayout {
    type Params = ();

    fn construct(ui: &mut Imm<CapsUi>, _: &mut ()) {
        // 1. Sidebar (Left)
        let mut sidebar = ui.ch().apply(style_sidebar);
        sidebar
            .entity_commands()
            .insert((Name::new("Sidebar"), Sidebar));

        // 2. Right Column (Main View + Bottom Bar)
        let mut right_col = ui.ch().flex_col().h_full().flex_grow();
        right_col
            .entity_commands()
            .insert(Name::new("Right Column"));

        right_col.add(|ui| {
            // Main View
            let mut main_view = ui
                .ch()
                .w_full()
                .flex_grow()
                .scroll_y()
                .bg(Color::srgba(0.0, 0.0, 0.0, 0.2));
            main_view
                .entity_commands()
                .insert((Name::new("Main View"), Content));

            // Bottom Bar
            let mut bottom_bar = ui.ch().apply(style_bottom_bar);
            bottom_bar
                .entity_commands()
                .insert((Name::new("Bottom Bar"), BottomBar));
        });

        // 3. Context Menu Overlay (absolute positioned, floats above everything)
        ui.ch().on_spawn_insert(|| {
            (
                Name::new("Context Menu Overlay"),
                ContextMenuOverlay,
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            )
        });
    }
}

pub fn spawn_game_layout(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.entity(ui_root.0).with_children(|parent| {
        parent.spawn((
            Name::new("Game Layout"),
            GameLayout,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                // Use absolute positioning so the gameplay UI fully covers the
                // screen, preventing any leftover UI from the previous screen
                // (e.g. the main menu) from showing through during the
                // transition frame before DespawnOnExit cleans it up.
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            // Opaque background to fully occlude anything behind
            BackgroundColor(GRAY_900),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
            DespawnOnExit(Screen::Gameplay),
        ));
    });
}
