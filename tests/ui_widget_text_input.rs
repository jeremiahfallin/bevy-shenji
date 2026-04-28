use bevy::prelude::*;
use shenji::ui::prelude::*;
use shenji::ui::widgets::text_input;

#[test]
fn text_input_initial_state() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(text_input::plugin);
    let world = app.world_mut();
    let mut commands = world.commands();
    text_input::text_input("hello").spawn(&mut commands);
    world.flush();
    let mut q = world.query::<&text_input::TextInputState>();
    let s = q.iter(world).next().unwrap();
    assert_eq!(s.value, "hello");
    assert_eq!(s.focused, false);
    assert_eq!(s.cursor, "hello".len());
}

#[test]
fn empty_initial_value() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(text_input::plugin);
    let world = app.world_mut();
    let mut commands = world.commands();
    text_input::text_input("").spawn(&mut commands);
    world.flush();
    let mut q = world.query::<&text_input::TextInputState>();
    let s = q.iter(world).next().unwrap();
    assert_eq!(s.value, "");
    assert_eq!(s.cursor, 0);
}
