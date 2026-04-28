use bevy::prelude::*;
use shenji::ui::prelude::*;

#[test]
fn badge_renders_label_text() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        badge("New").spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&Text>();
    let strings: Vec<String> = q.iter(world).map(|t| t.0.clone()).collect();
    assert!(strings.iter().any(|s| s == "New"));
}
