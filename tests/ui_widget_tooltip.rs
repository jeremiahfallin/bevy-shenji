use bevy::prelude::*;
use shenji::ui::prelude::*;

#[test]
fn tooltip_renders_body_text() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        tooltip("More info").spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&Text>();
    let strings: Vec<String> = q.iter(world).map(|t| t.0.clone()).collect();
    assert!(strings.iter().any(|s| s == "More info"));
}

#[test]
fn tooltip_uses_dark_surface() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        tooltip("More info").spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&BackgroundColor>();
    assert!(q.iter(world).any(|bg| bg.0 != Color::NONE));
}
