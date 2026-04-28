//! Manual ScrollView verification harness.
//!
//! Run: `cargo run --example scroll_view_test`
//! Expected: 400x400 viewport with 2000x2000 content. Mouse wheel scrolls
//! content. ScrollPosition reaches `content_size - viewport_size` at limits.

use bevy::prelude::*;
use bevy_declarative::BevyDeclarativePlugin;
use shenji::ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyDeclarativePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    div()
        .w(px(400.0))
        .h(px(400.0))
        .bg(SURFACE_BASE)
        .child(
            scroll_view()
                .horizontal()
                .vertical(true)
                .child(div().w(px(2000.0)).h(px(2000.0)).bg(PRIMARY_500)),
        )
        .spawn(&mut commands);
}
