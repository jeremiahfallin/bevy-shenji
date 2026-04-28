use bevy::prelude::*;
use shenji::ui::prelude::*;
use shenji::ui::widgets::tabs;

#[test]
fn tabs_state_initialized_with_active_index() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(tabs::plugin);
    let world = app.world_mut();
    let mut commands = world.commands();
    tabs::tabs(0)
        .tab("A", text("a content"))
        .tab("B", text("b content"))
        .spawn(&mut commands);
    world.flush();
    let mut q = world.query::<&tabs::TabsState>();
    let s = q.iter(world).next().unwrap();
    assert_eq!(s.active, 0);
    assert_eq!(s.count, 2);
}

#[test]
fn tabs_active_clamps_to_count() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(tabs::plugin);
    let world = app.world_mut();
    let mut commands = world.commands();
    tabs::tabs(99).tab("A", text("a")).spawn(&mut commands);
    world.flush();
    let mut q = world.query::<&tabs::TabsState>();
    let s = q.iter(world).next().unwrap();
    assert_eq!(s.active, 0);
}
