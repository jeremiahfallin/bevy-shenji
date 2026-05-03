# bevy_declarative UI Port — Phase B Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Migrate all 11 screens (and gameplay sidebar + bottom bar = 13 PRs total) from `bevy_immediate`-based `src/theme/` widgets to `bevy_declarative`-based `crate::ui::` builders.

**Architecture:** One PR per screen. Each PR rewrites the screen's file in place using `crate::ui::prelude::*`. Widgets that don't yet exist in `crate::ui::` (Dropdown, Modal, List, Table, plus completions of Tabs content-swap, Tooltip hover, ScrollView visual scrollbar) are built in the same PR that demands them. `src/theme/` and `bevy_immediate` remain in `Cargo.toml` until Phase C.

**Tech Stack:** Rust 2024, Bevy 0.18, `bevy_declarative` (path dep), `bevy_immediate` 0.5 (still present for un-migrated screens until Phase C).

**Reference design:** [`docs/plans/2026-04-29-bevy-declarative-port-phase-b-design.md`](2026-04-29-bevy-declarative-port-phase-b-design.md). Master design: [`docs/plans/2026-04-25-bevy-declarative-port-design.md`](2026-04-25-bevy-declarative-port-design.md).

**Preconditions before Task 1:**
- Bevy 0.18 upgrade merged (PR #24, commit `0aeb4fb`).
- Phase A foundations merged (PR #25, commit `b4f3307`).
- Phase A.5 prep PR merged: borders tokens ported to `src/ui/tokens/borders.rs`, `src/lib.rs`/`src/main.rs` flipped to single-compile, prelude glob collisions documented. **(Has its own design + plan; not yet written. Block Task 1 on this.)**
- `git status` clean. `cargo build` green on `main`.

---

## Conventions

- **One PR per task.** Each task is its own branch (`feat/ui-migrate-<screen>`), its own merge commit on `main`.
- **Branch from current `main`** — not from a previous task's branch. Tasks can in principle be done out of order if upstream is stable, but the order below minimizes API-design risk.
- **No worktree required.** The user prefers working directly in the main checkout (per the same decision in Phase A — see `2026-04-26-bevy-declarative-port-phase-a-plan.md` Task 1 retro). Stash any unrelated dirty files before checkout.
- **Per-screen template** (see below) is referenced by every task body. Tasks just list the file paths, bundled widget builds, interaction list, and acceptance criteria.
- **`cargo fmt --all` before every commit.** `cargo build` clean before opening the PR. Test runs are out-of-scope per-screen unless the screen has its own integration test (unlikely until Phase C).
- **Widget builds bundled into screen PRs.** A widget build commit precedes the screen-rewrite commit on the same branch. The PR body's "Bundled widget builds" section lists each.
- **`src/theme/` is read-only** for the duration of Phase B. If a migration appears to require touching `src/theme/`, stop and report — that's a sign of unintended coupling.

## Per-Screen Migration Template

This template applies to every Tasks 1–13 below. Don't repeat it in each task; reference it by name (`per-screen template`).

**Steps:**

1. **Branch.** `git checkout main && git pull && git checkout -b feat/ui-migrate-<screen>`.
2. **Read the old screen file end-to-end.** Note every `bevy_immediate` capability call (`ui.ch().*`), every theme widget call, layout structure, conditional rendering, and event handler. Sketch the equivalent `crate::ui::` builder tree as a comment at the top of the file or in scratch notes.
3. **Build any bundled widgets first.** Each new widget = new file under `src/ui/widgets/`, integration test under `tests/ui_widget_*.rs`, plugin registered in `src/ui/mod.rs::plugin`. Same TDD pattern as Phase A: failing test, implement, passing test. **One commit per widget.**
4. **Rewrite the screen file in place.** Replace the contents using `use crate::ui::prelude::*;`. Replace conditional-rendering patterns that previously relied on `bevy_immediate` keying with explicit despawn-and-respawn (Bevy systems watching `Changed<T>` to re-render). Match pre-migration layout/spacing/colors as closely as possible.
5. **Swap registration if needed.** If the screen's `pub fn plugin(app: &mut App)` signature changed, update its caller in `src/screens/mod.rs` or the parent module.
6. **Build clean.** `cargo build`. Fix until green.
7. **Boot smoke test.** `bevy run`. Navigate to the screen via its expected entry path. Run the per-screen interaction list. Watch for panics. Take a screenshot of the migrated screen.
8. **Commit screen rewrite.** `cargo fmt --all && git add -u && git commit -m "feat(ui): migrate <screen> to bevy_declarative"`.
9. **Open PR.** `git push -u origin <branch>` then `gh pr create` with title `feat(ui): migrate <screen> to bevy_declarative`. Body includes:
   - Summary (1-2 lines).
   - "Bundled widget builds" section listing each widget commit with SHA.
   - "Interaction list" section showing what was clicked/typed during smoke.
   - "Screenshots" section with before-and-after.
   - "Per-screen acceptance criteria" with each box checked.
   - "Known deferrals" section (if any).
10. **Wait for review/merge OR self-merge** per `superpowers:finishing-a-development-branch` skill.

**Per-screen acceptance criteria (every screen):**
- Reachable from its expected entry point.
- All interactive elements respond.
- No console panics during 30+ seconds of interaction.
- Visual parity with pre-migration (eyeball compare against pre-migration screenshot).
- For data-driven screens: live data renders correctly under the new builders.
- `cargo build` (debug + release) clean.
- `grep -rn "bevy_immediate\|crate::theme" <screen-file>` returns zero hits in the migrated file.

---

## Task 0: Verify Phase A.5 prep PR is merged

**Files:** none.

**Step 1: Check.** `git log --oneline main | head -10`. Look for commits matching the Phase A.5 scope: borders tokens port, lib.rs/main.rs flip, prelude collision audit. If absent, **stop the plan execution** and route to writing a Phase A.5 plan first.

**Step 2: Confirm clean baseline.** `cargo build` exit 0 on `main`. `cargo fmt --all -- --check` clean.

**Step 3: Confirm new tokens reachable.** `grep -rn "BORDER_WIDTH_\|RADIUS_" src/ui/tokens/` returns hits in `src/ui/tokens/borders.rs`.

If all checks pass, proceed to Task 1.

---

## Task 1: Migrate Splash screen

**Files:**
- Rewrite: `src/screens/splash.rs` (~177 lines pre-migration)
- Possibly modify: `src/screens/mod.rs` (only if `splash::plugin` signature changed)

**Bundled widget builds:** none.

**Approach:** Follow per-screen template. Splash is the smallest and validates the template end-to-end. Read the existing logo + auto-advance timer code, rewrite using `div().bg(...).child(...)` for layout and a `Timer` resource (already in shenji) for the advance.

**Interaction list:**
- Game launches; splash screen renders the logo.
- After ~2 seconds, transitions to Title screen automatically.

**Per-screen acceptance criteria:** standard (per template).

**Commit + PR:** as per template. PR title: `feat(ui): migrate Splash screen to bevy_declarative`.

---

## Task 2: Migrate Title screen

**Files:**
- Rewrite: `src/screens/title/main.rs` (~212 lines)
- Don't touch: `src/screens/title/settings.rs`, `src/screens/title/credits.rs`, `src/screens/title.rs` (Tasks 3, 4 handle those).

**Bundled widget builds:** none.

**Approach:** Read the existing layout. Rewrite using `div().flex().col()` for the menu column, `btn_primary("New Game")`/`btn_ghost("Settings")`/etc. for the buttons. The "SHENJI" title text uses `heading_1(...)` — confirm it picks up `LINE_HEIGHT_NORMAL` correctly (the upgrade added that token).

**Interaction list:**
- From Splash, transition to Title.
- Each button (New Game, Settings, Credits, Quit) is hoverable and clickable.
- New Game → NewGame screen.
- Settings → Settings menu (still bevy_immediate at this point — Task 3).
- Credits → Credits menu (Task 4).
- Quit → app exits.

**Per-screen acceptance criteria:** standard, plus visual parity with pre-migration title (font sizes, button colors, spacing).

**Commit + PR:** PR title: `feat(ui): migrate Title screen to bevy_declarative`.

---

## Task 3: Migrate Settings menu

**Files:**
- Rewrite: `src/screens/title/settings.rs` (~122 lines)

**Bundled widget builds (conditional):**
- **If the Settings menu uses tabs**: complete `Tabs` content-swap behavior. Add a system to `src/ui/widgets/tabs.rs::plugin` that watches `Changed<TabsState>` and despawns/respawns the active tab's content. Add an integration test under `tests/ui_widget_tabs_content_swap.rs`.
- **If Settings has audio sliders**: nothing to build (`Slider` was completed in Phase A); just verify it works in the integration.

If Settings is currently a stub (just a back button), skip the Tabs content-swap build — defer to whichever screen actually uses tab content first (likely Research in Task 13, more likely Settings in a future feature add).

**Approach:** Read the file. If it's mostly a stub, the migration is trivial.

**Interaction list:**
- From Title → Settings → confirm content renders.
- Each setting control responds (sliders move, checkboxes toggle, etc).
- Back button returns to Title.

**Per-screen acceptance criteria:** standard.

**Commit + PR:** PR title: `feat(ui): migrate Settings menu to bevy_declarative`. If Tabs content-swap was bundled, list the widget commit SHA in the PR body.

---

## Task 4: Migrate Credits menu

**Files:**
- Rewrite: `src/screens/title/credits.rs` (~127 lines)

**Bundled widget builds:** none.

**Approach:** Mostly text content. Use `heading_2`/`body`/`caption` presets liberally. May need a scrollable container if the credits list is long — use `scroll_view().vertical(true)`.

**Interaction list:**
- From Title → Credits → text renders.
- Scroll works if applicable.
- Back returns to Title.

**Per-screen acceptance criteria:** standard.

**Commit + PR:** `feat(ui): migrate Credits menu to bevy_declarative`.

---

## Task 5: Migrate Pause menu

**Files:**
- Rewrite: `src/screens/gameplay/pause.rs` (~88 lines)

**Bundled widget builds:** none.

**Approach:** Overlay panel (full-screen semi-transparent backdrop + centered menu). Use `div().w_full().h_full().bg(Color::srgba(0, 0, 0, 0.5))` for the backdrop, `panel()` (or a centered `div()` with `items_center().justify_center()`) for the menu. Buttons: Resume, Settings (re-routes to Title's settings — currently shared), Save, Quit to Title.

**Interaction list:**
- From Gameplay, press Esc → Pause overlay appears.
- Each button responds.
- Resume → returns to Gameplay.
- Quit to Title → exits gameplay screen.

**Per-screen acceptance criteria:** standard, plus the overlay backdrop dims gameplay as before.

**Commit + PR:** `feat(ui): migrate Pause menu to bevy_declarative`.

---

## Task 6: Migrate Loading screen

**Files:**
- Rewrite: `src/screens/loading.rs` (~65 lines)

**Bundled widget builds:** none.

**Approach:** Centered `progress_bar(fraction)` driven by `AssetTracking` resource progress. `label("Loading...")` above it.

**Interaction list:**
- From NewGame, transition to Loading → progress bar advances → transition to Gameplay when complete.

**Per-screen acceptance criteria:** standard, plus progress bar visibly animates as assets load (not stuck at 0% or jumps to 100%).

**Commit + PR:** `feat(ui): migrate Loading screen to bevy_declarative`.

---

## Task 7: Migrate NewGame screen

**Files:**
- Rewrite: `src/screens/new_game.rs` (~161 lines)

**Bundled widget builds:** none. (`TextInput` was built in Phase A.)

**Approach:** Form layout with player-name `text_input("")`, possibly difficulty `radio` group, Start/Cancel buttons.

**Interaction list:**
- From Title → New Game → form renders.
- Click text input → focuses; type characters → value updates visibly.
- Other controls (radio, etc.) respond.
- Start → transitions to Loading.
- Cancel → returns to Title.

**Per-screen acceptance criteria:** standard, plus typing into the text input is responsive (no per-keystroke lag, no dropped characters).

**Commit + PR:** `feat(ui): migrate NewGame screen to bevy_declarative`.

---

## Task 8: Migrate Gameplay sidebar

**Files:**
- Rewrite: `src/game/ui/sidebar.rs` (~138 lines)

**Bundled widget builds:**
- **`List`** widget — `src/ui/widgets/list.rs`, `tests/ui_widget_list.rs`, plugin registered in `src/ui/mod.rs::plugin`.

**`List` widget design (build before screen rewrite):**
- API: `list().item(content).item(content)…` chainable; or `list().items(iter)` for dynamic content.
- Stateful: `ListState { selected: Option<usize> }`. Click on item → set `selected`, emit `ListItemSelected { entity, index }`.
- Render: vertical column of clickable rows, each item highlighted on hover and when selected.
- Inside a `scroll_view()` if content exceeds height — caller wraps if needed; List itself is just the column.

**Approach:** Read sidebar.rs. The sidebar likely shows tabs (Dashboard / Characters / Squads / Research) with content selection driving which content view renders. Use `tabs(active).tab(label, view_marker)` pattern, with the actual content rendered by the existing content-view dispatch mechanism (which Tasks 10–13 migrate separately).

**Interaction list:**
- In Gameplay, sidebar visible.
- Click each tab → active content view changes.
- (Content views still bevy_immediate at this point — they'll show their old appearance until Tasks 10–13.)

**Per-screen acceptance criteria:** standard, plus tab clicks update the active content area.

**Commit + PR:** `feat(ui): migrate Gameplay sidebar to bevy_declarative; build List widget`. Body includes the List widget commit SHA and integration test results.

---

## Task 9: Migrate Bottom bar

**Files:**
- Rewrite: `src/game/ui/bottom_bar.rs` (~53 lines)

**Bundled widget builds:** none.

**Approach:** Horizontal row of status text + a few buttons (likely save indicator, time controls, etc.). `div().flex().row().items_center().justify_between()` with `body(...)` and `btn_ghost(...)` children.

**Interaction list:**
- In Gameplay, bottom bar visible.
- Buttons respond.
- Status text updates as game state changes.

**Per-screen acceptance criteria:** standard.

**Commit + PR:** `feat(ui): migrate Bottom bar to bevy_declarative`.

---

## Task 10: Migrate Dashboard content view

**Files:**
- Rewrite: `src/game/ui/content/dashboard.rs` (~118 lines)

**Bundled widget builds:** none.

**Approach:** The Dashboard renders sections (Resources, Workers, Research, Notifications). Each section has a header + content. Use `panel().child(heading_2("Resources")).child(...)` pattern.

For the Resources section's `if items.is_empty() { "No resources yet" } else { for item in items {...} }` conditional, use a Bevy system that watches `Changed<BaseInventory>` and **despawns + respawns** the entire Dashboard subtree on change. This is the despawn-and-respawn policy from the master design and structurally avoids the keying bug from the upgrade.

**Per-screen-specific acceptance criterion:** the Resources section's stale-entity overlap (deferred from upgrade PR #24) is gone. Verify by:
1. Boot with the current `assets/saves/autosave.ron` (which has resources). Inspect Dashboard → Resources → no overlap.
2. Optionally: temporarily clear `BaseInventory.items` at runtime (or load a freshly-started game), confirm "No resources yet" renders alone.
3. Optionally: trigger a state where items go from empty → populated → confirm clean transition.

**Interaction list:**
- In Gameplay, Dashboard tab active, sections render.
- Numbers update as gameplay proceeds.

**Commit + PR:** `feat(ui): migrate Dashboard content view to bevy_declarative`. PR body: explicit screenshots of Resources section in both empty and populated states. Reference the deferred overlap bug from PR #24's body.

---

## Task 11: Migrate Characters content view

**Files:**
- Rewrite: `src/game/ui/content/characters.rs` (~102 lines pre-migration; the user's `d1615e5` experimental edits become input to the design, not the file)

**Bundled widget builds:**
- **`ScrollView` visual scrollbar** if needed for the bug fix. Add a track + thumb to `src/ui/components/scroll_view.rs`. Track stays inside the viewport (this is the bug fix); thumb size proportional to viewport/content ratio; thumb position proportional to scroll fraction. Optional: drag-the-thumb interaction.
- Update `tests/ui_scroll_view.rs` with new tests for thumb positioning and viewport-bounded geometry.

**Approach:**

The pre-migration Characters view shows the character list in a horizontally scrolling row. The bug: scrollbar extends past viewport, content can't scroll fully to the rightmost character.

Read the user's experimental commit `d1615e5` to see what layout properties they tried (likely `flex_grow: 1`, `min_width: 0`, etc. — Bevy/CSS flex-shrink-prevention tricks). The new builder version should achieve the same intent natively: a fixed-width container for the scroll viewport, with `scroll_view().horizontal().vertical(false)` containing the row of character cards.

**Per-screen-specific acceptance criterion:** horizontal scroll reaches the rightmost character; scrollbar (if rendered) stays inside viewport at default and resized window sizes. Verify by:
1. Load a game with a roster wide enough to overflow the viewport.
2. Scroll to the right end. Confirm the rightmost character card is fully visible, not clipped.
3. Resize the window to a smaller and larger size. Confirm scrollbar geometry recomputes correctly each time.
4. Screenshot scroll-at-start and scroll-at-end.

**Interaction list:**
- In Gameplay → Characters tab active → roster of cards renders horizontally.
- Mouse wheel scrolls horizontally (or uses vertical wheel mapped to horizontal — depends on game UX).
- (If scrollbar visual is built) drag scrollbar thumb works.
- Click a character card → existing selection behavior (unchanged from pre-migration).

**Commit + PR:** `feat(ui): migrate Characters content view to bevy_declarative; fix horizontal scroll bug`. PR body: explicit screenshots of scroll-at-start, scroll-at-end, and resized window. Reference the deferred bug from PR #24's body and Phase A ScrollView's deferred visual scrollbar.

---

## Task 12: Migrate Squads content view

**Files:**
- Rewrite: `src/game/ui/content/squads.rs` (~1180 lines pre-migration — **largest file in Phase B**)

**Bundled widget builds:**
- **`Table`** widget *if* squads displays tabular data (likely — squads-as-rows with member-as-columns or stat-as-columns). Build before screen rewrite. New file under `src/ui/widgets/table.rs`, test under `tests/ui_widget_table.rs`, plugin registered in `src/ui/mod.rs::plugin`. Header row, body rows, optionally row hover/selection.

**Approach:**

Squads is the largest pre-migration file. **Read it slowly** before writing the new version. If the structure decomposes into clear sub-sections (squad list, squad detail, member assignment, etc.), consider splitting the rewrite into multiple commits on the branch:
1. Commit: build Table widget (if needed).
2. Commit: rewrite squad list section.
3. Commit: rewrite squad detail section.
4. Commit: rewrite member assignment section.
5. Commit: wire it together; final layout pass.

Still one PR. The commits are for reviewer bisect-ability within the PR.

If the file is mostly one big function with internal complexity, the rewrite can be one commit; just make sure the resulting code is decomposed into helper functions per section.

**Interaction list:**
- In Gameplay → Squads tab active → squad list renders.
- Click a squad → detail view populates.
- Member assignment controls respond.
- (Whatever else the screen does — verify by reading the source.)

**Per-screen acceptance criteria:** standard, plus all squads-specific interactions preserved.

**Commit + PR:** `feat(ui): migrate Squads content view to bevy_declarative`. If Table built, list its commit SHA in PR body.

---

## Task 13: Migrate Research content view

**Files:**
- Rewrite: `src/game/ui/content/research.rs` (~206 lines pre-migration)

**Bundled widget builds:**
- **`Dropdown`** widget — `src/ui/widgets/dropdown.rs`, test, plugin. Click to open menu, click an option to select + close. Stateful: `DropdownState { value: String, open: bool, options: Vec<String> }`. Event: `DropdownChanged { entity, value }`.
- **`Modal`** widget — `src/ui/widgets/modal.rs`, test, plugin. Centered overlay panel with click-outside-to-dismiss. Stateful: `ModalState { open: bool }`. Event: `ModalDismissed { entity }`.
- **`Tooltip` hover trigger** if Research uses tooltips on tech-tree nodes — complete the hover system in `src/ui/widgets/tooltip.rs`. Watch `Pointer<Over>` and `Pointer<Out>` on entities tagged with a `TooltipTarget` component; show/hide the linked tooltip entity. (Trigger model: caller wraps target with `with_tooltip(text)` helper that auto-attaches the marker + the hidden tooltip child.)

**Approach:**

Research is the riskiest screen — three potential new widget builds plus a tech-tree layout that may have its own complexity. **If the cumulative diff feels too large for one PR**, split into separate PRs for each widget build before the screen migration:
- PR 13a: build Dropdown widget
- PR 13b: build Modal widget
- PR 13c: complete Tooltip hover trigger
- PR 13d: migrate Research content view

This is the only task allowed to deviate from the "one PR per screen" rule, and only if cumulative diff > comfortable review size.

**Interaction list:**
- In Gameplay → Research tab active → tech tree renders.
- Hover over a tech node → tooltip appears (if applicable).
- Click a tech node → details modal opens.
- Modal close button (or click outside) dismisses it.
- Dropdown for filtering/sorting techs (if applicable) opens, selection updates the view.
- Start research on an available tech → research progress begins.

**Per-screen acceptance criteria:** standard, plus all research-specific interactions preserved.

**Commit + PR:** `feat(ui): migrate Research content view to bevy_declarative`. If split per the deviation rule above, follow with a 13b/c/d sequence.

---

## Phase B exit checklist

After Task 13's PR merges, verify before declaring Phase B done:

- [ ] All 13 PRs merged.
- [ ] `grep -rn "bevy_immediate" src/screens/ src/game/ui/` returns zero hits.
- [ ] `grep -rn "use crate::theme" src/screens/ src/game/ui/` returns zero hits.
- [ ] `cargo build` (debug + release + wasm32) all succeed.
- [ ] `cargo fmt --all -- --check` clean.
- [ ] `cargo test` green (all `tests/ui_*.rs` files).
- [ ] Full game playthrough: Splash → Title → New Game → Loading → Gameplay → all four content views → Pause → Save → Quit → relaunch → Continue (load autosave) → 60+s of play. No regressions vs. pre-Phase-B.
- [ ] No `unimplemented!()` macros leaked into shipped code.

When the checklist passes, Phase C planning can begin: design + plan for deletion of `src/theme/` and removal of `bevy_immediate` from `Cargo.toml`.

---

## Notes for the implementer

- **Read each screen's source before writing the new version.** Do not skip this step — the per-screen specifics in this plan are summaries, not specs. The actual file may have edge cases the plan doesn't capture.
- **Match pre-migration layout closely.** This is a port, not a redesign. Visual deviations need explicit justification in the PR body.
- **Despawn-and-respawn for data-driven sections.** Bevy's `Changed<T>` queries combined with `commands.entity(parent).despawn_descendants()` followed by re-spawning is the canonical pattern for the new UI. Avoid trying to mutate child entities in place.
- **`src/theme/` is read-only.** If you find yourself wanting to edit a theme widget, stop and reconsider — that means a screen is still indirectly coupled to the old paradigm.
- **`bevy_immediate` is still imported by un-migrated screens.** Don't remove it from `Cargo.toml` until Phase C.
- **Stop and report** if any per-screen acceptance criterion fails and the fix is non-obvious. Don't improvise around bugs.
- **PR cadence is up to you.** This plan is sized for execution across multiple sessions. Each task is independently mergeable; pause between tasks if you need a break. The only ordering constraint is: Phase A.5 prep PR must be done before Task 1.
