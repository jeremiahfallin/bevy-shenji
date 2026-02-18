use crate::game::character::{CharacterInfo, CharacterLocation};
use crate::game::location::{LocationInfo, LocationResources, LocationType};
use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::{Imm, attach::ImmediateAttach, ui::CapsUi};

#[derive(Component)]
pub struct LocationsView;

impl ImmediateAttach<CapsUi> for LocationsView {
    type Params = (
        Query<
            'static,
            'static,
            (&'static LocationInfo, &'static LocationResources),
        >,
        Query<'static, 'static, (&'static CharacterInfo, &'static CharacterLocation)>,
    );

    fn construct(
        ui: &mut Imm<CapsUi>,
        (location_query, character_query): &mut (
            Query<(&LocationInfo, &LocationResources)>,
            Query<(&CharacterInfo, &CharacterLocation)>,
        ),
    ) {
        ui.ch().header("Locations");

        ui.ch()
            .flex_col()
            .w_full()
            .p(Val::Px(10.0))
            .row_gap(10.0)
            .add(|ui| {
                let mut locations: Vec<_> = location_query
                    .iter()
                    .filter(|(info, _)| info.discovered)
                    .collect();
                locations.sort_by_key(|(info, _)| info.distance);

                if locations.is_empty() {
                    ui.ch()
                        .label("No locations discovered yet")
                        .text_color(Color::srgb(0.5, 0.5, 0.5));
                    return;
                }

                for (info, resources) in &locations {
                    ui.ch()
                        .flex_col()
                        .w_full()
                        .p(Val::Px(8.0))
                        .bg(GRAY_800)
                        .mb(Val::Px(4.0))
                        .add(|ui| {
                            // Name and type
                            ui.ch()
                                .flex_row()
                                .justify_between()
                                .w_full()
                                .mb(Val::Px(4.0))
                                .add(|ui| {
                                    ui.ch()
                                        .label(&info.name)
                                        .font_bold()
                                        .text_color(Color::WHITE);

                                    let type_str = match info.loc_type {
                                        LocationType::Base => "Base",
                                        LocationType::Mine => "Mine",
                                        LocationType::Forest => "Forest",
                                        LocationType::Ruins => "Ruins",
                                        LocationType::City => "City",
                                        LocationType::Wilderness => "Wilderness",
                                    };
                                    ui.ch()
                                        .label(type_str)
                                        .text_color(Color::srgb(0.6, 0.6, 0.6));
                                });

                            // Distance
                            if info.distance > 0 {
                                ui.ch()
                                    .label(format!("Distance: {}", info.distance))
                                    .text_color(Color::srgb(0.7, 0.7, 0.7))
                                    .mb(Val::Px(2.0));
                            }

                            // Resources (if applicable)
                            if !resources.resource_type.is_empty() && resources.capacity > 0 {
                                ui.ch()
                                    .flex_row()
                                    .w_full()
                                    .mb(Val::Px(2.0))
                                    .add(|ui| {
                                        ui.ch()
                                            .label(format!(
                                                "{}: {}/{}",
                                                resources.resource_type,
                                                resources.current_amount,
                                                resources.capacity
                                            ))
                                            .text_color(Color::srgb(0.8, 0.8, 0.5));
                                        ui.ch()
                                            .label(format!(
                                                " (yield: {}/tick)",
                                                resources.yield_rate
                                            ))
                                            .text_color(Color::srgb(0.5, 0.7, 0.5));
                                    });
                            }

                            // Characters at this location
                            let chars_here: Vec<_> = character_query
                                .iter()
                                .filter(|(_, loc)| loc.location_id == info.id)
                                .map(|(ci, _)| ci.name.clone())
                                .collect();

                            if !chars_here.is_empty() {
                                ui.ch()
                                    .label(format!("Characters: {}", chars_here.join(", ")))
                                    .text_color(Color::srgb(0.5, 0.8, 0.5));
                            }
                        });
                }
            });
    }
}
