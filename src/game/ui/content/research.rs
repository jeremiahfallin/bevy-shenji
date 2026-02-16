use crate::game::{
    research::{ResearchState, TechTree},
    resources::BaseState,
};
use crate::theme::{prelude::*, scroll::ImmUiScrollExt};
use bevy::prelude::*;
use bevy_immediate::{Imm, attach::ImmediateAttach, ui::CapsUi};

#[derive(Component)]
pub struct ResearchView;

impl ImmediateAttach<CapsUi> for ResearchView {
    type Params = (
        Res<'static, TechTree>,
        ResMut<'static, ResearchState>,
        ResMut<'static, BaseState>,
    );

    fn construct(
        ui: &mut Imm<CapsUi>,
        (tech_tree, research_state, base_state): &mut (
            Res<TechTree>,
            ResMut<ResearchState>,
            ResMut<BaseState>,
        ),
    ) {
        ui.ch().header("Research Tree");

        // Horizontal Scroll Area for the Tiers
        ui.ch().flex_row().w_full().h_full().scroll_x().add(|ui| {
            // Loop through Tiers (0 to 3 for example)
            for tier in 0..=3 {
                ui.ch()
                    .flex_col()
                    .w(Val::Px(250.0))
                    .p(Val::Px(10.0))
                    .add(|ui| {
                        ui.ch().label(format!("Tier {}", tier)).mb(Val::Px(10.0));

                        // Find techs in this tier
                        let mut techs_in_tier: Vec<_> =
                            tech_tree.defs.values().filter(|t| t.tier == tier).collect();

                        // Sort by name or ID for stability
                        techs_in_tier.sort_by_key(|t| &t.name);

                        for tech in techs_in_tier {
                            let is_unlocked = research_state.is_unlocked(&tech.id);
                            let can_research = research_state.can_research(&tech.id, tech_tree);
                            let can_afford = base_state.value.research_level >= tech.cost;

                            // Card container
                            ui.ch()
                                .flex_col()
                                .p(Val::Px(10.0))
                                .mb(Val::Px(10.0))
                                .rounded(4.0)
                                .bg(if is_unlocked {
                                    Color::srgb(0.2, 0.5, 0.2) // Green (Done)
                                } else if can_research {
                                    Color::srgb(0.3, 0.3, 0.3) // Gray (Available)
                                } else {
                                    Color::srgb(0.1, 0.1, 0.1) // Dark (Locked)
                                })
                                .add(|ui| {
                                    ui.ch().label(&tech.name).font_bold();
                                    ui.ch()
                                        .label(&tech.description)
                                        .text_size(12.0)
                                        .text_color(Color::srgb(0.8, 0.8, 0.8));

                                    if !is_unlocked {
                                        ui.ch().label(format!("Cost: {}", tech.cost)).text_color(
                                            if can_afford {
                                                Color::WHITE
                                            } else {
                                                Color::srgb(0.8, 0.2, 0.2)
                                            },
                                        );

                                        // Research Button
                                        if can_research {
                                            let tech_id = tech.id.clone();
                                            let cost = tech.cost;

                                            ui.ch().button()
                                                .mt(Val::Px(5.0))
                                                .disabled(!can_afford)
                                                .on_click_once(move |_: On<Pointer<Click>>,
                                                               mut r_state: ResMut<ResearchState>,
                                                               mut b_state: ResMut<BaseState>| {
                                                    // Double check logic inside the handler
                                                    if b_state.value.research_level >= cost {
                                                        b_state.value.research_level -= cost;
                                                        r_state.unlocked.insert(tech_id.clone());
                                                    }
                                                })
                                                .add(|ui| { ui.ch().label("Research"); });
                                        } else {
                                            ui.ch()
                                                .label("Locked via Deps")
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
