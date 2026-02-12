use crate::game::{
    character::{CharacterInfo, Health},
    resources::SquadState,
};
use crate::theme::prelude::*;
use bevy::prelude::*;

use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, CharacterCard>::new());
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, SquadsDisplay>::new());
}

pub fn render_character_card(ui: &mut Imm<CapsUi>, info: &CharacterInfo, health: &Health) {
    ui.ch()
        .flex_col()
        .p(Val::Px(16.0))
        .bg(Color::srgba(0.12, 0.14, 0.16, 0.8))
        .rounded(8.0)
        .border(1.0)
        .border_color(Color::srgb(0.33, 0.33, 0.33))
        .mb(Val::Px(10.0))
        .add(|ui| {
            // Name and basic info
            ui.ch().header(&info.name);
            ui.ch().label(format!("{} - {}", info.race, info.subrace));
            ui.ch().label(format!("Location: {}", info.location));

            ui.ch().on_spawn_insert(|| {
                (
                    Node {
                        height: Val::Px(1.0),
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::WHITE),
                )
            });

            // Stats or Health brief
            ui.ch().sub_header("Health");

            // Helper to show limb health
            let mut limb_row = |name: &str, value: u8| {
                ui.ch()
                    .on_spawn_insert(|| Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        width: Val::Percent(100.0),
                        ..default()
                    })
                    .add(|ui| {
                        ui.ch().label(name);
                        ui.ch().label(format!("{}", value));
                    });
            };

            limb_row("Head", health.head);
            limb_row("Chest", health.chest);
            limb_row("Stomach", health.stomach);
            limb_row("L Arm", health.left_arm);
            limb_row("R Arm", health.right_arm);
            limb_row("L Leg", health.left_leg);
            limb_row("R Leg", health.right_leg);
        });
}

#[derive(Component)]
pub struct CharacterCard;

impl ImmediateAttach<CapsUi> for CharacterCard {
    type Params = (
        Res<'static, SquadState>,
        Query<'static, 'static, (&'static CharacterInfo, &'static Health)>,
    );

    fn construct(
        ui: &mut Imm<CapsUi>,
        (squad_state, characters): &mut <Self::Params as bevy::ecs::system::SystemParam>::Item<
            '_,
            '_,
        >,
    ) {
        ui.ch()
            .on_spawn_insert(|| Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            })
            .add(|ui| {
                let Some(selected_id) = &squad_state.selected_character else {
                    ui.ch().label("No Character Selected");
                    return;
                };

                let Some(&entity) = squad_state.characters.get(selected_id) else {
                    ui.ch().label("Character Not Found");
                    return;
                };

                if let Ok((info, health)) = characters.get(entity) {
                    render_character_card(ui, info, health);
                } else {
                    ui.ch().label("Character missing data");
                }
            });
    }
}

#[derive(Component)]
pub struct SquadsDisplay;

impl ImmediateAttach<CapsUi> for SquadsDisplay {
    type Params = (
        Res<'static, SquadState>,
        Query<'static, 'static, (&'static CharacterInfo, &'static Health)>,
    );

    fn construct(
        ui: &mut Imm<CapsUi>,
        (squad_state, characters): &mut <Self::Params as bevy::ecs::system::SystemParam>::Item<
            '_,
            '_,
        >,
    ) {
        ui.ch()
            .on_spawn_insert(|| Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            })
            .add(|ui| {
                // Use the stored order
                let squad_ids = &squad_state.squad_order;

                if squad_ids.is_empty() {
                    ui.ch().label("No Squads");
                }

                for &squad_id in squad_ids {
                    ui.ch()
                        .on_spawn_insert(|| Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(10.0),
                            ..default()
                        })
                        .add(|ui| {
                            if let Some(squad) = squad_state.squads.get(&squad_id) {
                                ui.ch().sub_header(&squad.name);

                                if squad.members.is_empty() {
                                    ui.ch().label("Empty Squad");
                                } else {
                                    for char_id in &squad.members {
                                        if let Some(&entity) = squad_state.characters.get(char_id) {
                                            if let Ok((info, health)) = characters.get(entity) {
                                                render_character_card(ui, info, health);
                                            }
                                        }
                                    }
                                }
                            } else {
                                ui.ch().label(format!("Missing Squad {}", squad_id));
                            }
                        });

                    ui.ch().on_spawn_insert(|| {
                        (
                            Node {
                                height: Val::Px(1.0),
                                width: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::WHITE),
                        )
                    });
                }
            });
    }
}
