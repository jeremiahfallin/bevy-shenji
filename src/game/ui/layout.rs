//! Gameplay layout shell — left sidebar / right column with main view + bottom
//! bar / context-menu overlay.
//!
//! Spawned by `spawn_game_layout` on `OnEnter(Screen::Gameplay)`. An
//! `On<Add, GameLayout>` observer attaches the structural children carrying
//! the `Sidebar`, `Content`, `BottomBar`, and `ContextMenuOverlay` markers.
//! Each child's own observer (in its module) populates its tree.

use bevy::prelude::*;
use bevy_declarative::prelude::px;

use super::{
    bottom_bar::BottomBar, content::Content, context_menu::ContextMenuOverlay, sidebar::Sidebar,
};
use crate::UiRoot;
use crate::screens::Screen;
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(populate_layout_on_add);
}

#[derive(Component)]
pub struct GameLayout;

fn populate_layout_on_add(add: On<Add, GameLayout>, mut commands: Commands) {
    let layout = add.entity;

    // Sidebar (left).
    let sidebar = div()
        .col()
        .h_full()
        .min_w(Val::Px(250.0))
        .flex_shrink(0.0)
        .insert((Name::new("Sidebar"), Sidebar))
        .spawn(&mut commands)
        .id();

    // Right column: main view (Content) + bottom bar.
    let main_view = div()
        .col()
        .w_full()
        .flex_grow(1.0)
        .min_h(Val::Px(0.0))
        .bg(Color::srgba(0.0, 0.0, 0.0, 0.2))
        .insert((Name::new("Main View"), Content))
        .spawn(&mut commands)
        .id();

    let bottom_bar = div()
        .w_full()
        .h(Val::Px(80.0))
        .flex_shrink(0.0)
        .insert((Name::new("Bottom Bar"), BottomBar))
        .spawn(&mut commands)
        .id();

    let right_column = div()
        .col()
        .h_full()
        .flex_grow(1.0)
        .insert(Name::new("Right Column"))
        .spawn(&mut commands)
        .id();
    commands
        .entity(right_column)
        .add_children(&[main_view, bottom_bar]);

    commands
        .entity(layout)
        .add_children(&[sidebar, right_column]);

    // Context-menu overlay (absolute, floats above everything).
    let context_overlay = commands
        .spawn((
            Name::new("Context Menu Overlay"),
            ContextMenuOverlay,
            Node::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .id();
    commands.entity(layout).add_child(context_overlay);

    let _ = px;
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
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            BackgroundColor(GRAY_900),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
            DespawnOnExit(Screen::Gameplay),
        ));
    });
}
