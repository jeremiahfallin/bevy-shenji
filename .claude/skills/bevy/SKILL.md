---
name: bevy
description: Bevy game engine ECS expertise for Bevy 0.17. Use this skill when implementing game features, designing components/systems, debugging Bevy code, working with Bevy UI, or troubleshooting ECS architecture. Trigger whenever the task involves Bevy components, systems, queries, events, observers, resources, or entities — even if the user doesn't explicitly mention "Bevy".
---

# Bevy 0.17 Development Guide

## Consult Registry Examples First

The local Bevy examples are the most reliable reference. Check them before implementing new features:

```bash
~/.cargo/registry/src/index.crates.io-*/bevy-0.17.*/examples/
```

These cover UI, ECS patterns, rendering, input, audio, and more. When in doubt, find a relevant example and adapt it.

## Bevy 0.17 API Changes

Bevy 0.17 introduced significant breaking changes from 0.16:

- **Materials**: Handles wrapped in `MeshMaterial3d<T>` instead of bare `Handle<T>`
- **Events**: Observer pattern replaces the old event system — use `commands.trigger()` and `add_observer()` instead of `EventWriter`/`EventReader`
- **Colors**: Arithmetic operations on `Color` removed — extract and manipulate individual components instead
- **UI Nodes**: `Style` merged into `Node` — set layout properties directly on `Node` (e.g., `Node { width: Val::Px(200.0), .. }`)

## ECS Thinking

Bevy is data-oriented. Think in terms of **data** (components) and **transformations** (systems), not objects and methods.

- **Components** = Pure data, no logic (but helper methods via `impl` are fine)
- **Systems** = Pure functions that query and transform components
- **Events/Observers** = Decoupled communication between systems
- **Resources** = Global singleton state (use sparingly)

### Component Design

Keep components small and focused — one concern per component. This lets you compose entities flexibly and avoids wasting memory on entities that only need some fields.

```rust
// Good: composable, each component is optional per entity
#[derive(Component)]
pub struct Health { pub current: f32, pub max: f32 }

#[derive(Component)]
pub struct Armor { pub defense: f32 }

// Bad: monolithic, every entity pays for all fields
#[derive(Component)]
pub struct CombatStats { pub health: f32, pub armor: f32, pub strength: f32 }
```

Helper methods on components are encouraged:

```rust
impl Health {
    pub fn is_alive(&self) -> bool { self.current > 0.0 }
    pub fn percentage(&self) -> f32 { self.current / self.max }
}
```

### System Ordering

Order systems by data flow — later systems depend on earlier ones having run:

1. **Input processing** — read player/AI input
2. **State changes** — process events, update game state
3. **Derived values** — calculate anything that depends on state
4. **Visual updates** — materials, animations, transforms
5. **UI updates** — must run last since it reads final state

### Change Detection

Use `Changed<T>` filters to avoid redundant work. This is especially important for UI updates and derived calculations:

```rust
pub fn update_health_bar(
    query: Query<(&Health, &mut HealthBar), Changed<Health>>,
) {
    for (health, mut bar) in query.iter_mut() {
        bar.width = health.percentage() * 100.0;
    }
}
```

### Query Tips

- **Filter early**: `Query<&A, (With<B>, Without<C>)>` instead of filtering inside loops
- **Multiple mutable access**: Use `get_many_mut` when you need to mutate multiple entities from the same query
- **Optional components**: Use `Option<&T>` in queries for components that may not be present
- **Single entity**: Use `query.get_single()` with `if let Ok()` — don't unwrap, entities may not exist yet

## UI Patterns

Bevy uses a flexbox-like layout system. The standard pattern is marker components + startup spawn + update systems:

```rust
#[derive(Component)]
pub struct HealthBar;

// Spawn in startup
pub fn setup_ui(mut commands: Commands) {
    commands.spawn((
        HealthBar,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            width: Val::Px(200.0),
            height: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.8, 0.2, 0.2, 0.9)),
    ));
}

// Update reactively
pub fn update_health_ui(
    health: Query<&Health, (With<Player>, Changed<Health>)>,
    mut ui: Query<&mut Node, With<HealthBar>>,
) {
    if let (Ok(health), Ok(mut node)) = (health.get_single(), ui.get_single_mut()) {
        node.width = Val::Px(health.percentage() * 200.0);
    }
}
```

## Common Pitfalls

1. **Forgetting to register systems** — new systems do nothing until added to the app via `.add_systems()` in the plugin function
2. **Borrowing conflicts** — two systems that both mutably query the same component set can't run in parallel; split queries or use `get_many_mut`
3. **Missing `Changed<T>`** — expensive operations (material updates, UI rebuilds) should use change detection to avoid running every frame
4. **Wrong system ordering** — visual glitches often mean a rendering system runs before the state it reads is updated; add explicit ordering with `.chain()` or system sets
5. **Querying despawned entities** — always use `if let Ok()` or `get()` patterns, never `unwrap()` on entity queries
6. **Asset handle lifetimes** — store `Handle<T>` in components or resources to prevent assets from being dropped

## Performance

For prototypes (<100 entities), don't optimize — change detection is sufficient. For larger counts:

- Use `Changed<T>` and `Added<T>` filters everywhere
- Filter queries with `With`/`Without` instead of post-filtering in loops
- Return early from systems when relevant resources haven't changed
- Profile before optimizing — `cargo run --release` with `bevy/trace` feature for diagnostics

## Build Caution

Bevy takes a long time to compile from scratch. Avoid `cargo clean` unless absolutely necessary — each clean rebuild costs minutes. Stick to one Bevy version per project.
