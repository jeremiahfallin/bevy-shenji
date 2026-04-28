use bevy::prelude::*;
use shenji::ui::prelude::*;

#[test]
fn horizontal_sets_overflow_x_scroll() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        scroll_view()
            .horizontal()
            .vertical(false)
            .spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&Node>();
    let mut found = false;
    for n in q.iter(world) {
        if n.overflow.x == OverflowAxis::Scroll && n.overflow.y != OverflowAxis::Scroll {
            found = true;
        }
    }
    assert!(
        found,
        "horizontal-only ScrollView should have overflow.x=Scroll, overflow.y!=Scroll"
    );
}

#[test]
fn vertical_default_sets_overflow_y_scroll() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        scroll_view().spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&Node>();
    let mut found = false;
    for n in q.iter(world) {
        if n.overflow.y == OverflowAxis::Scroll {
            found = true;
        }
    }
    assert!(found, "default ScrollView should have overflow.y=Scroll");
}

#[test]
fn scroll_position_inserted_on_root() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        scroll_view().spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&ScrollPosition>();
    assert!(
        q.iter(world).count() > 0,
        "ScrollView should insert ScrollPosition on its root"
    );
}

#[test]
fn child_spawns_inside() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        scroll_view().child(text("inside")).spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&Text>();
    let strings: Vec<String> = q.iter(world).map(|t| t.0.clone()).collect();
    assert!(strings.iter().any(|s| s == "inside"));
}
