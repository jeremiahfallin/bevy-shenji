use bevy::prelude::*;
use bevy::text::TextFont;
use shenji::ui::prelude::*;

#[test]
fn label_produces_text_with_sm_font_size() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        label("Health").spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<(&Text, &TextFont)>();
    let mut found = false;
    for (t, f) in q.iter(world) {
        if t.0 == "Health" && (f.font_size - TEXT_SM).abs() < f32::EPSILON {
            found = true;
        }
    }
    assert!(found, "label should produce sm-sized text");
}
