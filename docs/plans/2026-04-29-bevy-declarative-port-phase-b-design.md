# bevy_declarative UI Port — Phase B Design (Screen Migrations)

**Date:** 2026-04-29
**Status:** Approved (brainstorming)
**Next:** implementation plan via `writing-plans` skill.

## Relationship to Other Plans

This design refines the Phase B portion of the master design at [`2026-04-25-bevy-declarative-port-design.md`](2026-04-25-bevy-declarative-port-design.md). Section 1 (end state, scope), Section 2 (per-screen template), and Section 3 (screen ordering + specifics) below cover the planning-level decisions specific to Phase B.

**Predecessor work (must be merged first):**
- Bevy 0.18 upgrade — [`2026-04-26-bevy-0-18-upgrade-design.md`](2026-04-26-bevy-0-18-upgrade-design.md), merged in PR #24.
- Phase A foundations — [`2026-04-26-bevy-declarative-port-phase-a-plan.md`](2026-04-26-bevy-declarative-port-phase-a-plan.md), merged in PR #25.
- **Phase A.5 prep PR** — borders tokens ported, `lib.rs`/`main.rs` flipped to single-compile, prelude glob collisions documented. **Not yet planned or executed; precondition for Phase B Task 1.**

## End State

When Phase B is done:
- All 11 screens (and the gameplay sidebar + bottom bar) render through `crate::ui::prelude::*` builders. No screen file calls `bevy_immediate` directly.
- `src/theme/` is dead code — present in the tree but unreferenced from `src/screens/` and `src/game/ui/`. Phase C deletes it.
- `bevy_immediate` is still in `Cargo.toml` (Phase C removes).
- All gameplay behavior matches the pre-Phase-B baseline. The Dashboard "Resources" overlap and the Characters horizontal-scroll-bound bugs are fixed (both are properties of the new builders, not separate fixes).
- Each migration landed as its own PR. `main` runnable at every commit.

## Scope (strict)

