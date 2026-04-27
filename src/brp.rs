//! Bevy Remote Protocol (BRP) integration for dev builds.
//! Exposes game state inspection and screenshot capture over HTTP (port 15702).

use bevy::prelude::*;
use bevy::remote::{BrpError, RemotePlugin, http::RemoteHttpPlugin};
use bevy::render::view::window::screenshot::{Screenshot, save_to_disk};
use serde_json::Value;

use crate::game::resources::UiState;
use crate::menus::Menu;
use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(
        RemotePlugin::default()
            .with_method("shenji/game_state", game_state_handler)
            .with_method("shenji/screenshot", screenshot_handler),
    )
    .add_plugins(RemoteHttpPlugin::default());
}

/// Returns the current game state: screen, menu, and active UI view.
fn game_state_handler(
    In(_params): In<Option<Value>>,
    screen: Res<State<Screen>>,
    menu: Res<State<Menu>>,
    ui_state: Option<Res<UiState>>,
) -> Result<Value, BrpError> {
    let active_view = ui_state.map(|s| format!("{:?}", s.active_view));
    Ok(serde_json::json!({
        "screen": format!("{:?}", screen.get()),
        "menu": format!("{:?}", menu.get()),
        "active_view": active_view,
    }))
}

/// Takes a screenshot saved to `./screenshots/screenshot-<timestamp>.png`.
/// Returns `{"path": "<absolute_path>"}`.
fn screenshot_handler(
    In(_params): In<Option<Value>>,
    mut commands: Commands,
) -> Result<Value, BrpError> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let relative = format!("./screenshots/screenshot-{timestamp}.png");
    std::fs::create_dir_all("./screenshots").ok();

    let abs_path = std::path::Path::new(&relative)
        .canonicalize()
        .unwrap_or_else(|_| std::path::PathBuf::from(&relative));

    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(relative));

    Ok(serde_json::json!({
        "path": abs_path.display().to_string(),
    }))
}
