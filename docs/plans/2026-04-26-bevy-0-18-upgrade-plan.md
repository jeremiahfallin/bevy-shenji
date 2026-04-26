# Bevy 0.17 → 0.18 Upgrade Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Upgrade shenji from Bevy 0.17.3 to Bevy 0.18, bumping `bevy_immediate` 0.4 → 0.5, applying all Bevy 0.18 migration touch-points, and preserving the loadability of the existing autosave.

**Architecture:** Single coordinated PR with one commit per logical migration touch-point. The "failing test" at every stage is `cargo build` — broken call sites surface as compile errors that the plan resolves in dependency order. Strict scope: only changes required to compile, run, and load the existing autosave on Bevy 0.18 + `bevy_immediate` 0.5. No refactors, no other dep bumps, no new tests beyond the smoke verification.

**Tech Stack:** Rust 2024, Bevy 0.18, `bevy_immediate` 0.5.

**Reference design:** [docs/plans/2026-04-26-bevy-0-18-upgrade-design.md](2026-04-26-bevy-0-18-upgrade-design.md).

**Preconditions before starting Task 1:**
- Working tree is clean (`git status` shows no uncommitted source changes outside this branch).
- A backup copy of the pre-upgrade `assets/saves/autosave.ron` exists outside the repo (e.g. `~/shenji-autosave-pre-0.18-backup.ron`). This is the artifact the verification step in Task 16 reloads.
- Branch off `main`: `git checkout -b chore/bevy-0-18-upgrade`.
- Internet access for `cargo update` to fetch new crate versions.

**Conventions:**
- One commit per task. Commit messages use the form shown in each task.
- Run `cargo fmt --all` before every commit. Don't bother running it as a separate step unless the task changes hand-edited code.
- `cargo clippy --all-targets -- -D warnings` only at the end (Task 17). Mid-upgrade, warnings are noise.
- If a step's expected output doesn't appear, **stop and report**. Don't improvise around it — the upgrade may have surfaced a touch-point not in the plan.
- Commands assume `bash` shell. On Windows, swap path separators if needed.

---

## Task 1: Bump Bevy and `bevy_immediate` versions

**Files:**
- Modify: `Cargo.toml:8-9`

**Step 1: Edit `Cargo.toml`**

Replace:

```toml
bevy = { version = "0.17.3", features = ["hotpatching", "experimental_bevy_ui_widgets"] }
bevy_immediate = { version = "0.4", features = ["hotpatching"] }
```

With:

```toml
bevy = { version = "0.18", features = ["hotpatching", "experimental_bevy_ui_widgets"] }
bevy_immediate = { version = "0.5", features = ["hotpatching"] }
```

**Step 2: Update lockfile**

Run: `cargo update -p bevy -p bevy_immediate`
Expected: lockfile updates; both packages now resolve to 0.18.x and 0.5.x respectively.

**Step 3: Attempt build (expected to fail)**

Run: `cargo build 2>&1 | tee /tmp/build-after-bump.log`
Expected: build fails. **Record the error count** at the bottom of the log (`grep "^error" /tmp/build-after-bump.log | wc -l`) and the first 5 distinct error categories. This is the working list for Tasks 2–11.

If `hotpatching` or `experimental_bevy_ui_widgets` is reported as an unknown feature in Bevy 0.18, drop the offending feature flag (e.g. change to `features = ["hotpatching"]` only or remove the line altogether). Add a code comment in `Cargo.toml`: `# experimental_bevy_ui_widgets removed in Bevy 0.18`.

**Step 4: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore(deps): bump bevy 0.17.3 -> 0.18 and bevy_immediate 0.4 -> 0.5

