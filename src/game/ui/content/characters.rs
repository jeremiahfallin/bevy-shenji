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

/// Marker for the Characters scroll container (temporary debug)
#[derive(Component)]
pub struct CharactersScrollDebug;

pub fn debug_characters_scroll(
    q: Query<(Entity, &Node, &ComputedNode, Option<&Children>), With<CharactersScrollDebug>>,
    child_q: Query<(&Node, &ComputedNode)>,
    mut timer: Local<f32>,
    time: Res<Time>,
) {
    *timer += time.delta_secs();
    if *timer < 2.0 {
        return; // only log every 2 seconds
    }
    *timer = 0.0;

    for (entity, node, computed, children) in q.iter() {
        let size = computed.size();
        info!(
            "[CharScroll] container {:?}: computed_size=({:.0}, {:.0}) overflow=({:?}, {:?})",
            entity, size.x, size.y, node.overflow.x, node.overflow.y,
        );
        if let Some(children) = children {
            for child in children.iter() {
                if let Ok((child_node, child_computed)) = child_q.get(child) {
                    let cs = child_computed.size();
                    info!(
                        "[CharScroll]   child {:?}: computed_size=({:.0}, {:.0}) display={:?} width={:?} height={:?}",
                        child, cs.x, cs.y, child_node.display, child_node.width, child_node.height,
                    );
                }
            }
        }
    }
}

impl ImmediateAttach<CapsUi> for CharactersView {
    type Params = (Query<'static, 'static, (Entity, &'static CharacterInfo, &'static Skills)>,);

    fn construct(ui: &mut Imm<CapsUi>, params: &mut (Query<(Entity, &CharacterInfo, &Skills)>,)) {
        let (character_query,) = params;

        // Positioned ancestor — fills the available space via percentages.
        // The absolute child below is taken out of flow so the wide table
        // can't push parent containers wider.
        ui.ch()
            .style(|n: &mut Node| {
                n.width = Val::Percent(100.0);
                n.height = Val::Percent(100.0);
                n.position_type = PositionType::Relative;
            })
            .add(|ui| {
                // Viewport: absolute-positioned, clips content.
                ui.ch()
                    .style(|n: &mut Node| {
                        n.position_type = PositionType::Absolute;
                        n.left = Val::Px(0.0);
                        n.top = Val::Px(0.0);
                        n.right = Val::Px(0.0);
                        n.bottom = Val::Px(0.0);
                        n.display = Display::Flex;
                        n.flex_direction = FlexDirection::Column;
                        n.align_items = AlignItems::Start;
                        n.overflow = Overflow::clip();
                    })
                    .on_spawn_insert(|| CharactersScrollDebug)
                    .add(|ui| {
                        // Scrollable content wrapper — uses the project's custom
                        // scroll system (UiScrollPosition + ScrollableContent).
                        // min_height ensures the wrapper covers the full viewport
                        // so pointer events are received everywhere.
                        ui.ch()
                            .style(|n: &mut Node| {
                                n.min_height = Val::Percent(100.0);
                            })
                            .on_spawn_insert(|| {
                                (UiScrollPosition::default(), ScrollableContent)
                            })
                            .add(|ui| {
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
                                                    ui.ch().label(
                                                        format!("{}", character.race.clone()),
                                                    );
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
                            });
                    });
            });
    }
}
