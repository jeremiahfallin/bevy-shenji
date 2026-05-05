//! Bottom bar — stack of fading notifications anchored to the bottom of the
//! main viewport.
//!
//! The notification list ticks every frame (TTL countdown), so we re-render
//! the bar by despawning the inner container's children and rebuilding the
//! tree on every change to `NotificationState`. This is the master design's
//! stated update policy for stateful retained-mode UI; it remains cheap
//! because there are at most a handful of active notifications.

use bevy::color::Alpha;
use bevy::prelude::*;
use bevy_declarative::prelude::px;

use crate::game::resources::{NotificationLevel, NotificationState};
use crate::screens::Screen;
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(populate_bottom_bar_on_add);
    app.add_systems(
        Update,
        refresh_notifications.run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component)]
pub struct BottomBar;

/// Inner container under the BottomBar that holds notification entities.
#[derive(Component)]
struct NotificationContainer;

fn populate_bottom_bar_on_add(
    add: On<Add, BottomBar>,
    mut commands: Commands,
    notifications: Res<NotificationState>,
) {
    let bar = add.entity;

    let container = div()
        .col()
        .w_full()
        .h_full()
        .justify_end()
        .p(px(SPACE_1))
        .insert(NotificationContainer);

    let container_entity = container.spawn(&mut commands).id();
    spawn_notification_children(&mut commands, container_entity, &notifications);
    commands.entity(bar).add_child(container_entity);
}

fn refresh_notifications(
    mut commands: Commands,
    notifications: Res<NotificationState>,
    container_q: Query<Entity, With<NotificationContainer>>,
) {
    if !notifications.is_changed() {
        return;
    }
    for container in &container_q {
        commands.entity(container).despawn_related::<Children>();
        spawn_notification_children(&mut commands, container, &notifications);
    }
}

fn spawn_notification_children(
    commands: &mut Commands,
    container: Entity,
    notifications: &NotificationState,
) {
    for notification in &notifications.notifications {
        let bg_color = match notification.level {
            NotificationLevel::Success => SUCCESS_600,
            NotificationLevel::Error => ERROR_600,
            NotificationLevel::Info => INFO_600,
        };
        // Fade out during the last second.
        let alpha = notification.ttl.clamp(0.0, 1.0);

        let entity = div()
            .pad_x(px(SPACE_2_5))
            .py(px(SPACE_1_5))
            .mt(px(SPACE_0_5))
            .bg(bg_color.with_alpha(alpha * 0.9))
            .child(text(notification.message.clone()).color(GRAY_100.with_alpha(alpha)))
            .spawn(commands)
            .id();
        commands.entity(container).add_child(entity);
    }
}
