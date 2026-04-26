# Bevy 0.17 → 0.18 Upgrade — Design

**Date:** 2026-04-26
**Status:** Approved (brainstorming)
**Next:** implementation plan via `writing-plans` skill.

## Problem

The `bevy_declarative` UI port plan (see [2026-04-25-bevy-declarative-port-design.md](2026-04-25-bevy-declarative-port-design.md)) requires `bevy_declarative`, which targets Bevy 0.18. Shenji is currently on Bevy 0.17.3. This upgrade lands first as a precondition for Phase A of the UI port. It also unblocks any future work that depends on Bevy 0.18 features.

## End State

- `Cargo.toml` declares `bevy = "0.18"` and `bevy_immediate = "0.5"`. Other deps unchanged.
- `cargo build`, `cargo build --release`, and `cargo build --target wasm32-unknown-unknown` succeed.
- `bevy run` boots the game through splash → title → new game → gameplay without panics or visual regressions.
- The pre-upgrade `assets/saves/autosave.ron` loads cleanly in the post-upgrade build. **Hard acceptance criterion.**
- Existing `src/theme/` UI continues to render and behave identically; any `bevy_immediate` 0.5 API drift is absorbed inside `src/theme/`.
- Bevy 0.18 migration applied across the codebase: `BorderRadius` on `Node`, `LineHeight` on text, observer mutability rules, `set_if_neq` for idempotent state transitions, etc.
- `CLAUDE.md` updated: "Bevy 0.17.3" → "Bevy 0.18".

## Scope (strict)

Only changes required to compile and run on Bevy 0.18 + `bevy_immediate` 0.5 with the autosave loadable. Nothing else.

**Out of scope:**
- Any change to `src/ui/` (does not exist yet).
- Bumping `rand`, `ron`, `serde`, `tracing`, `lucide-icons`.
- Refactoring `theme/` widgets beyond what `bevy_immediate` 0.5's API requires.
- New tests beyond upgrade-specific smoke verification.
- Deprecation-warning fixes that aren't compile errors.
- Documentation rewrites beyond the version bump in `CLAUDE.md`.

## Risks

- **`bevy_immediate` 0.5 API drift.** May ripple through `src/theme/widgets/`. Cap effort: if a single widget's update exceeds ~30 minutes, deprecate-in-place with a `// TODO: Phase A widget replaces this` comment and accept lower fidelity. The UI port replaces these widgets anyway.
- **Toolchain MSRV.** `nightly-2025-06-26` may not satisfy `bevy_immediate` 0.5's MSRV (Rust 1.89). Bump `rust-toolchain.toml` if needed.
- **Feature flag drift.** `hotpatching` and `experimental_bevy_ui_widgets` may have been renamed/removed in 0.18. Drop or rename with a comment; don't block the upgrade on chasing them.

## Migration Touch-Points

These are the places we expect to make code changes. Each is a discrete commit in the implementation plan.

1. **`BorderRadius` is now a `Node` field.** Grep for `BorderRadius` and `border_radius`. Move values into the `Node` literal.
2. **`LineHeight` required on `Text`/`Text2d`/`TextSpan`.** Grep for `Text::new(`, `TextSpan`, `Text2d`. Add `LineHeight` next to each spawn. Centralize in `src/theme/primitives/text.rs` if possible.
3. **`TextLayoutInfo.section_rects` → `run_geometry`.** Grep for `section_rects`. Update if shenji reads it directly.
4. **EntityEvent mutability.** Grep for observer signatures that mutate event payloads. Use `SetEntityEventTarget` where mutation is required.
5. **State transitions on equal.** Grep for `next_state.set(...)`. Switch to `set_if_neq` where idempotent transitions are relied on. Likely affects `Pause`, `Screen`, `Menu`.
6. **`AssetServer` / `AssetProcessor` construction.** Likely no-op (DefaultPlugins constructs these). Confirm with grep.
7. **`AssetPlugin` new field.** `src/main.rs:36–40` constructs `AssetPlugin { ..default() }`; the `..default()` should absorb new fields. Confirm by compiling.
8. **`bevy_immediate` 0.5 drift.** Surfaces as compile errors after the dep bump. Fix per-widget; deprecate-in-place per risk policy if cost balloons.
9. **`ron` re-export removal.** Already a direct dep; no-op.

## Order of Operations (single PR, multiple commits)

1. Bump `bevy` and `bevy_immediate` versions, plus toolchain if needed.
2. Run `cargo build`; record the broken-callsite count.
3. Fix touch-points 1–7 in compile-error order. One commit per touch-point.
4. Fix `bevy_immediate` 0.5 drift (touch-point 8). One commit per non-trivial widget.
5. Once `cargo build` is green, run `bevy run`. Fix runtime panics. One commit per panic fix.
6. Verify autosave loads. Commit any save-load fix.
7. Update `CLAUDE.md` version reference.

## Verification

**Build matrix (all must pass before merge):**

- `cargo build`
- `cargo build --release`
- `cargo build --target wasm32-unknown-unknown`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo fmt --all -- --check`

**Runtime smoke test (manual, screenshots in PR description):**

1. `bevy run` — splash → title transition works.
2. New Game → new-game screen loads.
3. Start a game; reach gameplay; open each content view (Dashboard, Characters, Squads, Research).
4. Pause menu opens and closes without issue.
5. Save and quit.
6. Relaunch. **Pre-upgrade `autosave.ron` must load cleanly** (hard acceptance criterion).
7. Continue play ≥60s; autosave loop runs without panic.

**WASM smoke test (one-time):** `bevy run --target web` — boots and shows interactive title.

**Behavior spot-checks:**
- After `set_if_neq` fixes: double-clicking the same menu button does not double-trigger `OnEnter` side effects.
- After `LineHeight` additions: text-heavy screens (Research, Characters) show identical line spacing to pre-upgrade.

**Out of scope:** new automated tests, performance comparison, transitive dep updates, `src/ui/` (does not exist yet).

## Rollback

If any acceptance criterion fails and the fix is non-obvious, revert the PR. The next attempt is a separate PR. No half-upgraded merges.
