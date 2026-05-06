//! Characters view — scrollable wide table of every character with their
//! skills.
//!
//! Acceptance criterion (Phase B Task 11): horizontal scroll reaches the
//! rightmost skill column. The fix is structural: every row has the same
//! fixed total width (sum of column widths), so the horizontal scroll
//! container's content width is well-defined and the scrollbar can reach
//! the end. The pre-migration layout fed an immediate-mode `scrollarea`
//! children with no fixed width, leaving the scrollable content shorter
//! than the visible row content.

use bevy::prelude::*;
use bevy_declarative::prelude::px;

use crate::game::character::{CharacterInfo, Skills};
use crate::screens::Screen;
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(populate_characters_on_add);
    app.add_systems(
        Update,
        refresh_characters.run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component)]
pub struct CharactersView;

const COL_NAME_WIDTH: f32 = 150.0;
const COL_RACE_WIDTH: f32 = 100.0;
const COL_STATUS_WIDTH: f32 = 120.0;
const COL_SKILL_WIDTH: f32 = 110.0;

fn populate_characters_on_add(
    add: On<Add, CharactersView>,
    mut commands: Commands,
    character_query: Query<(&CharacterInfo, &Skills)>,
) {
    spawn_characters_children(&mut commands, add.entity, &character_query);
}

fn refresh_characters(
    mut commands: Commands,
    view_q: Query<Entity, With<CharactersView>>,
    character_query: Query<(&CharacterInfo, &Skills)>,
    changed_q: Query<(), Or<(Changed<CharacterInfo>, Changed<Skills>)>>,
    mut last_count: Local<usize>,
) {
    let count = character_query.iter().count();
    let any_changed = !changed_q.is_empty() || count != *last_count;
    if !any_changed {
        return;
    }
    *last_count = count;
    for view in &view_q {
        commands.entity(view).despawn_related::<Children>();
        spawn_characters_children(&mut commands, view, &character_query);
    }
}

fn spawn_characters_children(
    commands: &mut Commands,
    view: Entity,
    character_query: &Query<(&CharacterInfo, &Skills)>,
) {
    let total_width = total_row_width();

    // Header row.
    let mut header = div()
        .flex()
        .row()
        .w(Val::Px(total_width))
        .pad_x(px(SPACE_2))
        .py(px(SPACE_2))
        .bg(GRAY_800)
        .child(header_cell("Name", COL_NAME_WIDTH))
        .child(header_cell("Race", COL_RACE_WIDTH))
        .child(header_cell("Status", COL_STATUS_WIDTH));
    for (skill, _) in Skills::default().iter() {
        header = header.child(header_cell(skill, COL_SKILL_WIDTH));
    }

    // Body: one row per character.
    let mut body = div().col();
    let mut idx: usize = 0;
    for (character, skills) in character_query.iter() {
        let bg = if idx % 2 == 0 { GRAY_900 } else { GRAY_800 };
        let mut row = div()
            .flex()
            .row()
            .w(Val::Px(total_width))
            .pad_x(px(SPACE_2))
            .py(px(SPACE_1))
            .bg(bg)
            .child(cell(label(character.name.clone()), COL_NAME_WIDTH))
            .child(cell(label(character.race.clone()), COL_RACE_WIDTH))
            .child(cell_div(
                div().child(badge("Active").bg(SUCCESS_600)),
                COL_STATUS_WIDTH,
            ));
        for (_, level) in skills.iter() {
            row = row.child(cell(label(format!("{}", level)), COL_SKILL_WIDTH));
        }
        body = body.child(row);
        idx += 1;
    }

    let table = div()
        .col()
        .w(Val::Px(total_width))
        .child(header)
        .child(body);

    let scroll = scroll_view()
        .horizontal()
        .vertical(true)
        .child(table);

    let scroll_entity = scroll.spawn(commands).id();
    commands.entity(scroll_entity).insert((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            min_width: Val::Px(0.0),
            min_height: Val::Px(0.0),
            overflow: Overflow::scroll(),
            ..default()
        },
    ));

    commands.entity(view).add_child(scroll_entity);
}

fn header_cell(s: &str, width: f32) -> Div {
    div()
        .flex()
        .row()
        .items_center()
        .w(Val::Px(width))
        .pad_x(px(SPACE_1))
        .child(text(s).color(Color::WHITE))
}

fn cell(content: TextEl, width: f32) -> Div {
    div()
        .flex()
        .row()
        .items_center()
        .w(Val::Px(width))
        .pad_x(px(SPACE_1))
        .child(content)
}

fn cell_div(content: Div, width: f32) -> Div {
    div()
        .flex()
        .row()
        .items_center()
        .w(Val::Px(width))
        .pad_x(px(SPACE_1))
        .child(content)
}

fn total_row_width() -> f32 {
    let skill_count = Skills::default().iter().count() as f32;
    COL_NAME_WIDTH + COL_RACE_WIDTH + COL_STATUS_WIDTH + COL_SKILL_WIDTH * skill_count
}
