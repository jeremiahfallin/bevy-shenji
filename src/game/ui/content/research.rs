//! Research view — read-only horizontal-scroll grid of tech-level columns,
//! each containing research cards colored by their state (unlocked /
//! in-progress / available / locked).
//!
//! Spawned once on `On<Add, ResearchView>`; rebuilt whenever any of
//! `GameData`, `ResearchState`, or `BaseInventory` changes. The pre-migration
//! plan flagged Dropdown + Modal + Tooltip-hover widget builds for this
//! task, but the actual screen is a pure display — no interactive elements
//! exist today. Defer those widget builds until a screen actually needs them.

use bevy::prelude::*;
use bevy_declarative::prelude::px;

use crate::game::{data::GameData, research::ResearchState, resources::BaseInventory};
use crate::screens::Screen;
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(populate_research_on_add);
    app.add_systems(Update, refresh_research.run_if(in_state(Screen::Gameplay)));
}

#[derive(Component)]
pub struct ResearchView;

fn populate_research_on_add(
    add: On<Add, ResearchView>,
    mut commands: Commands,
    game_data: Res<GameData>,
    research_state: Res<ResearchState>,
    base_inventory: Res<BaseInventory>,
) {
    spawn_research_children(
        &mut commands,
        add.entity,
        &game_data,
        &research_state,
        &base_inventory,
    );
}

fn refresh_research(
    mut commands: Commands,
    view_q: Query<Entity, With<ResearchView>>,
    game_data: Res<GameData>,
    research_state: Res<ResearchState>,
    base_inventory: Res<BaseInventory>,
) {
    let any_changed =
        game_data.is_changed() || research_state.is_changed() || base_inventory.is_changed();
    if !any_changed {
        return;
    }
    for view in &view_q {
        commands.entity(view).despawn_related::<Children>();
        spawn_research_children(
            &mut commands,
            view,
            &game_data,
            &research_state,
            &base_inventory,
        );
    }
}

fn spawn_research_children(
    commands: &mut Commands,
    view: Entity,
    game_data: &GameData,
    research_state: &ResearchState,
    base_inventory: &BaseInventory,
) {
    let mut root = div().col().w_full().h_full().child(heading_2("Research Tree"));

    // Optional "currently researching" banner.
    if let Some(current_id) = &research_state.current_research {
        if let Some(def) = game_data.get_research(current_id) {
            let progress_frac = if def.time > 0 {
                research_state.research_progress as f32 / def.time as f32
            } else {
                1.0
            };
            root = root.child(
                div()
                    .flex()
                    .row()
                    .w_full()
                    .p(px(SPACE_2))
                    .mb(px(SPACE_2_5))
                    .rounded(Val::Px(4.0))
                    .bg(Color::srgb(0.15, 0.15, 0.3))
                    .child(label(format!(
                        "Researching: {} ({:.0}%)",
                        def.name,
                        progress_frac * 100.0
                    ))),
            );
        }
    }

    // Determine the maximum tech_level present in data.
    let max_tech_level = game_data
        .research
        .values()
        .map(|r| r.tech_level)
        .max()
        .unwrap_or(1);

    // Horizontal-scrolling row of fixed-width tech-level columns.
    let mut columns_row = div().flex().row().w_full().h_full();
    for tech_level in 1..=max_tech_level {
        let mut column = div()
            .col()
            .w(Val::Px(280.0))
            .h_full()
            .p(px(SPACE_2_5))
            .child(label(format!("Tech Level {}", tech_level)).mb(px(SPACE_2_5)));

        let mut research_in_level: Vec<_> = game_data
            .research
            .values()
            .filter(|r| r.tech_level == tech_level)
            .collect();
        research_in_level.sort_by_key(|r| &r.name);

        for research in research_in_level {
            let is_unlocked = research_state.is_unlocked(&research.id);
            let can_research = research_state.can_research(&research.id, game_data);
            let can_afford = research
                .cost
                .iter()
                .all(|(item_id, &amount)| base_inventory.count(item_id) >= amount);
            let is_current = research_state.current_research.as_deref()
                == Some(research.id.as_str());

            let card_bg = if is_unlocked {
                Color::srgb(0.2, 0.5, 0.2)
            } else if is_current {
                Color::srgb(0.2, 0.2, 0.5)
            } else if can_research {
                Color::srgb(0.3, 0.3, 0.3)
            } else {
                Color::srgb(0.1, 0.1, 0.1)
            };

            let mut card = div()
                .col()
                .p(px(SPACE_2_5))
                .mb(px(SPACE_2_5))
                .rounded(Val::Px(4.0))
                .bg(card_bg)
                .child(text(research.name.clone()).color(Color::WHITE))
                .child(
                    text(format!("Type: {}", research.research_type))
                        .font_size(11.0)
                        .color(Color::srgb(0.7, 0.7, 0.7)),
                );

            if !is_unlocked {
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
                    card = card.child(
                        text(format!("Cost: {}", cost_str.join(", ")))
                            .font_size(11.0)
                            .color(if can_afford {
                                Color::WHITE
                            } else {
                                Color::srgb(0.8, 0.2, 0.2)
                            }),
                    );
                }
                card = card.child(
                    text(format!("Time: {} ticks", research.time))
                        .font_size(11.0)
                        .color(Color::srgb(0.7, 0.7, 0.7)),
                );
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
                    card = card.child(
                        text(format!("Requires: {}", prereq_names.join(", ")))
                            .font_size(10.0)
                            .color(Color::srgb(0.6, 0.6, 0.8)),
                    );
                }
                let (status_text, status_color) = if is_current {
                    ("In Progress...", Color::srgb(0.4, 0.4, 0.9))
                } else if can_research {
                    if can_afford {
                        ("Available", Color::srgb(0.2, 0.8, 0.2))
                    } else {
                        ("Need Resources", Color::srgb(0.8, 0.6, 0.2))
                    }
                } else {
                    ("Locked", Color::srgb(0.8, 0.2, 0.2))
                };
                card = card.child(text(status_text).font_size(10.0).color(status_color));
            } else {
                card = card.child(
                    text("Completed")
                        .font_size(10.0)
                        .color(Color::srgb(0.2, 0.8, 0.2)),
                );
            }

            column = column.child(card);
        }
        columns_row = columns_row.child(column);
    }

    let scroll = scroll_view().horizontal().vertical(false).child(columns_row);
    root = root.child(scroll);

    let root_entity = root.spawn(commands).id();
    commands.entity(view).add_child(root_entity);
}
