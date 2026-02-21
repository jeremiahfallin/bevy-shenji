use bevy::prelude::*;
use bevy::window::{CursorIcon, SystemCursorIcon};
use bevy_immediate::{ImmEntity, ImplCap};

use super::style::CapabilityUiLayout;

/// Marker component that declares what cursor an element should show on hover.
///
/// When a node with `UiCursor` is hovered (via `Interaction::Hovered` or `Interaction::Pressed`),
/// a system will set the window cursor accordingly and reset it when the pointer leaves.
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct UiCursor(pub SystemCursorIcon);

/// Fluent API for setting cursor style on UI nodes.
///
/// These methods insert a `UiCursor` marker component. A system registered by
/// the theme plugin reads `Interaction` changes and updates the window cursor.
pub trait ImmUiCursor {
    /// Set a custom system cursor for this node on hover.
    fn cursor(self, icon: SystemCursorIcon) -> Self;

    /// Pointer (hand) cursor — for clickable elements.
    fn cursor_pointer(self) -> Self;

    /// Text (I-beam) cursor — for text inputs.
    fn cursor_text(self) -> Self;

    /// Grab (open hand) cursor — for draggable elements.
    fn cursor_grab(self) -> Self;

    /// Grabbing (closed hand) cursor — while dragging.
    fn cursor_grabbing(self) -> Self;

    /// Move cursor — for moveable elements.
    fn cursor_move(self) -> Self;

    /// Not-allowed cursor — for disabled elements.
    fn cursor_not_allowed(self) -> Self;

    /// Crosshair cursor — for precision selection.
    fn cursor_crosshair(self) -> Self;

    /// Wait cursor — for loading states.
    fn cursor_wait(self) -> Self;

    /// Progress cursor — for partial loading states.
    fn cursor_progress(self) -> Self;

    /// Column resize cursor — for resizable columns.
    fn cursor_col_resize(self) -> Self;

    /// Row resize cursor — for resizable rows.
    fn cursor_row_resize(self) -> Self;

    /// Reset to default cursor.
    fn cursor_default(self) -> Self;
}

impl<Cap> ImmUiCursor for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout>,
{
    fn cursor(mut self, icon: SystemCursorIcon) -> Self {
        self.entity_commands().insert(UiCursor(icon));
        self
    }

    fn cursor_pointer(self) -> Self {
        self.cursor(SystemCursorIcon::Pointer)
    }

    fn cursor_text(self) -> Self {
        self.cursor(SystemCursorIcon::Text)
    }

    fn cursor_grab(self) -> Self {
        self.cursor(SystemCursorIcon::Grab)
    }

    fn cursor_grabbing(self) -> Self {
        self.cursor(SystemCursorIcon::Grabbing)
    }

    fn cursor_move(self) -> Self {
        self.cursor(SystemCursorIcon::Move)
    }

    fn cursor_not_allowed(self) -> Self {
        self.cursor(SystemCursorIcon::NotAllowed)
    }

    fn cursor_crosshair(self) -> Self {
        self.cursor(SystemCursorIcon::Crosshair)
    }

    fn cursor_wait(self) -> Self {
        self.cursor(SystemCursorIcon::Wait)
    }

    fn cursor_progress(self) -> Self {
        self.cursor(SystemCursorIcon::Progress)
    }

    fn cursor_col_resize(self) -> Self {
        self.cursor(SystemCursorIcon::ColResize)
    }

    fn cursor_row_resize(self) -> Self {
        self.cursor(SystemCursorIcon::RowResize)
    }

    fn cursor_default(self) -> Self {
        self.cursor(SystemCursorIcon::Default)
    }
}

/// System that updates the window cursor based on hovered UI nodes with `UiCursor`.
///
/// Register this with `app.add_systems(Update, update_cursor_from_ui)`.
pub fn update_cursor_from_ui(
    ui_query: Query<(&Interaction, &UiCursor), Changed<Interaction>>,
    mut windows: Query<&mut CursorIcon, With<Window>>,
) {
    let mut any_hovered = false;

    for (interaction, ui_cursor) in &ui_query {
        match interaction {
            Interaction::Hovered | Interaction::Pressed => {
                if let Ok(mut cursor) = windows.single_mut() {
                    *cursor = CursorIcon::System(ui_cursor.0);
                    any_hovered = true;
                }
            }
            Interaction::None => {
                // Will be handled by the reset below if no other node is hovered
            }
        }
    }

    // If a node just stopped being hovered and nothing else took over, reset to default
    if !any_hovered {
        for (interaction, _) in &ui_query {
            if *interaction == Interaction::None {
                if let Ok(mut cursor) = windows.single_mut() {
                    *cursor = CursorIcon::default();
                }
                break;
            }
        }
    }
}
