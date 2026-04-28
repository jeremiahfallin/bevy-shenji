use bevy::prelude::*;
use shenji::ui::prelude::*;

#[test]
fn panel_uses_surface_raised() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        panel().child(text("hi")).spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&BackgroundColor>();
    assert!(q.iter(world).any(|bg| bg.0 == SURFACE_RAISED));
}

#[test]
fn card_uses_surface_raised() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        card().child(text("c")).spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&BackgroundColor>();
    assert!(q.iter(world).any(|bg| bg.0 == SURFACE_RAISED));
}

#[test]
fn surface_uses_surface_base() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        surface().child(text("s")).spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&BackgroundColor>();
    assert!(q.iter(world).any(|bg| bg.0 == SURFACE_BASE));
}
