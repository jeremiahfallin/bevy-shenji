use bevy::prelude::*;
use shenji::ui::prelude::*;

#[test]
fn horizontal_divider_has_1px_height() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        divider().spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&Node>();
    let mut found = false;
    for n in q.iter(world) {
        if matches!(n.height, Val::Px(h) if (h - 1.0).abs() < f32::EPSILON) {
            found = true;
        }
    }
    assert!(found, "horizontal divider should be 1px tall");
}

#[test]
fn vertical_divider_has_1px_width() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        divider_vertical().spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&Node>();
    let mut found = false;
    for n in q.iter(world) {
        if matches!(n.width, Val::Px(w) if (w - 1.0).abs() < f32::EPSILON) {
            found = true;
        }
    }
    assert!(found, "vertical divider should be 1px wide");
}
