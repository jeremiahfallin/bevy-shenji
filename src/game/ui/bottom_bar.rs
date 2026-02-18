use crate::game::resources::{NotificationLevel, NotificationState};
use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, BottomBar>::new());
}

#[derive(Component)]
pub struct BottomBar;

impl ImmediateAttach<CapsUi> for BottomBar {
    type Params = Res<'static, NotificationState>;

    fn construct(ui: &mut Imm<CapsUi>, notifications: &mut Res<NotificationState>) {
        ui.ch()
            .w_full()
            .h_full()
            .flex_col()
            .justify_end()
            .p(Val::Px(4.0))
            .add(|ui| {
                for (i, notification) in notifications.notifications.iter().enumerate() {
                    let bg_color = match notification.level {
                        NotificationLevel::Success => SUCCESS_600,
                        NotificationLevel::Error => ERROR_600,
                        NotificationLevel::Info => INFO_600,
                    };

                    // Fade out during the last second
                    let alpha = notification.ttl.min(1.0).max(0.0);

                    ui.ch_id(format!("notif_{}", i))
                        .style(move |n: &mut Node| {
                            n.padding = UiRect::axes(Val::Px(10.0), Val::Px(6.0));
                            n.margin = UiRect::top(Val::Px(2.0));
                            n.border = UiRect::all(Val::Px(1.0));
                        })
                        .bg(bg_color.with_alpha(alpha * 0.9))
                        .add(|ui| {
                            ui.ch()
                                .label(&notification.message)
                                .text_color(GRAY_100.with_alpha(alpha));
                        });
                }
            });
    }
}
