//! Shenji's design system, built on `bevy_declarative`.
//!
//! Replaces the older `bevy_immediate`-based `src/theme/` module. Both
//! coexist during the migration; this module owns the post-port UI.

#![allow(dead_code)]

pub mod behaviors;
pub mod components;
pub mod prelude;
pub mod presets;
pub mod tokens;
pub mod widgets;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(widgets::checkbox::plugin);
    app.add_plugins(widgets::radio::plugin);
    app.add_plugins(widgets::slider::plugin);
    app.add_plugins(widgets::tabs::plugin);
    app.add_plugins(widgets::text_input::plugin);
}
