# bevy_declarative UI Port — Phase A.5 Design (Foundation Polish)

**Date:** 2026-04-29
**Status:** Approved (brainstorming)
**Next:** implementation plan via `writing-plans` skill.

## Relationship to Other Plans

Phase A.5 is the small follow-up PR between Phase A (foundations) and Phase B (screen migrations). It addresses three carry-overs flagged by the Phase A final review. Phase B Task 1 is blocked on this work.

**Predecessor work (must be merged first):**
- Phase A foundations — [`2026-04-26-bevy-declarative-port-phase-a-plan.md`](2026-04-26-bevy-declarative-port-phase-a-plan.md), merged in PR #25.

**Unblocks:**
- Phase B Task 1 onward — [`2026-04-29-bevy-declarative-port-phase-b-plan.md`](2026-04-29-bevy-declarative-port-phase-b-plan.md).

## End State

When Phase A.5 is done:
- `src/ui/tokens/borders.rs` exists, byte-equivalent to `src/theme/styles/borders.rs`, exposed via `crate::ui::prelude::*`. Parity test under `tests/ui_borders_parity.rs`.
- `src/lib.rs` declares all 9 top-level modules (`app_caps, asset_tracking, audio, brp [cfg dev], dev_tools [cfg dev], game, menus, screens, theme, ui`). `src/main.rs` removes its own `mod` declarations and consumes the lib via `use shenji::*` or equivalent. Each module compiles once.
- `src/ui/prelude.rs` adds explicit `pub use` lines that disambiguate every glob collision with `bevy::prelude::*`. Currently known: `px`. Audit may surface more.
- `examples/ui_gallery.rs` no longer needs an explicit `bevy_declarative::prelude::px` import workaround — the prelude is unambiguous.

## Scope (strict)

**In:**
- Borders tokens port + parity test.
- lib.rs / main.rs single-compile flip.
- Prelude collision audit + explicit re-export fixes.

**Out:**
- Other Phase A token gaps (e.g. grids, if any) — defer until a screen demands them.
- Screen-side imports cleanup — Phase B's job.
- Test coverage expansion of palette/spacing/typography parity tests — optional polish flagged by reviewer.
- Tooltip hover trigger, Tabs content-swap, ScrollView visual scrollbar — these are Phase B's domain (bundled into the screen PRs that need them).

## The Three Changes

### Change A: Borders tokens

Copy `src/theme/styles/borders.rs` verbatim (~42 lines, all `pub const`) to `src/ui/tokens/borders.rs`. Wire `pub mod borders;` in `src/ui/tokens/mod.rs` and `pub use crate::ui::tokens::borders::*;` in `src/ui/prelude.rs`.

Add `tests/ui_borders_parity.rs`:
```rust
use shenji::theme::styles::borders as old;
use shenji::ui::tokens::borders as new;

#[test]
fn border_widths_match() {
    assert_eq!(new::BORDER_WIDTH_0, old::BORDER_WIDTH_0);
    // ... all BORDER_WIDTH_* constants
}

#[test]
fn radius_constants_match() {
    assert_eq!(new::RADIUS_NONE, old::RADIUS_NONE);
    // ... all RADIUS_* constants
}
```

### Change B: lib.rs / main.rs flip

`src/lib.rs` becomes the canonical module declaration site:
```rust
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

pub use app_caps::*;
// Re-export anything main.rs needs (AppPlugin, Pause, AppSystems, PausableSystems, UiRoot, etc.)
```

`src/main.rs` removes all `mod` declarations and consumes via:
```rust
use shenji::*;
// ... or specific imports
```

The `#[path]` attributes go away. Each module compiles exactly once.

If `AppPlugin`, `Pause`, `AppSystems`, `PausableSystems`, `UiRoot`, or other types currently live in `main.rs`, they must move to `src/lib.rs` (or to a sub-module like `crate::app`) so the binary can `use` them. The migration is mechanical but worth a careful read of `main.rs` first to map out the visibility surface.

### Change C: Prelude collision audit + fixes

Read `src/ui/prelude.rs` and enumerate all glob `pub use` lines. For each, identify symbols that collide with `bevy::prelude::*` (the screens' other glob).

Known collision: `px` (both `bevy::prelude::px` and `bevy_declarative::prelude::px` exist).

For each collision, add an explicit `pub use <chosen-source>::<name>;` line in `prelude.rs`. This wins via Rust's "explicit beats glob" resolution. Document each collision in `prelude.rs`'s module doc comment — what it is, which side won, why.

The known fix:
```rust
pub use bevy_declarative::prelude::*;
// Disambiguate against bevy::prelude::px (also in scope at every screen call site).
// We pick bevy_declarative's `px` because it's the value-construction helper
// the new UI uses pervasively.
pub use bevy_declarative::prelude::px;
```

(Even though `px` is already in `bevy_declarative::prelude::*`, the explicit re-export forces "explicit beats glob" downstream.)

Verify by:
1. Compile `examples/ui_gallery.rs` after removing its current explicit `use bevy_declarative::style::values::px;` workaround.
2. Spot-check any other plausibly-colliding names (`pct`, `Color`, `Val` shorthand, etc.).

## PR Shape

**One PR, three commits in order A → B → C.** The changes are small (~50–100 lines net) and tightly related. Three PRs would be overkill.

Branch: `feat/ui-phase-a5-prep`.

Commit 1: `feat(ui): port border tokens with parity test`
Commit 2: `refactor: flip lib.rs to canonical module owner; thin main.rs`
Commit 3: `feat(ui): explicit prelude re-exports to disambiguate bevy globs`

PR title: `feat(ui): Phase A.5 prep — border tokens, single-compile flip, prelude disambiguation`

## Verification

**Per change:**
- A: `cargo test --test ui_borders_parity` passes (use direct test-binary execution if file lock issue persists).
- B: `cargo build` clean (debug + release); `bevy run` boots and reaches Title without panic; `cargo build --target wasm32-unknown-unknown` clean.
- C: `cargo build --example ui_gallery` clean after removing the explicit `px` workaround in the example.

**End-of-PR matrix:**
- `cargo fmt --all -- --check` clean.
- `cargo build` (debug + release + wasm32) all green.
- `cargo test` green.
- `cargo clippy --all-targets` clean (warnings allowed; no new errors).
- `bevy run` boots, splash → title transitions clean.

## Risks

- **Type relocation surprise** during Change B. If `main.rs` defines types that aren't trivially `pub`-able or have non-portable use, the flip may require a `crate::app` sub-module to host them. Mitigation: read `src/main.rs` end-to-end before starting Change B and map out the visibility surface.
- **Collision audit may surface more than `px`.** If many bevy_declarative names collide, the explicit re-export list grows. Acceptable; document each.
- **Doubled-compile invariant**: after Change B, `grep -rn "#\[path" src/lib.rs` should return zero hits. If not, the flip is incomplete.

## Rollback

If any change post-flip surfaces an unexpected breakage, revert just that commit. Each commit is independently revertable since they touch disjoint surfaces (Change A: new files only; Change B: main.rs/lib.rs only; Change C: prelude.rs only).