**In:** the 11 screens; widgets they demand (`Dropdown`, `Modal`, `List`, `Table`, plus completions of Phase A's deferred Tabs content-swap, Tooltip hover, ScrollView visual scrollbar — bundled into the screen PR that needs each).

**Out (deferred to Phase C):** deleting `src/theme/`; removing `bevy_immediate` from `Cargo.toml`; removing `src/lib.rs`'s `theme` re-export.

**Out of scope entirely:** `src/game/` logic refactors; save format changes; new gameplay features; the Bevy CLI WASM smoke fix.

## Per-Screen Migration Template

Each screen migration follows this shape. The implementation plan defines it once; per-screen tasks reference it.

**Steps:**

1. **Read the old screen end-to-end.** Note every theme widget call, layout structure, conditional rendering, and event handler. Sketch the equivalent `crate::ui::` builder tree.
2. **Build any missing widgets first**, in the same branch. New widget = new file under `src/ui/widgets/`, integration test under `tests/`, plugin registered in `src/ui/mod.rs::plugin`. One commit per widget.
3. **Rewrite the screen file in place** using `crate::ui::prelude::*`. Replace conditional-rendering patterns that previously relied on bevy_immediate keying with explicit despawn-and-respawn (per master design's update policy).
4. **Swap the registration** if the screen's plugin function changed signature.
5. **Build + smoke.** `cargo build` clean. `bevy run`, navigate to the screen, exercise the per-screen interaction list, screenshot.
6. **Commit + PR.** One PR per screen. Title: `feat(ui): migrate <screen> to bevy_declarative`. PR body includes the interaction list run, screenshots, and any new widget builds bundled in.

**File structure choice:** rewrite in place (same path, replaced contents). Reasons: registration site doesn't change, `git diff` shows the migration cleanly as one file replacement, no parallel-implementation naming dance.

**Per-screen acceptance criteria (every screen):**
- Reachable from its expected entry point.
- All interactive elements respond.
- No console panics during 30+ seconds of interaction.
- Visual parity with pre-migration (eyeball compare against pre-migration screenshot).
- For data-driven screens: live data renders correctly under the new builders.

**Per-screen-specific acceptance criteria** are listed alongside each screen below.

## Screen Order and Per-Screen Specifics

| # | Screen | File(s) | Bundled widget builds | Notable specifics |
|---|---|---|---|---|
| 1 | Splash | `src/screens/splash.rs` | none | Smallest. Validates the per-screen template end-to-end. |
| 2 | Title | `src/screens/title/main.rs` | none | Buttons + layout. Confirm `heading_1` picks up `LINE_HEIGHT_NORMAL`. |
| 3 | Settings | `src/screens/title/settings.rs` | Tabs content-swap if used; Slider integration if audio sliders exist | First screen likely to demand stateful widget completions. |
| 4 | Credits | `src/screens/title/credits.rs` | none | Mostly text. |
| 5 | Pause menu | `src/screens/gameplay/pause.rs` | none | Buttons + overlay. |
| 6 | Loading | `src/screens/loading.rs` | none | `progress_bar`, `label`. |
| 7 | NewGame | `src/screens/new_game.rs` | none | `text_input`. Confirms basic ASCII path for player-name entry. |
| 8 | Gameplay sidebar | `src/game/ui/sidebar.rs` | **`List`** widget | First non-screen UI. List API shaped by sidebar's actual needs. Slow review. |
| 9 | Bottom bar | `src/game/ui/bottom_bar.rs` | none | Status text + buttons. |
| 10 | Dashboard | `src/game/ui/content/dashboard.rs` | none | **Acceptance criterion:** Resources section's stale-entity overlap (deferred from upgrade) is gone. Verify with empty + populated autosaves. |
| 11 | Characters | `src/game/ui/content/characters.rs` | ScrollView visual scrollbar (track + thumb) if needed | **Acceptance criterion:** horizontal scroll reaches the rightmost character; scrollbar (if rendered) stays inside viewport at default and resized window sizes. The user's `d1615e5` experimental commit's intent feeds into the new layout. |
| 12 | Squads | `src/game/ui/content/squads.rs` | possibly `Table` | Standard. |
| 13 | Research | `src/game/ui/content/research.rs` | **`Dropdown` + `Modal`**, possibly Tooltip-hover trigger | Largest. If diff balloons past comfortable review, split into separate widget-build PRs + screen migration PR (locally, just for this screen). |

**Total PRs:** 13 (one per row).

**Why this ordering:**
- Screens 1–7 don't need new widgets (only Settings might need Tabs content-swap). Foundation validation.
- Screen 8 (Sidebar) introduces List, the first major widget build mid-Phase-B.
- Screens 10–11 carry explicit bug-fix acceptance criteria.
- Screen 13 is last because it concentrates the most new widget work.

## Risks

- **Settings might be a stub** — if `src/screens/title/settings.rs` is currently empty/minimal, its migration is trivial and Tabs content-swap completion can wait for a screen that actually needs it.
- **Sidebar's List widget API** shapes every future scrollable list in shenji. Worth careful review on that PR.
- **Research is the riskiest screen.** Three new widgets in one PR plus complex tech-tree layout. Pre-emptive mitigation: split if the diff exceeds review comfort.
- **Per-screen visual regressions** that aren't bugs but are noticed on screenshot review (e.g. slightly different padding, color, font-weight) need judgment calls. Default: match pre-migration as closely as possible; deviations need explicit justification in PR description.

## Verification

Per-screen verification is in the per-screen template above (build + smoke + screenshot + interaction list).

**Phase B exit criteria** (the equivalent of step 18 from the master design's Phase C):
- Grep for `bevy_immediate` callers in `src/screens/` and `src/game/ui/` returns zero hits.
- Grep for `crate::theme::` imports in `src/screens/` and `src/game/ui/` returns zero hits.
- All 13 PRs merged.
- Full game playthrough: splash → title → new game → all four content views → save → quit → relaunch → load → continue play 60+s. No regressions vs pre-Phase-B.

**Out of scope for Phase B verification** (Phase C handles): WASM smoke; deletion of `src/theme/`; removal of `bevy_immediate`.

## Tracked Follow-ups

- **Phase A.5 prep PR** (precondition): borders tokens, lib.rs flip, prelude collisions audit. Has its own design + plan to be written.
- **Phase C plan**: deletion of `src/theme/`, removal of `bevy_immediate`. Written after Phase B is done so it can reflect any leftover surprises.
- **Bevy CLI WASM bug** (deferred from upgrade): not part of Phase B; addressed if/when CLI is upgraded.
