use bevy::prelude::*;
use lucide_icons::Icon;
use shenji::ui::prelude::*;

#[test]
fn icon_produces_text_with_glyph_string() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    {
        let mut commands = world.commands();
        icon(Icon::Heart).spawn(&mut commands);
    }
    world.flush();
    let mut q = world.query::<&Text>();
    let strings: Vec<String> = q.iter(world).map(|t| t.0.clone()).collect();
    assert!(
        strings.iter().any(|s| !s.is_empty()),
        "icon should produce non-empty text"
    );
    for s in &strings {
        assert!(
            !s.contains("Heart"),
            "should produce a glyph, not the literal name"
        );
    }
}
