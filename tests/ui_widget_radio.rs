use bevy::prelude::*;
use shenji::ui::prelude::*;
use shenji::ui::widgets::radio;

#[test]
fn radio_state_inserted_with_correct_fields() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(radio::plugin);

    let world = app.world_mut();
    {
        let mut commands = world.commands();
        radio::radio("size", "small")
            .selected(true)
            .label("Small")
            .spawn(&mut commands);
        radio::radio("size", "medium")
            .label("Medium")
            .spawn(&mut commands);
    }
    world.flush();

    let mut q = world.query::<&radio::RadioState>();
    let states: Vec<radio::RadioState> = q.iter(world).cloned().collect();
    assert_eq!(states.len(), 2);

    let small = states.iter().find(|s| s.value == "small").unwrap();
    assert_eq!(small.group_id, "size");
    assert!(small.selected);
    assert!(!small.disabled);

    let medium = states.iter().find(|s| s.value == "medium").unwrap();
    assert_eq!(medium.group_id, "size");
    assert!(!medium.selected);
}

#[test]
fn radio_disabled_flag_propagates() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(radio::plugin);

    let world = app.world_mut();
    {
        let mut commands = world.commands();
        radio::radio("g", "v").disabled(true).spawn(&mut commands);
    }
    world.flush();

    let mut q = world.query::<&radio::RadioState>();
    let state = q.single(world).unwrap();
    assert!(state.disabled);
    assert!(!state.selected);
}
