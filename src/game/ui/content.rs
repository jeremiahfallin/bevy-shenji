use bevy::prelude::*;
use bevy_immediate::{
    Imm,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};

use crate::game::ui::inspector::{CharacterInspector, InspectorState, InspectorTab};
use crate::game::{character::CharacterInfo, resources::SquadState};
use crate::theme::{prelude::*, scroll::ImmUiScrollExt};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<InspectorState>();
    app.add_plugins((
        BevyImmediateAttachPlugin::<CapsUi, Content>::new(),
        BevyImmediateAttachPlugin::<CapsUi, CharacterInspector>::new(),
    ));
}

#[derive(Component)]
pub struct Content;

impl ImmediateAttach<CapsUi> for Content {
    // Inject the game state resource ('static lifetime is required here)
    type Params = (
        Res<'static, SquadState>,
        Query<'static, 'static, &'static CharacterInfo>,
    );

    fn construct(
        ui: &mut Imm<CapsUi>,
        (state, character_query): &mut <Self::Params as bevy::ecs::system::SystemParam>::Item<
            '_,
            '_,
        >,
    ) {
        ui.ch()
            .flex_col()
            .w_full()
            .h_full()
            .p(Val::Px(20.0))
            .add(|ui| {
                ui.ch().header("Squads");

                let character_text = format!("Number of characters: {}", state.characters.len());
                ui.ch().label(character_text);

                ui.ch().header("Squads");
                ui.ch().flex().flex_col().scrollarea(
                    |n| {
                        n.flex_direction = FlexDirection::Column;
                        n.row_gap = Val::Px(10.0);
                    },
                    |ui| {
                        for squad in state.squads.values() {
                            // Squad Header
                            ui.ch()
                                .label(&squad.name)
                                .text_size(20.0)
                                .text_color(Color::WHITE);

                            // Members List
                            ui.ch().flex_col().pl(Val::Px(10.0)).add(|ui| {
                                for member_id in squad.members.iter() {
                                    if let Some(&entity) = state.characters.get(member_id) {
                                        if let Ok(info) = character_query.get(entity) {
                                            // Make it a BUTTON to select the character
                                            // We need to clone the ID to pass it into the closure
                                            let char_id = info.id.clone();
                                            let char_name = info.name.clone();

                                            ui.ch()
                                                .button()
                                                .on_click_once(
                                                    move |_trigger: On<Pointer<Click>>,
                                                          mut inspector: ResMut<InspectorState>| {
                                                        inspector.selected_character_id =
                                                            Some(char_id.clone());
                                                        // Reset tab to Health when switching chars
                                                        inspector.active_tab = InspectorTab::Health;
                                                    },
                                                )
                                                .style(|n| {
                                                    n.justify_content = JustifyContent::FlexStart
                                                }) // Align text left
                                                .add(|ui| {
                                                    ui.ch()
                                                        .label(char_name)
                                                        .text_color(Color::srgb(0.9, 0.9, 0.9));
                                                });
                                        }
                                    }
                                }
                            });
                        }
                    },
                );

                ui.ch().flex_col().flex_grow().p(Val::Px(10.0)).add(|ui| {
                    ui.ch()
                        .w_full()
                        .h_full()
                        .on_spawn_insert(|| CharacterInspector);
                });
            });
    }
}