Build is broken after this commit. Subsequent commits in this PR
fix call sites for the 0.17 -> 0.18 migration."
```

---

## Task 2: Pin Rust toolchain if needed

**Files:**
- Possibly create: `rust-toolchain.toml`

**Step 1: Check current toolchain**

Run: `rustc --version`
Expected: prints a Rust version. If it's `1.89` or newer, **skip to Step 4** (no pin needed).

**Step 2: If MSRV is too low, install a working nightly**

Run: `rustup toolchain install nightly` (or specific date if reproducibility matters)
Expected: toolchain installs.

**Step 3: Pin toolchain**

Create `rust-toolchain.toml`:

```toml
[toolchain]
channel = "nightly"
components = ["rustfmt", "clippy"]
```

**Step 4: Verify build no longer fails on MSRV**

Run: `cargo build 2>&1 | head -20`
Expected: previous MSRV-related errors are gone (other compile errors remain — that's fine).

**Step 5: Commit (only if Step 3 created a file)**

```bash
git add rust-toolchain.toml
git commit -m "chore: pin nightly toolchain for bevy_immediate 0.5 MSRV"
```

If no file was created, skip the commit and proceed to Task 3.

---

## Task 3: Move `BorderRadius` from separate component to `Node` field

**Files:**
- Modify: `src/theme/scroll.rs`
- Modify: `src/theme/primitives/visuals.rs`

**Step 1: Survey occurrences**

Run: `grep -rn "BorderRadius" src/`
Expected: a small number of hits in the two files above (confirmed during plan authoring).

**Step 2: Apply migration**

For each `.insert(BorderRadius::all(...))` or `.spawn((..., BorderRadius::all(...)))` call site:

Before (Bevy 0.17):
```rust
commands.spawn((
    Node { /* ... */ },
    BorderRadius::all(Val::Px(8.0)),
));
```

After (Bevy 0.18):
```rust
commands.spawn(Node {
    border_radius: BorderRadius::all(Val::Px(8.0)),
    /* ... */
});
```

For trait-based mutation (e.g. `entity.insert(BorderRadius::...)` inside `bevy_immediate` extensions in `src/theme/primitives/visuals.rs`), set `node.border_radius = BorderRadius::...` on the `Node` mutable reference instead.

**Step 3: Verify those files compile**

Run: `cargo build 2>&1 | grep "BorderRadius"`
Expected: no `BorderRadius`-related errors remain. Other compile errors may persist.

**Step 4: Commit**

```bash
git add src/theme/scroll.rs src/theme/primitives/visuals.rs
git commit -m "fix(0.18): move BorderRadius onto Node field"
```

---

## Task 4: Add `LineHeight` to all `Text` / `Text2d` / `TextSpan` spawns

**Files:**
- Modify: every file containing `Text::new(`, `TextSpan`, or `Text2d` (discovered in Step 1).

**Step 1: Survey occurrences**

Run: `grep -rn "Text::new\|TextSpan\|Text2d" src/`
Expected: a list of files. Record the count.

**Step 2: Add a helper**

In `src/theme/primitives/text.rs` (or create if it lacks a suitable spot), add:

```rust
use bevy::prelude::*;

/// Default LineHeight for Bevy 0.18 — required component on Text spawns.
/// Uses the project's `LEADING_NORMAL` factor.
pub const DEFAULT_LINE_HEIGHT: LineHeight = LineHeight::RelativeToFont(1.5);
```

(If `LineHeight::RelativeToFont` is not the actual variant in 0.18, use whatever the migration guide recommends — likely `LineHeight::Relative(1.5)` or `LineHeight::Px(...)`.)

**Step 3: Apply migration**

For each `Text` spawn, add `LineHeight` adjacent in the bundle:

Before:
```rust
commands.spawn((Text::new("hello"), TextFont::default(), TextColor(Color::WHITE)));
```

After:
```rust
commands.spawn((
    Text::new("hello"),
    TextFont::default(),
    TextColor(Color::WHITE),
    DEFAULT_LINE_HEIGHT,
));
```

For `bevy_immediate`-based spawns inside `src/theme/`, the same component must be added to whatever bundle the cap inserts. Read `bevy_immediate` 0.5's `text` cap source if needed: `cargo doc --open -p bevy_immediate_ui` or consult `~/.cargo/registry/src/.../bevy_immediate_ui-0.5.0/`.

**Step 4: Verify**

Run: `cargo build 2>&1 | grep "LineHeight"`
Expected: no `LineHeight`-related missing-component errors remain.

**Step 5: Commit**

```bash
git add src/
git commit -m "fix(0.18): add LineHeight to all Text/TextSpan/Text2d spawns"
```

---

## Task 5: Update `set` → `set_if_neq` for idempotent state transitions

**Files:**
- Modify: any of the 9 files identified by `grep -l "next_state.set(" src/` containing transitions that may set the state to its current value.

**Step 1: Identify candidates**

Run: `grep -rn "next_state.set(" src/screens/ src/menus.rs`
Expected: list of `next_state.set(...)` calls. The candidates are calls that could be invoked while already in the target state — typically `Pause` toggles, Menu opens that re-trigger, `Screen` transitions on click.

**Step 2: Apply migration**

For each candidate, replace `.set(X)` with `.set_if_neq(X)` **only where idempotent transitions were relied upon**. Where a deliberate re-entry of the same state was used (e.g. resetting on a button press), leave `.set` alone — the new transition behavior is correct there.

Example:

Before:
```rust
fn on_pause_button(mut next: ResMut<NextState<Pause>>, current: Res<State<Pause>>) {
    next.set(Pause(!current.0));
}
```

`set_if_neq` on this is harmless because the value differs by definition. No change needed. **The likely real targets are screen transitions invoked by general "go to title" handlers that may run when already on Title.**

**Step 3: Verify build still compiles, behavior preserved**

Run: `cargo build`
Expected: builds (assuming Tasks 3–4 done). No compile errors from this task — the migration is behavioral, not type-level. We catch behavior bugs in Task 16 (autosave + smoke test).

**Step 4: Commit**

```bash
git add src/
git commit -m "fix(0.18): use set_if_neq for idempotent state transitions"
```

If no changes were necessary, skip the commit.

---

## Task 6: Update observer signatures for EntityEvent immutability

**Files:**
- Modify: any file with `On<Pointer<...>>` or other `On<EntityEvent>` observers that mutate the event payload.

**Step 1: Identify candidates**

Run: `grep -rn "On<Pointer\|On<.*Trigger\|EntityEvent" src/`

**Step 2: Survey for mutation patterns**

For each match, read the observer body. If it mutates the event (e.g. `event.propagate(false)` was done via `&mut`), note it. Most observers in shenji likely only read the event — those need no change.

**Step 3: Apply migration**

Where mutation is required, switch to `SetEntityEventTarget` per the [Bevy 0.18 migration guide](https://bevy.org/learn/migration-guides/0-17-to-0-18/). Most likely zero changes here.

**Step 4: Verify**

Run: `cargo build`
Expected: builds (assuming previous tasks done).

**Step 5: Commit (only if Step 3 changed code)**

```bash
git add src/
git commit -m "fix(0.18): respect EntityEvent immutability in observers"
```

---

## Task 7: Update `TextLayoutInfo.section_rects` references

**Step 1: Survey**

Run: `grep -rn "section_rects" src/`
Expected: zero hits is the most likely outcome (shenji doesn't seem to read text layout directly).

**Step 2: Migrate if needed**

If hits are found, replace `section_rects` with `run_geometry` and adapt field access. Otherwise, skip.

**Step 3: Commit (only if Step 2 changed code)**

```bash
git add src/
git commit -m "fix(0.18): replace TextLayoutInfo.section_rects with run_geometry"
```

---

## Task 8: Confirm `AssetPlugin` and `AssetServer` callsites still compile

**Files:**
- Inspect: `src/main.rs:36-40`

**Step 1: Read current AssetPlugin construction**

`src/main.rs:36–40` constructs:

```rust
.set(AssetPlugin {
    meta_check: AssetMetaCheck::Never,
    ..default()
})
```

**Step 2: Verify build**

Run: `cargo build 2>&1 | grep -i "AssetPlugin\|AssetServer\|AssetProcessor"`
Expected: no errors. The `..default()` should absorb the new `use_asset_processor_override` field.

**Step 3: If errors appear**

Add the new field explicitly: `use_asset_processor_override: None`. Commit as below.

**Step 4: Commit (only if changes were needed)**

```bash
git add src/main.rs
git commit -m "fix(0.18): handle AssetPlugin new field"
```

---

## Task 9: Resolve `bevy_immediate` 0.5 API drift — `src/theme/` widgets

**Files:**
- Modify: any file under `src/theme/` that fails to compile due to `bevy_immediate` 0.5 API changes.

**Step 1: Survey**

Run: `cargo build 2>&1 | grep "src/theme" | head -50`
Expected: a list of compile errors scoped to `src/theme/`. Most other migrations should be done by now; remaining errors are `bevy_immediate` drift.

**Step 2: For each error, check upstream**

Read the relevant 0.5 module source under `~/.cargo/registry/src/.../bevy_immediate_*-0.5.0/src/` (or `cargo doc --open -p bevy_immediate`). Match the new API.

**Step 3: Apply minimal fix per file**

One commit per file or per logical widget cluster. **Do not refactor.** If a widget's fix balloons past ~30 minutes:

1. Comment out the broken function bodies, leaving the signatures.
2. Make them `unimplemented!("Phase A widget replaces this — see docs/plans/2026-04-26-bevy-declarative-port-phase-a-plan.md")`.
3. Add `#[allow(unreachable_code)]` if needed.
4. Move on.

The goal is "compiles and boots." The Phase A UI port replaces these widgets entirely.

**Step 4: Verify each fix**

Run: `cargo build 2>&1 | grep "src/theme/<file>"` after each commit.
Expected: errors for that file are gone.

**Step 5: Commit per logical change**

```bash
git add src/theme/<file>.rs
git commit -m "fix(0.18): adapt <widget> to bevy_immediate 0.5 API"
```

---

## Task 10: Resolve `bevy_immediate` 0.5 API drift — call sites in `src/screens/` and `src/game/ui/`

**Files:**
- Modify: any file outside `src/theme/` that calls into the theme/widget API and fails to compile.

**Step 1: Survey**

Run: `cargo build 2>&1 | grep -v "src/theme" | head -30`
Expected: remaining errors in screens or game UI.

**Step 2: Apply minimal fixes**

Same approach as Task 9 — adapt call sites to new API; don't refactor; defer to Phase A where possible.

**Step 3: Verify build**

Run: `cargo build`
Expected: **builds clean. No compile errors anywhere.**

If errors remain that don't fit any prior task, **stop and report** — there is an unanticipated migration touch-point.

**Step 4: Commit per logical change**

```bash
git add src/<path>
git commit -m "fix(0.18): adapt <area> to updated theme/widget API"
```

---

## Task 11: First successful build — checkpoint

**Step 1: Confirm build matrix**

Run, in sequence:

```bash
cargo build
cargo build --release
cargo build --target wasm32-unknown-unknown
```

Expected: all three succeed. Record build times for the PR description.

**Step 2: Commit if any incidental fmt/whitespace drift was introduced**

Run: `cargo fmt --all`
If `git status` shows fmt-only changes, commit:

```bash
git add -u
git commit -m "style: cargo fmt after 0.18 migration"
```

---

## Task 12: Boot the game; fix runtime panics

**Step 1: First boot**

Run: `bevy run`
Expected: window opens; splash screen renders.

**Step 2: If panic, capture and fix**

If the game panics during startup:

1. Read the panic message and stack trace.
2. Locate the source line.
3. Determine if the issue is a 0.18 migration we missed (e.g. a state transition firing twice and triggering an assertion, a `Text` spawn missing `LineHeight` we didn't catch in static analysis, etc.).
4. Fix minimally.
5. Commit with `fix(0.18): <specific cause>` message.
6. Re-run `bevy run`.

Repeat until splash → title transitions cleanly.

**Step 3: Walk the screens**

Click through:
- Title → Settings → back
- Title → Credits → back
- Title → New Game → start a game
- Gameplay → open each content view (Dashboard, Characters, Squads, Research)
- Pause (Esc) → Settings → back → Resume
- Quit to title

After each transition, watch the terminal for warnings/panics. Each panic gets a fix commit. Each warning is noted but only fixed if it indicates incorrect behavior (per strict-scope policy).

**Step 4: Final boot run with no fixes needed**

Run: `bevy run`. Walk all screens. No panics, no migration warnings.

**Step 5: Commit each fix as you go**

Pattern: `fix(0.18): <runtime symptom>`. E.g.:

```bash
git commit -m "fix(0.18): handle empty TextLayoutInfo on title screen"
```

---

## Task 13: Verify save/load — pre-upgrade autosave loads cleanly

This is the **hard acceptance criterion** from the design.

**Step 1: Locate the backed-up pre-upgrade autosave**

Confirm `~/shenji-autosave-pre-0.18-backup.ron` (from preconditions) exists. If not, this step cannot be verified — pause the upgrade and investigate before proceeding.

**Step 2: Place the backup as the active autosave**

```bash
cp ~/shenji-autosave-pre-0.18-backup.ron assets/saves/autosave.ron
```

**Step 3: Boot and load**

Run: `bevy run`
In the title screen, choose "Continue" / "Load" (whichever shenji exposes).
Expected: the game state from before the upgrade loads. Continue play for ≥60 seconds.

**Step 4: If load fails**

The failure is one of:

1. **RON parse error** — schema drifted because some serialized type changed in 0.18. Read the error, identify the offending field, and either (a) make the loader tolerant via `#[serde(default)]` or `#[serde(skip)]` on the offending field if it's now obsolete, or (b) add a one-shot migration step in `src/game/save.rs`. **Do not change the on-disk format.**
2. **Panic during state restoration** — likely a Bevy state-transition behavior change. Log the offending transition; apply `set_if_neq` if missed.

Commit each fix individually with `fix(0.18): <save-load issue>`.

**Step 5: Confirm subsequent autosave roundtrip**

After 60+ seconds of play, the autosave timer fires. Quit. Inspect `assets/saves/autosave.ron` — it should be a fresh RON written under 0.18. Relaunch and load again to confirm the roundtrip.

**Step 6: Commit the test artifact for traceability (optional)**

If load required code changes, the post-upgrade `autosave.ron` may now reflect new defaults. Don't commit the autosave itself — it's a generated artifact. The fix commits in Step 4 are the real record.

---

## Task 14: WASM smoke test

**Step 1: Build**

Run: `bevy run --target web`
Expected: builds, opens browser, shows splash → title.

**Step 2: Walk the title menu in browser**

Click each top-level button. Confirm no console errors that didn't exist pre-upgrade. (Console output is visible in the browser devtools; capture screenshots if anything looks off.)

**Step 3: If WASM-specific panic**

Most 0.18 changes are backend-agnostic. A WASM-only failure is most likely a feature-flag issue (e.g. `getrandom` `wasm_js` already in `Cargo.toml:25`). Fix minimally; commit as `fix(0.18): <wasm-specific issue>`.

---

## Task 15: Update `CLAUDE.md` version reference

**Files:**
- Modify: `CLAUDE.md`

**Step 1: Find current reference**

Run: `grep -n "0.17" CLAUDE.md`
Expected: at least one hit ("Bevy 0.17.3" in the Quick Reference section).

**Step 2: Update**

Edit `CLAUDE.md` to replace `Bevy 0.17.3` with `Bevy 0.18`. Don't update other content — strict scope.

**Step 3: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: update CLAUDE.md to reference Bevy 0.18"
```

---

## Task 16: Final verification matrix

**Step 1: Run all checks**

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build
cargo build --release
cargo build --target wasm32-unknown-unknown
```

Expected: all green.

**Step 2: If clippy fails on new 0.18 lints**

Strict scope: fix only the lints that are also compile errors. Other lint failures get an `#[allow(...)]` with a comment `// TODO: address in follow-up after 0.18 upgrade lands` and a separate follow-up issue/PR.

**Step 3: Final manual smoke test**

Run: `bevy run`. Walk: splash → title → new game → gameplay → all four content views → pause → quit. Save mid-game; relaunch; load; play 60s.

**Step 4: Capture screenshots**

For the PR description, screenshot:
- Title screen
- One gameplay content view (Characters or Dashboard)
- Successful load of pre-upgrade autosave (e.g. screenshot of the gameplay state restored)

**Step 5: Commit any final touch-ups (only if needed)**

Pattern: `fix(0.18): <symptom>`.

---

## Task 17: Open the PR

**Step 1: Push the branch**

```bash
git push -u origin chore/bevy-0-18-upgrade
```

**Step 2: Open PR**

```bash
gh pr create --title "chore: upgrade to Bevy 0.18" --body "$(cat <<'EOF'
## Summary
- Bumps `bevy` 0.17.3 → 0.18 and `bevy_immediate` 0.4 → 0.5.
- Applies 0.17 → 0.18 migration touch-points: `BorderRadius` on `Node`, `LineHeight` on text, `set_if_neq` for idempotent transitions, observer immutability.
- Preserves loadability of pre-upgrade autosave (hard acceptance criterion).
- Strict scope: no other dep bumps, no refactors. Precondition for the bevy_declarative UI port.

## Test plan
- [ ] `cargo build` (debug, release, wasm32) all succeed
- [ ] `cargo clippy --all-targets -- -D warnings` clean
- [ ] `cargo test` green
- [ ] `bevy run` boots; all screens reachable; no panics
- [ ] Pre-upgrade `autosave.ron` loads cleanly into post-upgrade build
- [ ] Autosave roundtrip works post-upgrade
- [ ] `bevy run --target web` boots in browser

Design: docs/plans/2026-04-26-bevy-0-18-upgrade-design.md

🤖 Generated with [Claude Code](https://claude.com/claude-code)
EOF
)"
```

**Step 3: Paste smoke-test screenshots** as PR comments using `gh pr comment` or the GitHub web UI.

---

## Rollback procedure

If any post-merge symptom proves the upgrade broke production state:

```bash
git revert <merge-commit-sha>
```

Open a follow-up issue capturing the failure. The next attempt is a fresh PR off `main`.

---

## Notes for the implementer

- **Don't fight the design.** Strict scope means: if a refactor seems "obviously right," it doesn't belong in this PR. Note it as a follow-up.
- **Per-touch-point commits.** Reviewers can bisect by touch-point. A single mega-commit "fix all 0.18 issues" defeats this.
- **Defer don't fix** for `bevy_immediate` 0.5 widget drift if costly. The Phase A UI port replaces these widgets. `unimplemented!()` with a pointer to Phase A is a legitimate stopping point in a `theme/` widget.
- **Stop and report** if the upgrade reveals a touch-point not in this plan. Add it to the design doc as a follow-up addendum and surface for review before fixing.
- **The autosave precondition is real.** If you skip backing up `assets/saves/autosave.ron` before Task 1 and Task 13 fails, you cannot retest. Don't.
