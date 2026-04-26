# bevy_declarative UI Port — Design

**Date:** 2026-04-25
**Status:** Approved (brainstorming)
**Next:** implementation plan via `writing-plans` skill.

## Problem

Shenji's UI is built on `bevy_immediate` 0.4 (immediate-mode, trait-based capability system) under `src/theme/`. The current implementation has bugs — concretely, the Characters view's horizontal scrollbar extends past the viewport and content cannot be scrolled to the end. Switching to a retained-mode builder library is expected to resolve the scrolling class of bugs and align shenji with a simpler, more idiomatic Bevy UI paradigm.

## End State

- Shenji runs on **Bevy 0.18** (upgraded in a prior, separate PR).
- All UI is constructed through `bevy_declarative` builders (`div().flex().col()…spawn(&mut commands)`). No `ImmEntity`, `CapSet`, `ImplCap`, or `ui.ch()` chains remain in shenji.
- A new `src/ui/` module owns shenji's design system: tokens, presets, widgets, components, behaviors. `src/theme/` is deleted.
- `Cargo.toml` no longer depends on `bevy_immediate` (or `lucide_icons`-via-immediate). It depends on `bevy_declarative` via path: `C:\Users\bullf\dev\games\bevy_declarative`.
- `bevy_declarative` itself is **not modified**. Anything missing upstream is implemented inside `src/ui/`.
- The Characters view scrolls horizontally to the end with the scrollbar staying inside the viewport.

## Out of Scope

- Gameplay logic, save format, art, audio.
- Any change to `bevy_declarative`.
- Performance benchmarking.
- Automated visual regression CI.

## Phasing

The 0.17 → 0.18 Bevy upgrade lands first as a separate, prior PR. The UI port begins on a stable 0.18 base. The two paradigms coexist during migration; `bevy_immediate` is removed only after the last call site is gone.

## Module Layout

```
src/ui/
├── mod.rs              # pub fn plugin(app), prelude re-exports
├── tokens/             # palette, spacing, typography (data only)
├── presets/            # buttons, containers, text — free fns returning Div/Text
├── widgets/            # stateless (free fn) and stateful (typed builder) widgets
├── components/         # scroll_view (wraps bevy_declarative scroll, fixes Characters bug)
├── behaviors/          # focus traversal
└── prelude.rs          # one-stop import for screens
```

`src/ui/mod.rs::plugin` is registered from `main.rs` in place of the current `src/theme/mod.rs::plugin`.

## Widget Patterns

**Stateless visual widgets** (badge, divider, icon, label, progress_bar, tooltip, plain button, presets) are **free functions** returning a configured `Div` or `Text`. No new types, no state.

**Stateful interactive widgets** (checkbox, radio, slider, dropdown, text_input, tabs, modal, list, table) are **typed builder structs** implementing `Element + Styled`, paired with a free constructor function (`checkbox(initial)`, `slider(value, range)`).

Conventions for stateful widgets:

1. Builder struct holds config; consumed at spawn.
2. Runtime state lives in a `#[derive(Component)]` on the spawned entity, not in the builder.
3. Callbacks are stored as `Box<dyn Fn(...) + Send + Sync>` and attached as Bevy observers at spawn.
4. Behavior systems live next to the widget in the same file. The widget's `pub fn plugin(app)` is called from `src/ui/mod.rs`.
5. **No two-way binding.** State updates flow: input → component mutation → event/callback. The caller reflects state into the next frame's builder.
6. Widgets emit Bevy events (`CheckboxChanged { entity, checked }`, `SliderChanged { entity, value }`, `TextInputSubmitted { entity, text }`). Builders also expose a convenience `.on_change(closure)` that registers a one-shot observer for the widget's event — closure users get ergonomics, system users get idiomatic Bevy. Both routes coexist.

## Update Policy

**Despawn-and-respawn at screen-level on data changes.** Per-row diffing is premature for shenji's coarse UI update cadence (screen transitions, menu opens, data refreshes).

## Migration Plan

**Phase A — foundations (no screen migrated yet, both UI systems coexist):**

1. Add `bevy_declarative` path dep; register `BevyDeclarativePlugin`.
2. Create `src/ui/` skeleton; port `tokens/` (palette, spacing, typography).
3. Implement `presets/` (buttons, containers, text).
4. Implement stateless widgets: badge, divider, icon, label, progress_bar, tooltip.
5. Build shenji `ScrollView` in `components/scroll_view.rs`. **Fixes Characters bug here.**
6. Implement stateful widgets needed by simpler screens: checkbox, radio, slider, text_input, tabs. Defer dropdown, modal, list, table until a screen needs them.

**Phase B — screen migrations (one PR each):**

7. Splash. 8. Title. 9. Settings/Credits/Pause menus. 10. Loading. 11. NewGame. 12. Gameplay sidebar (implement `list` here). 13. Gameplay bottom bar. 14. Dashboard. 15. **Characters** — explicit acceptance criterion: horizontal scroll reaches end, scrollbar inside viewport. 16. Squads. 17. Research (implement `dropdown`, `modal` here).

**Phase C — cleanup:**

18. Delete `src/theme/`. Remove `bevy_immediate` from `Cargo.toml`. Full clippy/test/playthrough.
19. (Optional retrospective) tighten widget APIs based on migration learnings.

## Verification

**Per-PR baseline:** `cargo fmt --all` clean; `cargo clippy --all-targets` clean; `cargo test` green; `bevy run` boots without panic; `bevy run --target web` builds.

**Tokens:** unit tests asserting parity with `src/theme/styles/palette.rs`.

**Presets and stateless widgets:** an `examples/ui_gallery.rs` rendering one of each, with a reference screenshot captured via `bevy-eyes-on`. Subsequent PRs that touch these diff against the reference.

**ScrollView (step 5):** dedicated test screen with content larger than viewport in both axes. Acceptance criteria, written before implementation:

1. Scrollbar geometry stays inside the viewport at all content sizes.
2. Scrolling reaches the last pixel of content.
3. Resizing the viewport recomputes scrollbar size correctly.
4. Mouse wheel and drag both work.

**Each screen migration:** launch, exercise the screen's primary interaction list (defined in the implementation plan), screenshot before/after, paste in PR description.

**Step 15 (Characters):** PR description includes the named acceptance criterion verified with a roster wide enough to overflow.

**Cleanup (step 18):** grep for residual `bevy_immediate`, `ImmEntity`, `CapSet`, `ImplCap`, `ui.ch(` — must be zero. `cargo tree` confirms `bevy_immediate` not pulled transitively. Full playthrough splash → title → new game → gameplay → save/load → exit. Pre-migration save loads in post-migration build. WASM smoke test once at end of migration.

**Manual screenshot review per PR** is the visual-regression strategy. **One end-of-migration WASM smoke test** is sufficient.

## Risks

- **API churn during migration.** Widget shapes implemented early (step 6) may need revision when later screens (step 12, 17) reveal awkward call sites. Mitigation: defer `list`, `dropdown`, `modal`, `table` to the screens that need them; accept revision PRs on early widgets if real call sites demand them.
- **Save format coupling.** If any UI widget component ends up referenced by serialized state, step 18 tests catch it. Treated as separate fix; out of scope for this design.
- **Bevy 0.18 upgrade leaks into UI port scope.** Mitigation: 0.18 upgrade is its own PR with its own plan, merged before any step here begins.
