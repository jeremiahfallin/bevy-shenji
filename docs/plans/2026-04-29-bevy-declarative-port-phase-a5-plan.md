# bevy_declarative UI Port — Phase A.5 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Land the three foundation-polish carry-overs from Phase A so Phase B Task 1 can begin.

**Architecture:** One PR with three commits. (1) Port `theme/styles/borders.rs` to `ui/tokens/borders.rs` with parity test. (2) Flip `src/lib.rs` to canonical owner of all 9 top-level modules; thin `src/main.rs` to a binary that consumes the lib (eliminates the `#[path]` doubled-compile workaround). (3) Audit `src/ui/prelude.rs` glob collisions with `bevy::prelude::*` and add explicit re-exports for each.

**Tech Stack:** Rust 2024, Bevy 0.18, `bevy_declarative` (path dep), `bevy_immediate` 0.5.

**Reference design:** [`docs/plans/2026-04-29-bevy-declarative-port-phase-a5-design.md`](2026-04-29-bevy-declarative-port-phase-a5-design.md).

**Preconditions:**
- Phase A foundations merged (PR #25, commit `b4f3307`).
- Branch from current `main`.
- `git status` clean (stash any unrelated dirty files first; the user often has WIP autosave or settings files).
- `cargo build` clean on `main`.

**Conventions:**
- One PR, three commits in order: A → B → C.
- Branch: `feat/ui-phase-a5-prep`.
- `cargo fmt --all` before each commit.
- File-lock workaround for tests still applies: `cargo test --test <name> --no-run`, then run `target/debug/deps/<name>-*.exe` directly with rustup toolchain bin on PATH if `cargo test` is blocked by editor file lock.

---

## Task 0: Branch and verify baseline

**Files:** none.

**Step 1: Confirm clean working tree.**
```bash
cd C:/Users/bullf/dev/games/shenji
git status --short
```
Expected: empty. If dirty, stash with a clear name first.

**Step 2: Pull latest main and branch.**
```bash
git checkout main && git pull
git checkout -b feat/ui-phase-a5-prep
```

**Step 3: Verify baseline build.**
```bash
cargo build 2>&1 | tail -3
```
Expected: `Finished dev profile`. Exit 0.

---

## Task 1: Port border tokens (Change A)

**Files:**
- Create: `src/ui/tokens/borders.rs`
- Create: `tests/ui_borders_parity.rs`
- Modify: `src/ui/tokens/mod.rs`
- Modify: `src/ui/prelude.rs`

### Step 1: Write the failing test

Create `tests/ui_borders_parity.rs`:

```rust
//! Asserts that ported border tokens match `src/theme/styles/borders.rs`
//! exactly. Removed when `src/theme/` is deleted in Phase C.

use shenji::theme::styles::borders as old;
use shenji::ui::tokens::borders as new;

#[test]
fn border_widths_match() {
    assert_eq!(new::BORDER_WIDTH_0, old::BORDER_WIDTH_0);
    assert_eq!(new::BORDER_WIDTH_DEFAULT, old::BORDER_WIDTH_DEFAULT);
    assert_eq!(new::BORDER_WIDTH_2, old::BORDER_WIDTH_2);
    assert_eq!(new::BORDER_WIDTH_3, old::BORDER_WIDTH_3);
    assert_eq!(new::BORDER_WIDTH_4, old::BORDER_WIDTH_4);
}

#[test]
fn radius_constants_match() {
    assert_eq!(new::RADIUS_NONE, old::RADIUS_NONE);
    assert_eq!(new::RADIUS_SM, old::RADIUS_SM);
    assert_eq!(new::RADIUS_DEFAULT, old::RADIUS_DEFAULT);
    assert_eq!(new::RADIUS_MD, old::RADIUS_MD);
    // Add any other RADIUS_* constants present in src/theme/styles/borders.rs.
    // Read that file first to enumerate.
}
```

**Read `src/theme/styles/borders.rs` first** to confirm the exact set of `RADIUS_*` constants — the example above lists 4 but the file may have more (e.g. `RADIUS_LG`, `RADIUS_FULL`). Add an `assert_eq!` for every constant in the source file.

### Step 2: Run the test to verify it fails

```bash
cargo test --test ui_borders_parity --no-run 2>&1 | tail -5
```
Expected: compile error — `ui::tokens::borders` not found.

### Step 3: Port the borders module

Copy `src/theme/styles/borders.rs` verbatim to `src/ui/tokens/borders.rs`. Use:

```bash
cp src/theme/styles/borders.rs src/ui/tokens/borders.rs
```

(Or read + write via the file tools if `cp` is unavailable in your shell.)

Optionally add a one-line module doc at the top of the new file:

```rust
//! Border-width and border-radius design tokens.
//!
//! Ported verbatim from `crate::theme::styles::borders` for the
//! bevy_declarative UI port. Phase C deletes the original.
```

### Step 4: Wire the module

Edit `src/ui/tokens/mod.rs` — add `pub mod borders;` in alphabetical order:

```rust
pub mod borders;
pub mod palette;
pub mod spacing;
pub mod typography;
```

Edit `src/ui/prelude.rs` — add the borders re-export in the tokens block:

```rust
pub use crate::ui::tokens::borders::*;
pub use crate::ui::tokens::palette::*;
pub use crate::ui::tokens::spacing::*;
pub use crate::ui::tokens::typography::*;
```

### Step 5: Run the test to verify it passes

```bash
cargo test --test ui_borders_parity --no-run 2>&1 | tail -3
```
Expected: compiles. Then run the test binary:

```bash
PATH="/c/Users/bullf/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin:$PATH" \
  ./target/debug/deps/ui_borders_parity-*.exe
```
Expected: all asserts pass.

If `cargo test --test ui_borders_parity` works without the file-lock issue, that's fine too.

### Step 6: Commit

```bash
cargo fmt --all
git add src/ui/tokens/borders.rs src/ui/tokens/mod.rs src/ui/prelude.rs tests/ui_borders_parity.rs
git commit -m "feat(ui): port border tokens with parity test"
```

---

## Task 2: Flip lib.rs / main.rs (Change B)

**Files:**
- Modify: `src/lib.rs` (substantial rewrite)
- Modify: `src/main.rs` (substantial rewrite — moves types out, becomes thin)

### Step 1: Read current `src/main.rs` to map types out

`src/main.rs` currently defines:
- `mod` declarations for 9 modules.
- `AppPlugin` struct + `Plugin` impl.
- `AppSystems` enum (currently `enum`, no `pub` — used by other modules via `crate::AppSystems`).
- `Pause` struct (currently `struct`, no `pub` — used by other modules).
- `PausableSystems` struct (currently `struct`, no `pub` — used by other modules).
- `UiRoot` struct (currently `pub struct`).
- `configure_theme_sounds` fn (private).
- `spawn_camera` fn (private).
- `main()` fn (binary entry point).

After the flip, these should live in `src/lib.rs` (or a sub-module like `crate::app`) and be `pub` so `src/main.rs` can use them. The `main()` function stays in `src/main.rs`.

### Step 2: Write the new `src/lib.rs`

Replace the current `src/lib.rs` (which has only `theme` and `ui` via `#[path]`) with:

```rust
//! `shenji` library crate — exposes the app surface for the binary
//! (`src/main.rs`) and integration tests under `tests/`.

#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]

pub mod app_caps;
pub mod asset_tracking;
pub mod audio;
#[cfg(feature = "dev")]
pub mod brp;
#[cfg(feature = "dev")]
pub mod dev_tools;
pub mod game;
pub mod menus;
pub mod screens;
pub mod theme;
pub mod ui;

use crate::app_caps::AppCaps;
use crate::theme::prelude::*;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_declarative::BevyDeclarativePlugin;
use bevy_immediate::BevyImmediatePlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Shenji".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        )
        .add_plugins(BevyImmediatePlugin::<AppCaps>::default())
        .add_plugins(BevyDeclarativePlugin);

        app.add_plugins((
            asset_tracking::plugin,
            audio::plugin,
            game::plugin,
            #[cfg(feature = "dev")]
            brp::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
            ui::plugin,
        ));

        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

        let root = app
            .world_mut()
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(SPACE_5),
                    ..default()
                },
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
                Name::new("UiRoot"),
            ))
            .id();
        app.insert_resource(UiRoot(root));

        app.add_systems(Startup, configure_theme_sounds);
        app.add_systems(Startup, spawn_camera);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AppSystems {
    TickTimers,
    RecordInput,
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PausableSystems;

/// The root UI entity that all screens attach their UI to.
#[derive(Resource)]
pub struct UiRoot(pub Entity);

fn configure_theme_sounds(asset_server: Res<AssetServer>, mut theme_config: ResMut<ThemeConfig>) {
    theme_config.hover_sound = Some(asset_server.load("audio/sound_effects/button_hover.ogg"));
    theme_config.click_sound = Some(asset_server.load("audio/sound_effects/button_click.ogg"));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}
```

Key differences from the original `main.rs` content:
- Module declarations are `pub mod` (not `mod`).
- `AppSystems`, `Pause`, `PausableSystems` gain `pub` (they were crate-visible by virtue of being top-level in main.rs; now they need `pub` to be reachable from main.rs through the lib).
- The `#[path]` workaround is gone — modules own their canonical declaration.
- `configure_theme_sounds` and `spawn_camera` stay private to the lib (no callers outside).

### Step 3: Write the new `src/main.rs`

Replace `src/main.rs` with a thin binary:

```rust
//! `shenji` binary — runs the game.

#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use shenji::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
```

### Step 4: Verify build

```bash
cargo build 2>&1 | tail -5
```
Expected: clean, exit 0.

If errors appear about `crate::AppSystems` not being found in submodules (e.g. `src/screens/mod.rs` may reference `crate::AppSystems::Update`), those are unaffected — `crate::` from inside the lib still points to lib's root. No changes needed in submodules.

### Step 5: Verify the runtime

```bash
bevy run 2>&1 | head -20
```
Watch for:
- Game launches.
- Splash → Title transition.
- No new panics vs main baseline.

Kill the process after ~10 seconds (it'll keep running the game; ctrl+c or close window).

### Step 6: Verify each compilation surface

```bash
cargo build --release 2>&1 | tail -3
cargo build --target wasm32-unknown-unknown --no-default-features 2>&1 | tail -3
```
All should succeed.

### Step 7: Verify the doubled-compile invariant is gone

```bash
grep -n "#\[path" src/lib.rs
```
Expected: zero matches. If any `#[path]` lingers, the flip is incomplete.

### Step 8: Commit

```bash
cargo fmt --all
git add src/lib.rs src/main.rs
git commit -m "refactor: flip lib.rs to canonical module owner; thin main.rs

Eliminates the #[path] workaround that caused theme/ and ui/ to compile
twice (once into the binary, once into the rlib for integration tests).
Each module now compiles exactly once. Required Pause, AppSystems, and
PausableSystems to gain pub visibility so the binary can consume them
through the lib."
```

---

## Task 3: Prelude collision audit + fixes (Change C)

**Files:**
- Modify: `src/ui/prelude.rs`
- Modify: `examples/ui_gallery.rs` (remove the explicit `px` workaround that's now unnecessary)

### Step 1: Audit prelude collisions

Read `src/ui/prelude.rs` to see current globs. List each glob source:
- `bevy_declarative::prelude::*`
- `crate::ui::tokens::palette::*`
- `crate::ui::tokens::spacing::*`
- `crate::ui::tokens::typography::*`
- `crate::ui::tokens::borders::*` (added in Task 1)
- `crate::ui::presets::buttons::*`
- `crate::ui::presets::containers::*`
- `crate::ui::presets::text::*`
- `crate::ui::widgets::*::*` (each widget)
- `crate::ui::components::scroll_view::*`

For each, identify symbols that also exist in `bevy::prelude::*`. Common known/likely collisions:
- `px` (both `bevy::prelude::px` and `bevy_declarative::prelude::px`)
- Possibly `pct` (check)
- Possibly common Bevy types if any preset re-exports them

To audit, you can:
```bash
# Roughly enumerate bevy_declarative's prelude
grep -h "pub use" C:/Users/bullf/dev/games/bevy_declarative/src/lib.rs

# Spot-check known-likely collision names by searching bevy's prelude
# (Not a quick one-liner; rely on compile-time disambiguation warnings as a hint.)
```

A pragmatic approach: build the gallery example **without** its current `bevy_declarative::style::values::px` workaround. If Rust complains about ambiguous `px`, that confirms the collision and that the explicit re-export is needed. Repeat for any other names.

### Step 2: Add explicit re-exports for each collision

Edit `src/ui/prelude.rs`. After the `pub use bevy_declarative::prelude::*;` line, add a doc comment block listing collisions and an explicit `pub use` for each. Example:

```rust
//! One-stop import for screens consuming the new UI.
//!
//! Usage: `use crate::ui::prelude::*;`
//!
//! ## Glob collisions with `bevy::prelude::*`
//!
//! When a screen has both `use bevy::prelude::*;` and `use crate::ui::prelude::*;`
//! in scope, name collisions resolve to whichever explicit `pub use` shadows
//! the glob below. Today the known collisions are:
//!
//! - `px`: resolved to `bevy_declarative::prelude::px` (the value-construction
//!   helper used by the new UI's builder API).

pub use bevy_declarative::prelude::*;

// Disambiguate against bevy::prelude::px (also brought in at every screen
// call site). Explicit re-exports beat globs in Rust resolution.
pub use bevy_declarative::prelude::px;

// ... rest of token / preset / widget re-exports unchanged ...
```

If the audit found other collisions (e.g. `pct`, `Color` shorthand, `Val`-related), add `pub use` lines for each, with a doc-comment line explaining the choice.

### Step 3: Verify the gallery example no longer needs its workaround

Look at `examples/ui_gallery.rs` for an explicit `bevy_declarative::style::values::px` import or similar workaround. Remove it (now that prelude disambiguates). The gallery should now compile relying solely on `use shenji::ui::prelude::*;` for `px`.

If the gallery doesn't have such an import (the workaround may have been documented but not present in the actual code), skip this step.

### Step 4: Verify build

```bash
cargo build --example ui_gallery 2>&1 | tail -3
cargo build 2>&1 | tail -3
```
Both should succeed.

### Step 5: Commit

```bash
cargo fmt --all
git add src/ui/prelude.rs examples/ui_gallery.rs
git commit -m "feat(ui): explicit prelude re-exports to disambiguate bevy globs

Adds explicit pub use lines that win Rust's 'explicit beats glob'
resolution against bevy::prelude::*. Documented in the prelude module
doc. Currently known: px. Removes the workaround import in
examples/ui_gallery.rs."
```

---

## Task 4: PR & merge

### Step 1: Final verification

```bash
cargo fmt --all -- --check
cargo build
cargo build --release
cargo build --target wasm32-unknown-unknown --no-default-features
cargo test --no-run
```

All should succeed. Run the test binaries with the file-lock workaround if needed.

### Step 2: Push branch

```bash
git push -u origin feat/ui-phase-a5-prep
```

### Step 3: Open PR

```bash
gh pr create --title "feat(ui): Phase A.5 prep — border tokens, single-compile flip, prelude disambiguation" --body "$(cat <<'EOF'
## Summary

Three foundation-polish carry-overs from Phase A's final review.
Unblocks Phase B's first screen migration.

- **A: Border tokens.** Port `theme/styles/borders.rs` to `ui/tokens/borders.rs` with parity test.
- **B: Single-compile flip.** `src/lib.rs` becomes the canonical owner of all 9 top-level modules. `src/main.rs` becomes a thin binary that consumes `shenji::AppPlugin`. Eliminates the `#[path]` workaround that compiled `theme/` and `ui/` twice.
- **C: Prelude disambiguation.** Explicit `pub use` lines in `src/ui/prelude.rs` resolve glob collisions with `bevy::prelude::*`. Currently known: `px`. Removes the workaround import in `examples/ui_gallery.rs`.

## Test plan
- [x] `cargo test --test ui_borders_parity` passes.
- [x] `cargo build` (debug + release + wasm32) all succeed.
- [x] `cargo fmt --all -- --check` clean.
- [x] `bevy run` boots and reaches Title without panic.
- [x] `grep -n "#\[path" src/lib.rs` returns zero matches.
- [x] `cargo build --example ui_gallery` clean after removing the explicit `px` workaround.

## Unblocks
- Phase B Task 1 (Splash migration) and onward.

Design: `docs/plans/2026-04-29-bevy-declarative-port-phase-a5-design.md`
Plan: `docs/plans/2026-04-29-bevy-declarative-port-phase-a5-plan.md`

🤖 Generated with [Claude Code](https://claude.com/claude-code)
EOF
)"
```

### Step 4: Hand off to `superpowers:finishing-a-development-branch` for merge

Same flow as Phase A: present 4 options, user picks merge-locally, push main, cleanup branch.

---

## Notes for the implementer

- **Read each file before modifying.** Especially `src/main.rs` — its current shape is the source of truth for what types need to move to lib.rs.
- **Keep changes mechanical.** This PR's job is to relocate code, not refactor it. If you find yourself wanting to rename a type or restructure a function, save it for a separate PR.
- **`src/theme/` is read-only** for Phase A.5, same as Phase B.
- **If Step 4 of Task 2 (`bevy run`) shows new panics** that weren't there on `main`, stop and investigate — the visibility flip may have broken something subtle (e.g. a private `fn spawn_camera` becoming `pub` shouldn't matter, but if it does, surface the issue).
- **The prelude collision audit (Task 3) is small but speculative.** If you find no collisions other than the documented `px`, the change is just one explicit `pub use` line + a doc comment. That's the expected outcome.
