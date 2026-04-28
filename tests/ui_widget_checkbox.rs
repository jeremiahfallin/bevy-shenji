use bevy::prelude::*;
use shenji::ui::prelude::*;
use shenji::ui::widgets::checkbox;

#[test]
fn checkbox_state_inserted_with_initial_unchecked() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(checkbox::plugin);

    let world = app.world_mut();
    {
        let mut commands = world.commands();
        checkbox::checkbox(false)
            .label("Enable")
            .spawn(&mut commands);
    }
    world.flush();

    let entity = world
        .query_filtered::<Entity, With<checkbox::CheckboxState>>()
        .single(world)
        .unwrap();
    let state = world.get::<checkbox::CheckboxState>(entity).unwrap();
    assert_eq!(state.checked, false);
    assert_eq!(state.disabled, false);
}

#[test]
fn checkbox_state_includes_disabled_flag() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(checkbox::plugin);

    let world = app.world_mut();
    {
        let mut commands = world.commands();
        checkbox::checkbox(true)
            .label("X")
            .disabled(true)
            .spawn(&mut commands);
    }
    world.flush();

    let entity = world
        .query_filtered::<Entity, With<checkbox::CheckboxState>>()
        .single(world)
        .unwrap();
    let state = world.get::<checkbox::CheckboxState>(entity).unwrap();
    assert_eq!(state.checked, true);
    assert_eq!(state.disabled, true);
}
