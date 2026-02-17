# Shenji

A 2D management/strategy game built with **Rust** and **Bevy 0.17**.

## Quick Reference

- **Language**: Rust (Edition 2024, nightly-2025-06-26)
- **Engine**: Bevy 0.17.3 with `hotpatching` and `experimental_bevy_ui_widgets`
- **UI Framework**: bevy_immediate 0.4 (immediate-mode, trait-based capabilities)
- **Build Tool**: Bevy CLI (`bevy run` / `bevy build`)
- **Package Manager**: Cargo

## Commands

```sh
bevy run              # Run native dev build (hot-reloading enabled)
bevy run --release    # Run native release build
bevy run --target web # Run web dev build (WASM)
cargo test            # Run tests
cargo fmt --all       # Format code
cargo clippy          # Lint
```

## Project Structure

```
src/
├── main.rs              # Entry point, AppPlugin
├── screens/             # Game screens: Splash → Title → NewGame → Loading → Gameplay
│   ├── gameplay.rs      # Main gameplay screen
│   └── gameplay/pause.rs
├── game/                # Core game logic
│   ├── character.rs     # Character components (Health, Skills, Equipment, Inventory)
│   ├── location.rs      # Location system (Village, City, Ruins, Wilderness)
│   ├── research.rs      # Tech tree with dependency graph
│   ├── resources.rs     # GameState, PlayerState, SquadState, UiState
│   ├── save.rs          # JSON save/load with autosave (60s interval)
│   ├── systems.rs       # Core game systems
│   └── ui/              # In-game UI (sidebar, bottom bar, content views)
│       └── content/     # Dashboard, Characters, Squads, Research views
├── theme/               # UI theme system
│   ├── primitives/      # Layout, style, text, image, visuals helpers
│   ├── styles/          # Palette, typography, buttons, containers, grids
│   └── widgets/         # Reusable: Button, Icon, Label, List
├── asset_tracking.rs    # Asset loading
├── audio.rs             # Music and SFX
└── menus.rs             # Menu state machine (Main, Settings, Credits, Pause)
assets/
├── audio/               # .ogg music and sound effects
├── fonts/               # GoogleSans.ttf, Kenney Space.ttf
├── images/              # Sprites and splash
└── saves/               # JSON save files
```

## Architecture & Conventions

### Bevy Patterns
- **Plugin registration**: Each module exposes `pub fn plugin(app: &mut App)` — not `impl Plugin`
- **State machines**: `Screen` enum for major states, `Menu` enum for overlays, `Pause(bool)` for pause
- **Components**: `#[derive(Component)]` with `#[reflect(Component)]`
- **Resources**: `#[derive(Resource)]` with `#[reflect(Resource)]`
- **System sets**: `AppSystems::TickTimers`, `AppSystems::Update` for execution ordering

### UI (bevy_immediate)
- Trait-based capabilities: `CapabilityButton`, `CapabilityUiVisuals`, etc.
- Chainable builder API: `ui.ch().w_full().h_full().bg(color)`
- `ImmediateAttach` trait for spawning UI entities
- Theme lives in `src/theme/styles/palette.rs` — colors are `PRIMARY_500`, `GRAY_900`, etc.

### Naming
- **Types/Components/Resources**: PascalCase (`CharacterInfo`, `GameState`)
- **Functions/Systems**: snake_case (`spawn_camera`, `spawn_game_layout`)
- **Constants**: UPPER_SNAKE_CASE (`PRIMARY_500`, `GRAY_900`)
- **Modules**: snake_case, use `mod.rs` pattern

### Serialization
- `serde` + `serde_json` for save/load
- Game state persisted as JSON to `assets/saves/`

### Code Style
- 4-space indentation, LF line endings, UTF-8
- Clippy allows `too_many_arguments` and `type_complexity` (Bevy convention)
- Format with `cargo fmt --all`

## Features / Cargo Features

- `dev_native` (default) — native dev with dynamic linking + hot-reloading
- `dev` — core dev features (bevy dev tools, asset hot-reloading)
