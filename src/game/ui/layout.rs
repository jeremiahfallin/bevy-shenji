use super::{bottom_bar::BottomBar, content::Content, sidebar::Sidebar};
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
            main_view.entity_commands().insert(Name::new("Main View"));

            main_view.add(|ui| {
                let mut content = ui
                    .ch()
                    .w_full()
                    .min_h(Val::Px(500.0))
                    .bg(Color::srgba(0.1, 0.1, 0.1, 0.5));
                content
                    .entity_commands()
                    .insert((Name::new("Placeholder Map"), Content));
            });

            // Bottom Bar
            let mut bottom_bar = ui.ch().apply(style_bottom_bar);
            bottom_bar
                .entity_commands()
                .insert((Name::new("Bottom Bar"), BottomBar));
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
                ..default()
            },
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ));
    });
}
