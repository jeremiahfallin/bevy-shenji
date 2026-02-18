use crate::game::{data::GameData, research::ResearchState, resources::BaseInventory};
use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::{Imm, attach::ImmediateAttach, ui::CapsUi};

#[derive(Component)]
pub struct ResearchView;

impl ImmediateAttach<CapsUi> for ResearchView {
    type Params = (
        Res<'static, GameData>,
        ResMut<'static, ResearchState>,
        Res<'static, BaseInventory>,
    );

    fn construct(
        ui: &mut Imm<CapsUi>,
        (game_data, research_state, base_inventory): &mut (
            Res<GameData>,
            ResMut<ResearchState>,
            Res<BaseInventory>,
        ),
    ) {
        ui.ch().header("Research Tree");

        // Show current research progress if any
        if let Some(ref current_id) = research_state.current_research {
            if let Some(def) = game_data.get_research(current_id) {
                let progress_frac = if def.time > 0 {
                    research_state.research_progress as f32 / def.time as f32
                } else {
                    1.0
                };
                ui.ch()
                    .flex_row()
                    .w_full()
                    .p(Val::Px(8.0))
                    .mb(Val::Px(10.0))
                    .rounded(4.0)
                    .bg(Color::srgb(0.15, 0.15, 0.3))
                    .add(|ui| {
                        ui.ch().label(format!(
                            "Researching: {} ({:.0}%)",
                            def.name,
                            progress_frac * 100.0
                        ));
                    });
            }
        }

        // Determine the maximum tech_level present in data
        let max_tech_level = game_data
            .research
            .values()
            .map(|r| r.tech_level)
            .max()
            .unwrap_or(1);

        // Horizontal Scroll Area for the Tech Levels
        ui.ch().flex_row().w_full().h_full().scroll_x().add(|ui| {
            for tech_level in 1..=max_tech_level {
                ui.ch()
                    .flex_col()
                    .w(Val::Px(280.0))
                    .p(Val::Px(10.0))
                    .add(|ui| {
                        ui.ch()
                            .label(format!("Tech Level {}", tech_level))
                            .mb(Val::Px(10.0));

                        // Collect and sort research for this tech level
                        let mut research_in_level: Vec<_> = game_data
                            .research
                            .values()
                            .filter(|r| r.tech_level == tech_level)
                            .collect();
                        research_in_level.sort_by_key(|r| &r.name);

                        for research in research_in_level {
                            let is_unlocked = research_state.is_unlocked(&research.id);
                            let can_research =
                                research_state.can_research(&research.id, game_data);

                            // Check if the player can afford the cost
                            let can_afford = research.cost.iter().all(|(item_id, &amount)| {
                                base_inventory.count(item_id) >= amount
                            });

                            let is_current = research_state.current_research.as_deref()
                                == Some(research.id.as_str());

                            // Card container
                            ui.ch()
                                .flex_col()
                                .p(Val::Px(10.0))
                                .mb(Val::Px(10.0))
                                .rounded(4.0)
                                .bg(if is_unlocked {
                                    Color::srgb(0.2, 0.5, 0.2) // Green (Done)
                                } else if is_current {
                                    Color::srgb(0.2, 0.2, 0.5) // Blue (In Progress)
                                } else if can_research {
                                    Color::srgb(0.3, 0.3, 0.3) // Gray (Available)
                                } else {
                                    Color::srgb(0.1, 0.1, 0.1) // Dark (Locked)
                                })
                                .add(|ui| {
                                    // Name and type
                                    ui.ch().label(&research.name).font_bold();
                                    ui.ch()
                                        .label(format!("Type: {}", research.research_type))
                                        .text_size(11.0)
                                        .text_color(Color::srgb(0.7, 0.7, 0.7));

                                    if !is_unlocked {
                                        // Show cost
                                        if !research.cost.is_empty() {
                                            let cost_str: Vec<String> = research
                                                .cost
                                                .iter()
                                                .map(|(item_id, amount)| {
                                                    let item_name = game_data
                                                        .get_item(item_id)
                                                        .map(|i| i.name.as_str())
                                                        .unwrap_or(item_id.as_str());
                                                    let have = base_inventory.count(item_id);
                                                    format!("{} {}/{}", item_name, have, amount)
                                                })
                                                .collect();
                                            ui.ch()
                                                .label(format!("Cost: {}", cost_str.join(", ")))
                                                .text_size(11.0)
                                                .text_color(if can_afford {
                                                    Color::WHITE
                                                } else {
                                                    Color::srgb(0.8, 0.2, 0.2)
                                                });
                                        }

                                        // Show time
                                        ui.ch()
                                            .label(format!("Time: {} ticks", research.time))
                                            .text_size(11.0)
                                            .text_color(Color::srgb(0.7, 0.7, 0.7));

                                        // Show prerequisites
                                        if !research.prerequisites.is_empty() {
                                            let prereq_names: Vec<String> = research
                                                .prerequisites
                                                .iter()
                                                .map(|pid| {
                                                    game_data
                                                        .get_research(pid)
                                                        .map(|r| r.name.as_str())
                                                        .unwrap_or(pid.as_str())
                                                        .to_string()
                                                })
                                                .collect();
                                            ui.ch()
                                                .label(format!(
                                                    "Requires: {}",
                                                    prereq_names.join(", ")
                                                ))
                                                .text_size(10.0)
                                                .text_color(Color::srgb(0.6, 0.6, 0.8));
                                        }

                                        // Research Button
                                        if is_current {
                                            ui.ch()
                                                .label("In Progress...")
                                                .text_size(10.0)
                                                .text_color(Color::srgb(0.4, 0.4, 0.9));
                                        } else if can_research {
                                            let research_id = research.id.clone();
                                            let cost_clone = research.cost.clone();

                                            ui.ch()
                                                .button()
                                                .mt(Val::Px(5.0))
                                                .disabled(!can_afford)
                                                .on_click_once(
                                                    move |_: On<Pointer<Click>>,
                                                          mut r_state: ResMut<ResearchState>,
                                                          mut b_inv: ResMut<BaseInventory>| {
                                                        // Deduct costs from base inventory
                                                        let affordable = cost_clone
                                                            .iter()
                                                            .all(|(item_id, &amount)| {
                                                                b_inv.count(item_id) >= amount
                                                            });
                                                        if affordable {
                                                            for (item_id, &amount) in &cost_clone {
                                                                b_inv.remove(item_id, amount);
                                                            }
                                                            r_state.current_research =
                                                                Some(research_id.clone());
                                                            r_state.research_progress = 0;
                                                        }
                                                    },
                                                )
                                                .add(|ui| {
                                                    ui.ch().label("Research");
                                                });
                                        } else {
                                            ui.ch()
                                                .label("Locked")
                                                .text_size(10.0)
                                                .text_color(Color::srgb(0.8, 0.2, 0.2));
                                        }
                                    } else {
                                        ui.ch()
                                            .label("Completed")
                                            .text_size(10.0)
                                            .text_color(Color::srgb(0.2, 0.8, 0.2));
                                    }
                                });
                        }
                    });
            }
        });
    }
}
