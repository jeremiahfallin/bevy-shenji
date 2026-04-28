use bevy::prelude::*;
use shenji::ui::prelude::*;

#[test]
fn btn_primary_has_primary_500_bg() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        btn_primary("Play").spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&BackgroundColor>();
    let mut found = false;
    for bg in q.iter(world) {
        if bg.0 == PRIMARY_500 {
            found = true;
        }
    }
    assert!(
        found,
        "btn_primary should set BackgroundColor to PRIMARY_500"
    );
}

#[test]
fn btn_ghost_has_no_solid_primary_bg() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        btn_ghost("Cancel").spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&BackgroundColor>();
    for bg in q.iter(world) {
        assert_ne!(bg.0, PRIMARY_500, "ghost variant must not use primary fill");
    }
}
