use crate::{
    game::{
        character::CharacterInfo,
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
    type Params = (Query<'static, 'static, (Entity, &'static CharacterInfo)>,);

    fn construct(ui: &mut Imm<CapsUi>, params: &mut (Query<(Entity, &CharacterInfo)>,)) {
        let (character_query,) = params;
        ui.ch().header("Characters");

        ui.ch().flex_col().w_full().p(Val::Px(SPACE_2_5)).add(|ui| {
            for (entity, info) in character_query.iter() {
                let char_id = info.id.clone();
                let char_name = info.name.clone();

                ui.ch()
                    .button()
                    .w_full()
                    .mb(Val::Px(SPACE_1))
                    .p(Val::Px(SPACE_2_5))
                    // Standard Left Click (Selection)
                    .on_click_once(
                        move |trigger: On<Pointer<Click>>,
                              mut inspector: ResMut<InspectorState>| {
                            if trigger.event().button == PointerButton::Primary {
                                inspector.selected_character_id = Some(char_id.clone());
                            }
                        },
                    )
                    // Context Menu (Right Click) - No conflict now!
                    .context_menu(ContextMenuType::Character, entity)
                    .add(|ui| {
                        ui.ch().label(char_name);
                    });
            }
        });
    }
}
