use crate::{
    game::{
        character::CharacterInfo,
        character::Skills,
        ui::{
            context_menu::{ContextMenuType, ImmUiContextMenuExt},
            inspector::InspectorState,
        },
    },
    theme::prelude::*,
};
use bevy::prelude::*;
use bevy_immediate::{Imm, attach::ImmediateAttach, ui::CapsUi};

#[derive(Component)]
pub struct CharactersView;

impl ImmediateAttach<CapsUi> for CharactersView {
    type Params = (Query<'static, 'static, (Entity, &'static CharacterInfo, &'static Skills)>,);

    fn construct(ui: &mut Imm<CapsUi>, params: &mut (Query<(Entity, &CharacterInfo, &Skills)>,)) {
        let (character_query,) = params;

        ui.ch()
            .flex_col()
            .flex_1()
            .h_full()
            .overflow_clip()
            .add(|ui| {
                ui.ch().flex_1().scrollarea(
                    |n| {
                        n.display = Display::Flex;
                        n.flex_direction = FlexDirection::Column;
                        n.align_items = AlignItems::Start;
                    },
                    |ui| {
                        let mut table = Table::new()
                            .column(Column::px(150.0))
                            .column(Column::px(100.0))
                            .column(Column::auto());

                        for _ in Skills::default().iter() {
                            table = table.column(Column::auto());
                        }

                        table.striped(true).render(ui, |table| {
                            table.thead(|row| {
                                row.th(|ui| {
                                    ui.ch().label("Name");
                                });
                                row.th(|ui| {
                                    ui.ch().label("Race");
                                });
                                row.th(|ui| {
                                    ui.ch().label("Status");
                                });
                                for (skill, _) in Skills::default().iter() {
                                    row.th(|ui| {
                                        ui.ch().label(skill);
                                    });
                                }
                            });
                            table.tbody(|body| {
                                for (_, character, skills) in character_query.iter() {
                                    body.tr(|row| {
                                        row.td(|ui| {
                                            ui.ch().label(character.name.clone());
                                        });
                                        row.td(|ui| {
                                            ui.ch().label(format!("{}", character.race.clone()));
                                        });
                                        row.td(|ui| {
                                            ui.ch()
                                                .badge("Active")
                                                .badge_variant(BadgeVariant::Success);
                                        });
                                        for (_, level) in skills.iter() {
                                            row.td(|ui| {
                                                ui.ch().label(format!("{}", level));
                                            });
                                        }
                                    });
                                }
                            });
                        });
                    },
                );
            });
    }
}
