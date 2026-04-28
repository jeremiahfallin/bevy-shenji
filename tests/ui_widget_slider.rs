use bevy::prelude::*;
use shenji::ui::prelude::*;
use shenji::ui::widgets::slider;

#[test]
fn slider_state_initialized_with_value_and_range() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(slider::plugin);
    let world = app.world_mut();
    let mut commands = world.commands();
    slider::slider(0.5, 0.0..=1.0).spawn(&mut commands);
    world.flush();
    let mut q = world.query::<&slider::SliderState>();
    let s = q.iter(world).next().unwrap();
    assert!((s.value - 0.5).abs() < f32::EPSILON);
    assert!((s.min - 0.0).abs() < f32::EPSILON);
    assert!((s.max - 1.0).abs() < f32::EPSILON);
}

#[test]
fn slider_clamps_initial_value_to_range() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(slider::plugin);
    let world = app.world_mut();
    let mut commands = world.commands();
    slider::slider(2.0, 0.0..=1.0).spawn(&mut commands);
    world.flush();
    let mut q = world.query::<&slider::SliderState>();
    let s = q.iter(world).next().unwrap();
    assert!((s.value - 1.0).abs() < f32::EPSILON);
}
