use crate::game::character::{Equipment, Health, Inventory, Skills};
use crate::game::resources::SquadState;
use crate::theme::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_immediate::{Imm, attach::ImmediateAttach, ui::CapsUi};

// FIX: Derive 'Resource' so it can be used in Res<...>
#[derive(Resource, Default)]
pub struct InspectorState {
    pub selected_character_id: Option<String>,
    pub active_tab: InspectorTab,
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum InspectorTab {
    #[default]
    Health,
    Equipment,
    Skills,
    Inventory,
}

// The UI Widget Component
#[derive(Component)]
pub struct CharacterInspector;

// Custom SystemParam to handle multiple resources with correct lifetimes
#[derive(SystemParam)]
pub struct InspectorParams<'w> {
    pub squad_state: Res<'w, SquadState>,
    pub inspector_state: Res<'w, InspectorState>,
}

impl ImmediateAttach<CapsUi> for CharacterInspector {
    // We inject the Global Game State (SquadState), Local UI State (InspectorState), and Character Components
    type Params = (
        Res<'static, SquadState>,
        Res<'static, InspectorState>,
        Query<
            'static,
            'static,
            (
                &'static Health,
                &'static Skills,
                &'static Equipment,
                &'static Inventory,
            ),
        >,
    );

    fn construct(
        ui: &mut Imm<CapsUi>,
        params: &mut (
            Res<SquadState>,
            Res<InspectorState>,
            Query<(&Health, &Skills, &Equipment, &Inventory)>,
        ),
    ) {
        let (squad_state, inspector_state, char_query) = (&params.0, &params.1, &params.2);

        // 1. Validation: Do we have a selected character?
        let Some(char_entity) = &inspector_state.selected_character_id else {
            ui.ch().label("Select a character to inspect").style(|n| {
                n.align_self = AlignSelf::Center;
                n.margin = UiRect::all(Val::Auto);
            });
            return;
        };

        // 2. Fetch Data: Does that character exist?
        let Some(entity) = squad_state.characters.get(char_entity) else {
            ui.ch().label("Character not found");
            return;
        };

        let Ok((health, skills, equipment, inventory)) = char_query.get(*entity) else {
            ui.ch().label("Character missing data");
            return;
        };

        // 3. Render Inspector
        ui.ch()
            .flex_col()
            .flex_grow()
            .apply(style_panel_central)
            .p(Val::Px(15.0))
            .add(|ui| match inspector_state.active_tab {
                InspectorTab::Health => {
                    // FIX: Iterate over all limbs
                    for (part, hp) in health.iter() {
                        ui.ch()
                            .flex_row()
                            .justify_between()
                            .w_full()
                            .mb(Val::Px(5.0))
                            .add(|ui| {
                                ui.ch().label(part).text_color(Color::srgb(0.8, 0.8, 0.8));
                                // Color scaling
                                let color = if hp > 80 {
                                    Color::srgb(0.0, 1.0, 0.0)
                                } else if hp > 40 {
                                    Color::srgb(1.0, 1.0, 0.0)
                                } else {
                                    Color::srgb(1.0, 0.0, 0.0)
                                };
                                ui.ch().label(format!("{}", hp)).text_color(color);
                            });
                    }
                    ui.ch().label("Status").mt(Val::Px(10.0)).mb(Val::Px(5.0));
                    ui.ch().label(format!("Hunger: {}", health.hunger));
                }
                InspectorTab::Equipment => {
                    // FIX: Use the display_equip_slot helper
                    let eq = equipment;
                    display_equip_slot(ui, "Head", &eq.head);
                    display_equip_slot(ui, "Chest", &eq.chest);
                    display_equip_slot(ui, "Legs", &eq.legs);
                    display_equip_slot(ui, "Feet", &eq.feet);
                    display_equip_slot(ui, "Hands", &eq.hands);
                    display_equip_slot(ui, "Main Hand", &eq.main_hand);
                }
                InspectorTab::Skills => {
                    // (This part looked fine in your upload)
                    ui.ch().scrollarea(
                        |n| {
                            n.flex_direction = FlexDirection::Column;
                        },
                        |ui| {
                            for (skill, xp) in skills.iter() {
                                ui.ch()
                                    .flex_row()
                                    .justify_between()
                                    .w_full()
                                    .mb(Val::Px(2.0))
                                    .add(|ui| {
                                        ui.ch().label(skill).text_color(Color::srgb(0.8, 0.8, 0.8));
                                        ui.ch()
                                            .label(format!("{}", xp_to_level(xp.into())))
                                            .text_color(Color::WHITE);
                                    });
                            }
                        },
                    );
                }
                InspectorTab::Inventory => {
                    // (This part looked fine in your upload)
                    if inventory.items.is_empty() {
                        ui.ch()
                            .label("Empty")
                            .text_color(Color::srgb(0.5, 0.5, 0.5));
                    } else {
                        for (item, count) in &inventory.items {
                            ui.ch().flex_row().justify_between().w_full().add(|ui| {
                                ui.ch().label(format!("{}: {}", item, count));
                                ui.ch().button().add(|ui| {
                                    ui.ch().label("Drop");
                                });
                            });
                        }
                    }
                }
            });
    }
}

// Helper: Tab Button
fn tab_button(ui: &mut Imm<CapsUi>, text: &str, tab: InspectorTab, active: InspectorTab) {
    let is_active = tab == active;
    ui.ch()
        .button()
        .on_click_once(
            move |_trigger: On<Pointer<Click>>, mut state: ResMut<InspectorState>| {
                state.active_tab = tab;
            },
        )
        .style(move |n| {
            // Underline effect for active tab
            n.border = UiRect::bottom(Val::Px(if is_active { 2.0 } else { 0.0 }));
        })
        .bg(Color::NONE) // Transparent background like a tab
        .add(|ui| {
            ui.ch().label(text).text_color(if is_active {
                Color::WHITE
            } else {
                Color::srgb(0.6, 0.6, 0.6)
            });
        });
}

// Helper: Math
fn xp_to_level(xp: u32) -> u32 {
    (xp as f32).sqrt() as u32
}

fn display_equip_slot(ui: &mut Imm<CapsUi>, slot_name: &str, item: &Option<String>) {
    ui.ch()
        .flex_row()
        .justify_between()
        .w_full()
        .mb(Val::Px(5.0))
        .add(|ui| {
            ui.ch()
                .label(slot_name)
                .text_color(Color::srgb(0.7, 0.7, 0.7));
            match item {
                Some(name) => {
                    ui.ch().label(name).text_color(Color::WHITE);
                }
                None => {
                    ui.ch()
                        .label("Empty")
                        .text_color(Color::srgb(0.3, 0.3, 0.3));
                }
            }
        });
}
