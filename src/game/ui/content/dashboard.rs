use crate::game::action::{Action, ActionState};
use crate::game::research::ResearchState;
use crate::game::resources::{BaseInventory, NotificationLevel, NotificationState};
use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::{Imm, attach::ImmediateAttach, ui::CapsUi};

#[derive(Component)]
pub struct DashboardView;

impl ImmediateAttach<CapsUi> for DashboardView {
    type Params = (
        Res<'static, BaseInventory>,
        Res<'static, ResearchState>,
        Res<'static, NotificationState>,
        Query<'static, 'static, &'static ActionState>,
    );

    fn construct(
        ui: &mut Imm<CapsUi>,
        (base_inv, research_state, notif_state, action_query): &mut (
            Res<BaseInventory>,
            Res<ResearchState>,
            Res<NotificationState>,
            Query<&ActionState>,
        ),
    ) {
        ui.ch().header("Dashboard");

        ui.ch()
            .flex_col()
            .w_full()
            .p(Val::Px(10.0))
            .row_gap(10.0)
            .add(|ui| {
                // --- Resources Section ---
                ui.ch().flex_col().w_full().add(|ui| {
                    ui.ch().label("Resources").font_bold().mb(Val::Px(5.0));

                    if base_inv.items.is_empty() {
                        ui.ch()
                            .label("No resources yet")
                            .text_color(Color::srgb(0.5, 0.5, 0.5));
                    } else {
                        let mut items: Vec<_> = base_inv.items.iter().collect();
                        items.sort_by_key(|(name, _)| (*name).clone());

                        for (name, count) in items {
                            ui.ch()
                                .flex_row()
                                .justify_between()
                                .w_full()
                                .mb(Val::Px(2.0))
                                .add(|ui| {
                                    ui.ch().label(name).text_color(Color::srgb(0.8, 0.8, 0.8));
                                    ui.ch().label(format!("{}", count)).text_color(Color::WHITE);
                                });
                        }
                    }
                });

                // --- Workers Section ---
                ui.ch().flex_col().w_full().add(|ui| {
                    ui.ch().label("Workers").font_bold().mb(Val::Px(5.0));

                    let mut active = 0u32;
                    let mut idle = 0u32;

                    for action_state in action_query.iter() {
                        match &action_state.current_action {
                            None | Some(Action::Idle) => idle += 1,
                            Some(_) => active += 1,
                        }
                    }

                    ui.ch()
                        .label(format!("Workers: {} active, {} idle", active, idle));
                });

                // --- Research Section ---
                ui.ch().flex_col().w_full().add(|ui| {
                    ui.ch().label("Research").font_bold().mb(Val::Px(5.0));

                    let unlocked_count = research_state.unlocked.len();
                    ui.ch()
                        .label(format!("Technologies unlocked: {}", unlocked_count));
                });

                // --- Notifications Section ---
                ui.ch().flex_col().w_full().add(|ui| {
                    ui.ch().label("Notifications").font_bold().mb(Val::Px(5.0));

                    let notifs: Vec<_> = notif_state.notifications.iter().rev().take(5).collect();

                    if notifs.is_empty() {
                        ui.ch()
                            .label("No notifications")
                            .text_color(Color::srgb(0.5, 0.5, 0.5));
                    } else {
                        for notif in notifs {
                            let color = match notif.level {
                                NotificationLevel::Info => INFO_600,
                                NotificationLevel::Success => SUCCESS_600,
                                NotificationLevel::Error => ERROR_600,
                            };
                            ui.ch()
                                .label(&notif.message)
                                .text_color(color)
                                .mb(Val::Px(2.0));
                        }
                    }
                });
            });
    }
}
