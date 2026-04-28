//! UI gallery — visual reference for shenji's design system built on bevy_declarative.
//!
//! Run: `cargo run --example ui_gallery`
//!
//! Renders one of each preset and widget so visual changes can be reviewed
//! at a glance. Reference screenshot lives at
//! `docs/plans/2026-04-26-ui-gallery-reference.png` (added when captured).

use bevy::prelude::*;
use bevy_declarative::BevyDeclarativePlugin;
use bevy_declarative::prelude::px;
use lucide_icons::Icon;
use shenji::ui::prelude::*;
use shenji::ui::widgets::{checkbox, radio, slider, tabs, text_input};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyDeclarativePlugin)
        // Register every shenji widget plugin
        .add_plugins(checkbox::plugin)
        .add_plugins(radio::plugin)
        .add_plugins(slider::plugin)
        .add_plugins(tabs::plugin)
        .add_plugins(text_input::plugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    div()
        .w_full()
        .h_full()
        .bg(SURFACE_BASE)
        .child(
            scroll_view().vertical(true).child(
                div()
                    .flex()
                    .col()
                    .p(px(SPACE_4))
                    .child(heading_1("UI Gallery"))
                    .child(body(
                        "Visual reference for shenji's bevy_declarative design system.",
                    ))
                    .child(heading_2("Buttons"))
                    .child(
                        div()
                            .flex()
                            .row()
                            .child(btn_primary("Primary"))
                            .child(btn_ghost("Ghost"))
                            .child(btn_danger("Danger")),
                    )
                    .child(heading_2("Containers"))
                    .child(panel().child(text("Panel content")))
                    .child(card().child(text("Card content")))
                    .child(heading_2("Typography"))
                    .child(heading_1("Heading 1"))
                    .child(heading_2("Heading 2"))
                    .child(heading_3("Heading 3"))
                    .child(body("Body text"))
                    .child(body_sm("Small body text"))
                    .child(caption("Caption"))
                    .child(overline("Overline"))
                    .child(heading_2("Badges & Dividers"))
                    .child(badge("New"))
                    .child(divider())
                    .child(divider_vertical())
                    .child(heading_2("Icons"))
                    .child(icon(Icon::Settings))
                    .child(icon(Icon::Heart))
                    .child(icon(Icon::Search))
                    .child(heading_2("Labels & Progress"))
                    .child(label("Form label"))
                    .child(progress_bar(0.0))
                    .child(progress_bar(0.5))
                    .child(progress_bar(1.0))
                    .child(heading_2("Tooltip"))
                    .child(tooltip("Tooltip body"))
                    .child(heading_2("Stateful widgets"))
                    .child(checkbox::checkbox(false).label("Unchecked"))
                    .child(checkbox::checkbox(true).label("Checked"))
                    .child(checkbox::checkbox(true).label("Disabled").disabled(true))
                    .child(radio::radio("size", "small").label("Small").selected(true))
                    .child(radio::radio("size", "medium").label("Medium"))
                    .child(slider::slider(0.5, 0.0..=1.0))
                    .child(text_input::text_input("hello"))
                    .child(
                        tabs::tabs(0)
                            .tab("Tab A", div().child(text("Tab A content")))
                            .tab("Tab B", div().child(text("Tab B content"))),
                    ),
            ),
        )
        .spawn(&mut commands);
}
