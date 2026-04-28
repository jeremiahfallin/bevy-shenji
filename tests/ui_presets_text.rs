use bevy::prelude::*;
use bevy::text::TextFont;
use shenji::ui::prelude::*;

#[test]
fn heading_1_renders_with_3xl_font_size() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        heading_1("Title").spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&TextFont>();
    let sizes: Vec<f32> = q.iter(world).map(|f| f.font_size).collect();
    assert!(
        sizes.iter().any(|&s| (s - TEXT_3XL).abs() < f32::EPSILON),
        "expected a TextFont with size {}, got {:?}",
        TEXT_3XL,
        sizes
    );
}

#[test]
fn body_renders_with_base_font_size() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        body("paragraph").spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&TextFont>();
    let sizes: Vec<f32> = q.iter(world).map(|f| f.font_size).collect();
    assert!(sizes.iter().any(|&s| (s - TEXT_BASE).abs() < f32::EPSILON));
}

#[test]
fn caption_renders_with_xs_font_size() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        caption("meta").spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&TextFont>();
    let sizes: Vec<f32> = q.iter(world).map(|f| f.font_size).collect();
    assert!(sizes.iter().any(|&s| (s - TEXT_XS).abs() < f32::EPSILON));
}
