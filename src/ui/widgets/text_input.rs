//! TextInput widget — click to focus, captures keyboard while focused.

use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::ecs::system::EntityCommands;
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::picking::events::{Click, Pointer};
use bevy::prelude::*;
use bevy_declarative::element::Element;
use bevy_declarative::style::styled::Styled;

use crate::ui::tokens::palette::*;

const INPUT_WIDTH: f32 = 160.0;
const INPUT_HEIGHT: f32 = 28.0;
const INPUT_PADDING: f32 = 6.0;

/// Emitted when a text input's value changes (typing, backspace).
#[derive(Message, Debug, Clone)]
pub struct TextInputChanged {
    pub entity: Entity,
    pub value: String,
}

/// Emitted when the user presses Enter while focused.
#[derive(Message, Debug, Clone)]
pub struct TextInputSubmitted {
    pub entity: Entity,
    pub value: String,
}

/// State component attached to the text input root.
#[derive(Component, Debug, Clone)]
pub struct TextInputState {
    pub value: String,
    pub cursor: usize,
    pub focused: bool,
}

pub struct TextInput {
    node: Node,
    initial: String,
}

pub fn text_input(initial: &str) -> TextInput {
    TextInput {
        node: Node {
            width: Val::Px(INPUT_WIDTH),
            height: Val::Px(INPUT_HEIGHT),
            padding: UiRect::all(Val::Px(INPUT_PADDING)),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            ..default()
        },
        initial: initial.to_string(),
    }
}

impl TextInput {
    fn build_state(&self) -> TextInputState {
        TextInputState {
            cursor: self.initial.len(),
            value: self.initial.clone(),
            focused: false,
        }
    }

    pub fn spawn<'a>(self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let state = self.build_state();
        let initial = self.initial.clone();
        let mut ec = commands.spawn((self.node, BackgroundColor(SURFACE_INSET), state));
        ec.with_children(|inner| {
            inner.spawn((
                Text::new(initial),
                TextFont::default(),
                TextColor(TEXT_PRIMARY),
            ));
        });
        ec
    }
}

impl Styled for TextInput {
    fn style_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl Element for TextInput {
    fn spawn_with_parent(self: Box<Self>, parent: &mut ChildSpawnerCommands) {
        let state = self.build_state();
        let initial = self.initial.clone();
        parent
            .spawn((self.node, BackgroundColor(SURFACE_INSET), state))
            .with_children(|inner| {
                inner.spawn((
                    Text::new(initial),
                    TextFont::default(),
                    TextColor(TEXT_PRIMARY),
                ));
            });
    }
}

pub fn plugin(app: &mut App) {
    app.add_message::<TextInputChanged>();
    app.add_message::<TextInputSubmitted>();
    app.add_observer(on_text_input_click);
    app.add_systems(Update, keyboard_input_system);
}

fn on_text_input_click(click: On<Pointer<Click>>, mut q: Query<(Entity, &mut TextInputState)>) {
    let target = click.entity;
    // Only act if the click hit a TextInput entity.
    if q.get(target).is_err() {
        return;
    }
    for (entity, mut state) in q.iter_mut() {
        let want_focus = entity == target;
        if state.focused != want_focus {
            state.focused = want_focus;
        }
    }
}

fn keyboard_input_system(
    mut keys: MessageReader<KeyboardInput>,
    mut q: Query<(Entity, &mut TextInputState)>,
    mut changed: MessageWriter<TextInputChanged>,
    mut submitted: MessageWriter<TextInputSubmitted>,
) {
    for ev in keys.read() {
        if ev.state != ButtonState::Pressed {
            continue;
        }
        // Find the focused input.
        let Some((entity, mut state)) = q.iter_mut().find(|(_, s)| s.focused) else {
            continue;
        };
        match &ev.logical_key {
            Key::Character(s) => {
                let s = s.to_string();
                let cursor = state.cursor.min(state.value.len());
                state.value.insert_str(cursor, &s);
                state.cursor = cursor + s.len();
                changed.write(TextInputChanged {
                    entity,
                    value: state.value.clone(),
                });
            }
            Key::Space => {
                let cursor = state.cursor.min(state.value.len());
                state.value.insert(cursor, ' ');
                state.cursor = cursor + 1;
                changed.write(TextInputChanged {
                    entity,
                    value: state.value.clone(),
                });
            }
            Key::Backspace => {
                if state.cursor > 0 && !state.value.is_empty() {
                    let new_cursor = state.cursor - 1;
                    state.value.remove(new_cursor);
                    state.cursor = new_cursor;
                    changed.write(TextInputChanged {
                        entity,
                        value: state.value.clone(),
                    });
                }
            }
            Key::Enter => {
                submitted.write(TextInputSubmitted {
                    entity,
                    value: state.value.clone(),
                });
            }
            _ => {}
        }
    }
}
