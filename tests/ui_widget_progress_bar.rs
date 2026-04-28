use bevy::prelude::*;
use shenji::ui::prelude::*;

#[test]
fn half_progress_inner_is_50pct_wide() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        progress_bar(0.5).spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&Node>();
    let widths: Vec<Val> = q.iter(world).map(|n| n.width).collect();
    assert!(
        widths
            .iter()
            .any(|w| matches!(w, Val::Percent(p) if (*p - 50.0).abs() < f32::EPSILON)),
        "inner bar at fraction 0.5 should have width 50%"
    );
}

#[test]
fn over_one_clamps_to_100pct() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        progress_bar(1.5).spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&Node>();
    let widths: Vec<Val> = q.iter(world).map(|n| n.width).collect();
    assert!(
        widths
            .iter()
            .any(|w| matches!(w, Val::Percent(p) if (*p - 100.0).abs() < f32::EPSILON))
    );
}

#[test]
fn negative_clamps_to_0pct() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        progress_bar(-0.2).spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&Node>();
    let widths: Vec<Val> = q.iter(world).map(|n| n.width).collect();
    assert!(
        widths
            .iter()
            .any(|w| matches!(w, Val::Percent(p) if p.abs() < f32::EPSILON))
    );
}
