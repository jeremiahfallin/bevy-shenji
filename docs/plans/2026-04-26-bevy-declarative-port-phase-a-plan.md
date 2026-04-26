# bevy_declarative UI Port — Phase A (Foundations) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace shenji's `bevy_immediate`-based design system with a `bevy_declarative`-backed `src/ui/` module. Phase A delivers the foundation: dependency wiring, design tokens, presets, stateless widgets, ScrollView, and stateful widgets needed by simple screens. No game screens are migrated in this plan — that is Phase B.

**Architecture:** New `src/ui/` module mirrors `bevy_declarative`'s shape. Stateless visual widgets are free functions returning configured `Div`/`Text`. Stateful interactive widgets are typed builder structs implementing `Element + Styled`, with runtime state in `#[derive(Component)]` and behaviors driven by Bevy events. `bevy_immediate` and `src/theme/` remain in place during Phase A — both UI systems coexist; nothing in `src/screens/` or `src/game/ui/` changes here.

**Tech Stack:** Rust 2024, Bevy 0.18, `bevy_declarative` 0.1 (path dep at `C:\Users\bullf\dev\games\bevy_declarative`).

**Reference design:** [docs/plans/2026-04-25-bevy-declarative-port-design.md](2026-04-25-bevy-declarative-port-design.md).

**Preconditions before starting Task 1:**
- Bevy upgrade from 0.17.3 → 0.18 is **already merged to `main`**. `cargo build` succeeds against 0.18. This plan does not touch the upgrade itself.
- Working tree is clean. New feature work happens on a branch off the post-0.18 `main`.
- `C:\Users\bullf\dev\games\bevy_declarative` exists locally and `cargo build` succeeds inside it on Bevy 0.18.

**Conventions used throughout the plan:**
- Each task is one PR. PRs are reviewable and revertable independently.
- Each task uses TDD: failing test first, minimal implementation, passing test, commit.
- Run `cargo fmt --all && cargo clippy --all-targets -- -D warnings && cargo test` before every commit.
- Commits are conventional: `feat(ui): …`, `test(ui): …`, `chore(ui): …`.
- Helper test imports for builder-only assertions: `use bevy::prelude::*; use bevy_declarative::prelude::*;`.
- Helper test pattern for spawn-time assertions: build a headless `App` with `MinimalPlugins`, call `commands.spawn(...)` via the builder's `.spawn(&mut commands)`, run one update, query the world.

---

## Task 1: Add `bevy_declarative` dependency and register plugin

**Files:**
- Modify: `Cargo.toml` (add path dep, between `bevy_immediate` and `lucide-icons`)
- Modify: `src/main.rs` (add `BevyDeclarativePlugin` registration, immediately after `BevyImmediatePlugin`)

**Step 1: Edit `Cargo.toml`**

Add line 10 (after `bevy_immediate` line):

```toml
bevy_declarative = { path = "../bevy_declarative" }
```

**Step 2: Edit `src/main.rs`**

After line 22 (`use bevy_immediate::BevyImmediatePlugin;`), add:

```rust
use bevy_declarative::BevyDeclarativePlugin;
```

After line 52 (the `.add_plugins(BevyImmediatePlugin::<AppCaps>::default());` line), add `.add_plugins(BevyDeclarativePlugin)`. The chain becomes:

```rust
        .add_plugins(BevyImmediatePlugin::<AppCaps>::default())
        .add_plugins(BevyDeclarativePlugin);
```

**Step 3: Verify it compiles**

Run: `cargo build`
Expected: builds clean, no warnings about `bevy_declarative` being unused.

**Step 4: Verify it runs**

Run: `bevy run`
Expected: game launches to splash screen identically to before. No visual change. Quit with `Esc` or window close.

**Step 5: Commit**

```bash
git add Cargo.toml Cargo.lock src/main.rs
git commit -m "chore(ui): add bevy_declarative path dep and register plugin"
```

---

## Task 2: Create `src/ui/` module skeleton

**Files:**
- Create: `src/ui/mod.rs`
- Create: `src/ui/prelude.rs`
- Create: `src/ui/tokens/mod.rs` (empty stub)
- Create: `src/ui/presets/mod.rs` (empty stub)
- Create: `src/ui/widgets/mod.rs` (empty stub)
- Create: `src/ui/components/mod.rs` (empty stub)
- Create: `src/ui/behaviors/mod.rs` (empty stub)
- Modify: `src/main.rs` (register `ui::plugin`)

**Step 1: Create `src/ui/mod.rs`**

```rust
//! Shenji's design system, built on `bevy_declarative`.
//!
//! Replaces the older `bevy_immediate`-based `src/theme/` module. Both
//! coexist during the migration; this module owns the post-port UI.

#![allow(dead_code)]

pub mod behaviors;
pub mod components;
pub mod presets;
pub mod prelude;
pub mod tokens;
pub mod widgets;

use bevy::prelude::*;

pub fn plugin(_app: &mut App) {
    // Submodule plugins register themselves here as they are implemented.
}
```

**Step 2: Create empty submodule stubs**

Each of `src/ui/{tokens,presets,widgets,components,behaviors}/mod.rs` is a single line:

```rust
//! Stub — populated by subsequent plan tasks.
```

**Step 3: Create `src/ui/prelude.rs`**

```rust
//! One-stop import for screens consuming the new UI.
//!
//! Usage: `use crate::ui::prelude::*;`

pub use bevy_declarative::prelude::*;
```

**Step 4: Register `ui::plugin` in `main.rs`**

In the `.add_plugins((...))` block at lines 55–66, add `ui::plugin,` immediately after `theme::plugin,`. Add `mod ui;` next to `mod theme;` near the top of the file.

**Step 5: Verify compilation**

Run: `cargo build`
Expected: clean build. No unused-module warnings (each `mod.rs` declares its submodules).

**Step 6: Commit**

```bash
git add src/ui src/main.rs
git commit -m "feat(ui): scaffold src/ui module with empty submodules"
```

---

## Task 3: Port palette tokens

**Files:**
- Create: `src/ui/tokens/palette.rs`
- Create: `tests/ui_palette_parity.rs`
- Modify: `src/ui/tokens/mod.rs`
- Modify: `src/ui/prelude.rs`

**Step 1: Write the failing parity test**

Create `tests/ui_palette_parity.rs`:

```rust
//! Asserts that ported palette constants match `src/theme/styles/palette.rs`
//! exactly. Removed when `src/theme/` is deleted in Phase C.

use shenji::theme::styles::palette as old;
use shenji::ui::tokens::palette as new;

#[test]
fn primary_constants_match() {
    assert_eq!(new::PRIMARY_400, old::PRIMARY_400);
    assert_eq!(new::PRIMARY_500, old::PRIMARY_500);
    assert_eq!(new::PRIMARY_600, old::PRIMARY_600);
    assert_eq!(new::PRIMARY_700, old::PRIMARY_700);
}

#[test]
fn gray_constants_match() {
    assert_eq!(new::GRAY_50, old::GRAY_50);
    assert_eq!(new::GRAY_900, old::GRAY_900);
    assert_eq!(new::GRAY_950, old::GRAY_950);
}

#[test]
fn semantic_text_constants_match() {
    assert_eq!(new::TEXT_PRIMARY, old::TEXT_PRIMARY);
    assert_eq!(new::TEXT_SECONDARY, old::TEXT_SECONDARY);
    assert_eq!(new::TEXT_MUTED, old::TEXT_MUTED);
}

#[test]
fn surface_constants_match() {
    assert_eq!(new::SURFACE_BASE, old::SURFACE_BASE);
    assert_eq!(new::SURFACE_RAISED, old::SURFACE_RAISED);
}
```

For this to compile, `shenji` must expose its modules from a `lib.rs`. **If `src/lib.rs` does not yet exist, create it as part of this task** with:

```rust
pub mod theme;
pub mod ui;
```

Then add `[lib] name = "shenji" path = "src/lib.rs"` to `Cargo.toml` if needed (only if integration tests don't already pick it up).

**Step 2: Run the test to verify it fails**

Run: `cargo test --test ui_palette_parity`
Expected: compile error — `ui::tokens::palette` not found.

**Step 3: Implement `src/ui/tokens/palette.rs`**

Copy `src/theme/styles/palette.rs` verbatim into `src/ui/tokens/palette.rs`. The file is data-only; no `bevy_immediate` references to remove.

**Step 4: Wire the module**

In `src/ui/tokens/mod.rs`:

```rust
pub mod palette;
```

In `src/ui/prelude.rs`, append:

```rust
pub use crate::ui::tokens::palette::*;
```

**Step 5: Re-run the test**

Run: `cargo test --test ui_palette_parity`
Expected: 4 tests pass.

**Step 6: Commit**

```bash
git add src/ui/tokens/palette.rs src/ui/tokens/mod.rs src/ui/prelude.rs tests/ui_palette_parity.rs src/lib.rs Cargo.toml
git commit -m "feat(ui): port palette tokens with parity test against theme"
```

---

## Task 4: Port spacing tokens

**Files:**
- Create: `src/ui/tokens/spacing.rs`
- Create: `tests/ui_spacing_parity.rs`
- Modify: `src/ui/tokens/mod.rs`
- Modify: `src/ui/prelude.rs`

**Step 1: Write the failing test**

`tests/ui_spacing_parity.rs`:

```rust
use shenji::theme::styles::spacing as old;
use shenji::ui::tokens::spacing as new;

#[test]
fn spacing_scale_matches() {
    assert_eq!(new::SPACE_0, old::SPACE_0);
    assert_eq!(new::SPACE_1, old::SPACE_1);
    assert_eq!(new::SPACE_2, old::SPACE_2);
    assert_eq!(new::SPACE_4, old::SPACE_4);
    assert_eq!(new::SPACE_8, old::SPACE_8);
    assert_eq!(new::SPACE_24, old::SPACE_24);
}
```

**Step 2: Run, expect fail**

Run: `cargo test --test ui_spacing_parity`
Expected: compile error.

**Step 3: Implement**

Copy `src/theme/styles/spacing.rs` verbatim into `src/ui/tokens/spacing.rs`.

**Step 4: Wire**

`src/ui/tokens/mod.rs`: add `pub mod spacing;`.
`src/ui/prelude.rs`: append `pub use crate::ui::tokens::spacing::*;`.

**Step 5: Re-run**

Run: `cargo test --test ui_spacing_parity`
Expected: pass.

**Step 6: Commit**

```bash
git add src/ui/tokens/spacing.rs src/ui/tokens/mod.rs src/ui/prelude.rs tests/ui_spacing_parity.rs
git commit -m "feat(ui): port spacing tokens with parity test"
```

---

## Task 5: Port typography tokens

**Files:**
- Create: `src/ui/tokens/typography.rs`
- Create: `tests/ui_typography_parity.rs`
- Modify: `src/ui/tokens/mod.rs`
- Modify: `src/ui/prelude.rs`

**Step 1: Write the failing test**

```rust
use shenji::theme::styles::typography as old;
use shenji::ui::tokens::typography as new;

#[test]
fn font_size_scale_matches() {
    assert_eq!(new::TEXT_XS, old::TEXT_XS);
    assert_eq!(new::TEXT_BASE, old::TEXT_BASE);
    assert_eq!(new::TEXT_3XL, old::TEXT_3XL);
    assert_eq!(new::TEXT_5XL, old::TEXT_5XL);
}

#[test]
fn line_heights_match() {
    assert_eq!(new::LEADING_TIGHT, old::LEADING_TIGHT);
    assert_eq!(new::LEADING_NORMAL, old::LEADING_NORMAL);
}
```

**Step 2: Run, expect fail.** `cargo test --test ui_typography_parity`.

**Step 3: Implement `src/ui/tokens/typography.rs`**

Copy **only the constants** from `src/theme/styles/typography.rs` (lines 1–18). Do **not** copy the `style_*` functions — those use `ImmEntity` and are replaced by the `presets/` module in later tasks.

```rust
//! Font-size and line-height scales. Preset functions live in
//! `crate::ui::presets::text`.

pub const TEXT_XS: f32 = 12.0;
pub const TEXT_SM: f32 = 14.0;
pub const TEXT_BASE: f32 = 16.0;
pub const TEXT_LG: f32 = 18.0;
pub const TEXT_XL: f32 = 20.0;
pub const TEXT_2XL: f32 = 24.0;
pub const TEXT_3XL: f32 = 30.0;
pub const TEXT_4XL: f32 = 48.0;
pub const TEXT_5XL: f32 = 64.0;

pub const LEADING_TIGHT: f32 = 1.25;
pub const LEADING_NORMAL: f32 = 1.5;
pub const LEADING_RELAXED: f32 = 1.75;
```

**Step 4: Wire**

`src/ui/tokens/mod.rs`: add `pub mod typography;`.
`src/ui/prelude.rs`: append `pub use crate::ui::tokens::typography::*;`.

**Step 5: Re-run.** Expected: pass.

**Step 6: Commit**

```bash
git add src/ui/tokens/typography.rs src/ui/tokens/mod.rs src/ui/prelude.rs tests/ui_typography_parity.rs
git commit -m "feat(ui): port typography scale constants with parity test"
```

---

## Task 6: Implement `presets/buttons` (primary, ghost, danger)

**Files:**
- Create: `src/ui/presets/buttons.rs`
- Create: `tests/ui_presets_buttons.rs`
- Modify: `src/ui/presets/mod.rs`
- Modify: `src/ui/prelude.rs`

**Step 1: Write the failing builder-construction tests**

`tests/ui_presets_buttons.rs`:

```rust
use bevy::prelude::*;
use bevy_declarative::prelude::*;
use shenji::ui::prelude::*;

#[test]
fn btn_primary_has_primary_500_bg() {
    let div: Div = btn_primary("Play");
    // Internal `bg` field is private; assert by spawning and querying:
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let mut world = app.world_mut();
    let mut commands = world.commands();
    div.spawn(&mut commands);
    world.flush();
    let mut q = world.query::<&BackgroundColor>();
    let mut found = false;
    for bg in q.iter(world) {
        if bg.0 == PRIMARY_500 { found = true; }
    }
    assert!(found, "btn_primary should set BackgroundColor to PRIMARY_500");
}

#[test]
fn btn_ghost_has_no_solid_bg() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    let mut commands = world.commands();
    btn_ghost("Cancel").spawn(&mut commands);
    world.flush();
    let mut q = world.query::<&BackgroundColor>();
    for bg in q.iter(world) {
        assert_ne!(bg.0, PRIMARY_500, "ghost variant must not use primary fill");
    }
}
```

**Step 2: Run, expect fail.** `cargo test --test ui_presets_buttons`.

**Step 3: Implement `src/ui/presets/buttons.rs`**

```rust
use bevy_declarative::prelude::*;

use crate::ui::tokens::palette::*;
use crate::ui::tokens::spacing::*;

/// Filled primary action button. Used for "Start", "Save", "Confirm" etc.
pub fn btn_primary(label: impl Into<String>) -> Div {
    div()
        .flex().row().items_center().justify_center()
        .px(px(SPACE_4)).py(px(SPACE_2))
        .bg(PRIMARY_500)
        .rounded(px(SPACE_1))
        .child(text(label).color(TEXT_PRIMARY))
}

/// Ghost button — outlined, transparent fill. Used for secondary actions.
pub fn btn_ghost(label: impl Into<String>) -> Div {
    div()
        .flex().row().items_center().justify_center()
        .px(px(SPACE_4)).py(px(SPACE_2))
        .rounded(px(SPACE_1))
        .child(text(label).color(TEXT_PRIMARY))
}

/// Danger button — destructive actions like "Delete", "Quit without saving".
pub fn btn_danger(label: impl Into<String>) -> Div {
    div()
        .flex().row().items_center().justify_center()
        .px(px(SPACE_4)).py(px(SPACE_2))
        .bg(ERROR_500)
        .rounded(px(SPACE_1))
        .child(text(label).color(TEXT_PRIMARY))
}
```

If method names like `px(...)`, `py(...)`, `rounded(...)` don't exist in `bevy_declarative::Styled`, check the source at `C:\Users\bullf\dev\games\bevy_declarative\src\style\styled.rs` and use whatever the closest matching API is. **Do not modify `bevy_declarative`** — adapt the preset to use what's available, or fall back to `.padding(...)` + raw `Val::Px`.

**Step 4: Wire**

`src/ui/presets/mod.rs`: add `pub mod buttons;`.
`src/ui/prelude.rs`: append `pub use crate::ui::presets::buttons::*;`.

**Step 5: Re-run tests.** Expected: 2 pass.

**Step 6: Commit**

```bash
git add src/ui/presets/buttons.rs src/ui/presets/mod.rs src/ui/prelude.rs tests/ui_presets_buttons.rs
git commit -m "feat(ui): add btn_primary, btn_ghost, btn_danger presets"
```

---

## Task 7: Implement `presets/containers` (panel, card, surface)

**Files:** `src/ui/presets/containers.rs`, `tests/ui_presets_containers.rs`, wire as before.

**Step 1: Failing test**

```rust
use bevy::prelude::*;
use bevy_declarative::prelude::*;
use shenji::ui::prelude::*;

#[test]
fn panel_uses_surface_raised() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    let mut commands = world.commands();
    panel().child(text("hi")).spawn(&mut commands);
    world.flush();
    let mut q = world.query::<&BackgroundColor>();
    assert!(q.iter(world).any(|bg| bg.0 == SURFACE_RAISED));
}
```

**Step 2: Run, expect fail.**

**Step 3: Implement**

```rust
use bevy_declarative::prelude::*;
use crate::ui::tokens::palette::*;
use crate::ui::tokens::spacing::*;

pub fn panel() -> Div {
    div().flex().col().p(px(SPACE_4)).bg(SURFACE_RAISED).rounded(px(SPACE_2))
}

pub fn card() -> Div {
    div().flex().col().p(px(SPACE_3)).bg(SURFACE_RAISED).rounded(px(SPACE_1))
}

pub fn surface() -> Div {
    div().flex().col().p(px(SPACE_5)).bg(SURFACE_BASE)
}
```

**Step 4: Wire** (`src/ui/presets/mod.rs`, `src/ui/prelude.rs`).

**Step 5: Re-run.** Expected: pass.

**Step 6: Commit**

```bash
git commit -m "feat(ui): add panel, card, surface container presets"
```

---

## Task 8: Implement `presets/text` (heading_1/2/3, body, caption, overline)

**Files:** `src/ui/presets/text.rs`, `tests/ui_presets_text.rs`, wire.

**Step 1: Failing test**

```rust
use bevy::prelude::*;
use bevy_declarative::prelude::*;
use shenji::ui::prelude::*;

#[test]
fn heading_1_renders_with_3xl_font_size() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    let mut commands = world.commands();
    heading_1("Title").spawn(&mut commands);
    world.flush();
    let mut q = world.query::<&TextFont>();
    let sizes: Vec<f32> = q.iter(world).map(|f| f.font_size).collect();
    assert!(sizes.iter().any(|&s| (s - TEXT_3XL).abs() < f32::EPSILON));
}
```

(Adjust component names to whatever `bevy_declarative` actually emits — read its `text.rs` element to confirm. The test is the contract: heading_1 produces text whose font size is `TEXT_3XL`.)

**Step 2: Run, expect fail.**

**Step 3: Implement**

```rust
use bevy_declarative::prelude::*;
use crate::ui::tokens::palette::*;
use crate::ui::tokens::typography::*;

pub fn heading_1(s: impl Into<String>) -> TextEl {
    text(s).font_size(TEXT_3XL).color(TEXT_PRIMARY)
}
pub fn heading_2(s: impl Into<String>) -> TextEl {
    text(s).font_size(TEXT_2XL).color(TEXT_PRIMARY)
}
pub fn heading_3(s: impl Into<String>) -> TextEl {
    text(s).font_size(TEXT_XL).color(TEXT_PRIMARY)
}
pub fn body(s: impl Into<String>) -> TextEl {
    text(s).font_size(TEXT_BASE).color(TEXT_SECONDARY)
}
pub fn body_sm(s: impl Into<String>) -> TextEl {
    text(s).font_size(TEXT_SM).color(TEXT_SECONDARY)
}
pub fn caption(s: impl Into<String>) -> TextEl {
    text(s).font_size(TEXT_XS).color(TEXT_MUTED)
}
pub fn overline(s: impl Into<String>) -> TextEl {
    text(s).font_size(TEXT_XS).color(TEXT_MUTED)
}
```

If `font_size` doesn't exist on `TextEl`, check `C:\Users\bullf\dev\games\bevy_declarative\src\element\text.rs` and use the actual API. Adapt; do not modify upstream.

**Step 4: Wire.** **Step 5: Pass.** **Step 6: Commit** `feat(ui): add typography presets (heading_1..3, body, caption, overline)`.

---

## Task 9: Implement stateless widget — `badge`

**Files:** `src/ui/widgets/badge.rs`, `tests/ui_widget_badge.rs`, wire `pub mod badge;` in `src/ui/widgets/mod.rs` and re-export in prelude.

**Step 1: Failing test**

```rust
use bevy::prelude::*;
use bevy_declarative::prelude::*;
use shenji::ui::prelude::*;

#[test]
fn badge_renders_label_text() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let world = app.world_mut();
    let mut commands = world.commands();
    badge("New").spawn(&mut commands);
    world.flush();
    let mut q = world.query::<&Text>();
    let strings: Vec<String> = q.iter(world).map(|t| t.0.clone()).collect();
    assert!(strings.iter().any(|s| s == "New"));
}
```

**Step 2: Fail.** **Step 3: Implement** as a `Div` with small padding, accent bg, rounded corners, holding a `text(label)` child. **Step 4: Wire.** **Step 5: Pass.** **Step 6:** commit `feat(ui): add badge widget`.

---

## Task 10: Implement stateless widget — `divider`

Same shape as Task 9. Test: divider produces a node whose height is `1.0` (horizontal) or width is `1.0` (vertical). Provide `divider()` and `divider_vertical()`.

Commit: `feat(ui): add divider widget (horizontal and vertical)`.

---

## Task 11: Implement stateless widget — `icon`

**Files:** `src/ui/widgets/icon.rs`, `tests/ui_widget_icon.rs`.

`lucide-icons` 0.563 is already in `Cargo.toml`. Use it for glyph lookup. Implementation:

```rust
use bevy_declarative::prelude::*;
use lucide_icons::Icon;
use crate::ui::tokens::typography::TEXT_BASE;

pub fn icon(glyph: Icon) -> TextEl {
    // Lucide ships glyphs as a font; render the codepoint as a text element.
    text(glyph.to_string()).font_size(TEXT_BASE)
}
```

If `bevy_declarative`'s `text()` doesn't support a custom font handle, accept that for now — set the font in a follow-up once a screen actually requires the lucide font. The test asserts: `icon(Icon::Settings)` produces a `Text` whose content equals `Icon::Settings.to_string()`.

Commit: `feat(ui): add icon widget backed by lucide-icons`.

---

## Task 12: Implement stateless widget — `label`

Free function `label(text: &str) -> TextEl` returning `body_sm`-styled text. Test: produces a `Text` with the given content and `TEXT_SM` font size.

Commit: `feat(ui): add label widget`.

---

## Task 13: Implement stateless widget — `progress_bar`

**Files:** `src/ui/widgets/progress_bar.rs`, `tests/ui_widget_progress_bar.rs`.

API: `progress_bar(fraction: f32) -> Div` where `fraction` is clamped to `[0.0, 1.0]`.

Implementation: outer `Div` with `SURFACE_INSET` background, full width, fixed height `SPACE_2`. Inner `Div` (child) with width `Val::Percent(fraction * 100.0)`, height 100%, `PRIMARY_500` background.

**Test 1:** `progress_bar(0.5)` produces two nested divs; the inner one has `width: Val::Percent(50.0)`.
**Test 2:** `progress_bar(1.5)` clamps to 100%.
**Test 3:** `progress_bar(-0.2)` clamps to 0%.

Commit: `feat(ui): add progress_bar widget with clamped fraction`.

---

## Task 14: Implement stateless widget — `tooltip`

**Files:** `src/ui/widgets/tooltip.rs`, `tests/ui_widget_tooltip.rs`.

API: `tooltip(target: impl Element + 'static, body: impl Into<String>) -> Div`. Wraps `target` and renders the tooltip body absolutely-positioned, hidden by default. Shows on hover via a `Pointer<Over>` observer that flips a `TooltipState` component's visibility.

Because hover-show requires a behavior system, this widget is the first stateless widget that registers a plugin. Add:

```rust
pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_tooltip_visibility);
}
```

Register from `src/ui/widgets/mod.rs::plugin` (create if not present), then call `widgets::plugin` from `src/ui/mod.rs::plugin`.

**Test:** spawn a tooltip-wrapped element; assert that the tooltip body's `Visibility` defaults to `Hidden`.

Commit: `feat(ui): add tooltip widget with hover-show behavior`.

---

## Task 15: Implement `ScrollView` component (Characters bug fix lives here)

**Files:**
- Create: `src/ui/components/scroll_view.rs`
- Create: `tests/ui_scroll_view.rs`
- Create: `examples/scroll_view_test.rs` (manual verification harness)
- Modify: `src/ui/components/mod.rs`, `src/ui/prelude.rs`, `src/ui/mod.rs` (register plugin).

**API (builder struct, since it owns layout state):**

```rust
pub struct ScrollView {
    node: Node,
    children: Vec<Box<dyn Element>>,
    horizontal: bool,
    vertical: bool,
}

pub fn scroll_view() -> ScrollView {
    ScrollView {
        node: Node {
            overflow: Overflow {
                x: OverflowAxis::Scroll, // toggled below
                y: OverflowAxis::Scroll,
            },
            ..default()
        },
        children: Vec::new(),
        horizontal: false,
        vertical: true,
    }
}

impl ScrollView {
    pub fn horizontal(mut self) -> Self { self.horizontal = true; self }
    pub fn vertical(mut self, on: bool) -> Self { self.vertical = on; self }
    pub fn child(mut self, c: impl Element + 'static) -> Self {
        self.children.push(Box::new(c)); self
    }
}

impl Styled for ScrollView { fn style_mut(&mut self) -> &mut Node { &mut self.node } }

impl Element for ScrollView {
    fn spawn_with_parent(self: Box<Self>, parent: &mut ChildSpawnerCommands) {
        // Apply enabled axes:
        let mut node = self.node;
        node.overflow.x = if self.horizontal { OverflowAxis::Scroll } else { OverflowAxis::Visible };
        node.overflow.y = if self.vertical   { OverflowAxis::Scroll } else { OverflowAxis::Visible };

        parent.spawn((node, ScrollPosition::default(), ScrollView_Marker))
            .with_children(|inner| {
                // Inner content container (separate entity so scrollbar overlay can be sibling)
                // Children spawn here.
                for child in self.children {
                    child.spawn_with_parent(inner);
                }
            });
    }
}
```

**Bug-fix acceptance criteria** (test before implementation):

`tests/ui_scroll_view.rs` runs in a headless `App` with `MinimalPlugins + UiPlugin`:

```rust
#[test]
fn scrollbar_geometry_stays_inside_viewport() {
    // Spawn a ScrollView with viewport 200x200 containing 1000-wide content.
    // After one update, query the scrollbar entity's ComputedNode.
    // Assert: scrollbar's right edge <= viewport's right edge.
}

#[test]
fn scroll_offset_can_reach_content_end() {
    // Same setup. Set ScrollPosition.x to a large value (e.g. 10000).
    // After one update, assert ScrollPosition.x == content_width - viewport_width
    // (i.e. it clamps to exactly the end, not short of it).
}

#[test]
fn viewport_resize_recomputes_scrollbar() {
    // Spawn, capture initial scrollbar width.
    // Resize the parent node to half its previous width.
    // Assert scrollbar width changes proportionally on next update.
}
```

If these tests are too entangled with Bevy's UI internals to write headlessly, downgrade them to assertions on the layout policy:

- `ScrollView::horizontal()` produces a node with `overflow.x = OverflowAxis::Scroll`.
- `ScrollPosition` is inserted on the spawned root.
- Children spawn under a single inner container (not as direct children of the scrolling root) — verify by counting child entities.

The four manual acceptance criteria from the design doc remain the binding contract; this is verified via `examples/scroll_view_test.rs`:

```rust
//! Manual ScrollView verification harness.
//!
//! Run: `cargo run --example scroll_view_test`
//! Expectations (verify by eye, paste screenshot in PR):
//!   1. Scrollbar geometry stays inside viewport at all sizes.
//!   2. Scrolling reaches the last pixel of content.
//!   3. Resizing the window recomputes scrollbar size.
//!   4. Mouse wheel and drag both scroll.

use bevy::prelude::*;
use bevy_declarative::prelude::*;
use shenji::ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyDeclarativePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    div().w(px(400.0)).h(px(400.0)).bg(SURFACE_BASE)
        .child(
            scroll_view().horizontal().vertical(true)
                .child(
                    // Big content: 2000 x 2000
                    div().w(px(2000.0)).h(px(2000.0)).bg(PRIMARY_500)
                )
        )
        .spawn(&mut commands);
}
```

**Step-by-step:**

1. Write the 3 headless tests above. Run, expect compile/fail.
2. Implement `ScrollView` builder + `Element` impl. Run tests, get them green (downgrade as needed per the note).
3. Write `examples/scroll_view_test.rs`. Run `cargo run --example scroll_view_test`. Manually verify all 4 criteria. **Take a screenshot showing scroll-at-end and scrollbar-in-bounds.**
4. Wire into `src/ui/prelude.rs` (`pub use crate::ui::components::scroll_view::*;`).
5. Commit: `feat(ui): add ScrollView with viewport-bounded scrollbar`.

**If a test reveals that the bug reproduces in `bevy_declarative`'s scroll handling itself**, do not modify `bevy_declarative`. Instead, copy `bevy_declarative/src/scroll.rs`'s logic into `src/ui/components/scroll_view.rs` and adapt the calculation. Document the deviation with a comment pointing to the upstream source.

---

## Task 16: Implement stateful widget — `Checkbox`

**Files:** `src/ui/widgets/checkbox.rs`, `tests/ui_widget_checkbox.rs`.

**Step 1: Failing tests**

```rust
use bevy::prelude::*;
use bevy_declarative::prelude::*;
use shenji::ui::prelude::*;

#[test]
fn checkbox_emits_change_event_on_click() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_event::<CheckboxChanged>();
    app.add_plugins(crate::ui::widgets::checkbox::plugin);

    let world = app.world_mut();
    let mut commands = world.commands();
    checkbox(false).label("Enable").spawn(&mut commands);
    world.flush();

    let entity = world.query_filtered::<Entity, With<CheckboxState>>().single(world);

    // Simulate a click via the same observer Bevy would fire.
    world.commands().trigger_targets(Pointer::<Click>::default(), entity);
    app.update();

    let events = app.world().resource::<Events<CheckboxChanged>>();
    let mut reader = events.get_reader();
    let evs: Vec<_> = reader.read(events).collect();
    assert_eq!(evs.len(), 1);
    assert_eq!(evs[0].entity, entity);
    assert_eq!(evs[0].checked, true);
}

#[test]
fn disabled_checkbox_ignores_clicks() {
    // Same setup but checkbox(false).disabled(true).
    // Trigger click. Assert no CheckboxChanged event emitted.
}
```

**Step 2: Run, expect fail.**

**Step 3: Implement**

```rust
use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;
use bevy_declarative::prelude::*;

#[derive(Event, Debug, Clone, Copy)]
pub struct CheckboxChanged {
    pub entity: Entity,
    pub checked: bool,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct CheckboxState {
    pub checked: bool,
    pub disabled: bool,
}

pub struct Checkbox {
    node: Node,
    checked: bool,
    disabled: bool,
    label: Option<String>,
}

pub fn checkbox(initial: bool) -> Checkbox {
    Checkbox { node: Node::default(), checked: initial, disabled: false, label: None }
}

impl Checkbox {
    pub fn label(mut self, s: impl Into<String>) -> Self { self.label = Some(s.into()); self }
    pub fn disabled(mut self, d: bool) -> Self { self.disabled = d; self }
}

impl Styled for Checkbox { fn style_mut(&mut self) -> &mut Node { &mut self.node } }

impl Element for Checkbox {
    fn spawn_with_parent(self: Box<Self>, parent: &mut ChildSpawnerCommands) {
        let state = CheckboxState { checked: self.checked, disabled: self.disabled };
        let label = self.label.clone();
        parent.spawn((self.node, state)).with_children(|inner| {
            // Indicator
            let mut indicator = Node::default();
            indicator.width = Val::Px(16.0);
            indicator.height = Val::Px(16.0);
            inner.spawn((
                indicator,
                BackgroundColor(if self.checked { PRIMARY_500 } else { SURFACE_INSET }),
            ));
            if let Some(label_text) = label {
                inner.spawn((Text::new(label_text), TextFont::default()));
            }
        });
    }
}

pub fn plugin(app: &mut App) {
    app.add_event::<CheckboxChanged>();
    app.add_observer(on_checkbox_click);
}

fn on_checkbox_click(
    click: On<Pointer<Click>>,
    mut q: Query<&mut CheckboxState>,
    mut events: EventWriter<CheckboxChanged>,
) {
    let Ok(mut state) = q.get_mut(click.target()) else { return; };
    if state.disabled { return; }
    state.checked = !state.checked;
    events.write(CheckboxChanged { entity: click.target(), checked: state.checked });
}
```

Adjust component / observer API to whatever Bevy 0.18 actually exposes — `Pointer<Click>` may be `On<Pointer<Click>>` or similar. The contract is: clicking a non-disabled checkbox toggles `CheckboxState.checked` and emits `CheckboxChanged`.

**Step 4: Wire** (`src/ui/widgets/mod.rs::plugin` calls `checkbox::plugin`).
**Step 5: Pass.**
**Step 6: Commit** `feat(ui): add Checkbox widget with state and change events`.

---

## Task 17: Implement stateful widget — `Radio` (group)

Same pattern as Checkbox but with a `RadioGroup` resource/component tracking the selected value of type `T: PartialEq + Clone + Send + Sync + 'static`.

API: `radio(group_id: GroupId, value: T)` produces one option. Selection is exclusive within the same `group_id`.

Event: `RadioChanged { group: GroupId, value: T }`. Since `T` is generic, restrict to `String` for v1 (sufficient for menu options) and revisit if a screen needs typed values.

Tests: clicking option B in a group where option A is selected emits `RadioChanged` with B's value, and A's `RadioState.selected` flips to false.

Commit: `feat(ui): add Radio group widget with exclusive selection`.

---

## Task 18: Implement stateful widget — `Slider`

API: `slider(value: f32, range: RangeInclusive<f32>)`. Renders a track + thumb. Drag updates `SliderState.value` and emits `SliderChanged { entity, value }`.

Implementation hints:
- Drag handled by observers on `Pointer<DragStart>`, `Pointer<Drag>`, `Pointer<DragEnd>`.
- Convert pointer x-position relative to track width to a value in `range`.
- Emit `SliderChanged` on every `Drag` event (caller can debounce in their listener if needed).

Tests:
- `slider(0.5, 0.0..=1.0)` creates state with value 0.5.
- Manually setting `SliderState.value` to 0.75 and triggering a re-render places the thumb at 75% of track width.
- Drag from track midpoint to `track_x + track_width * 0.25` produces `SliderChanged` with value `0.75`. (Headless drag simulation may need a synthetic `Pointer<Drag>` trigger.)

Commit: `feat(ui): add Slider widget with drag support`.

---

## Task 19: Implement stateful widget — `TextInput`

API: `text_input(initial: &str)`. Renders an input field; captures keyboard when focused (clicked).

State: `TextInputState { value: String, focused: bool, cursor: usize }`.
Events: `TextInputChanged { entity, value }` per keystroke; `TextInputSubmitted { entity, value }` on Enter.

Implementation hints:
- Focus model: clicking the input sets `focused = true`. Clicking elsewhere defocuses (use a global `Pointer<Click>` observer that sets `focused = false` on all `TextInputState`s except the clicked target).
- Keyboard input: read `Res<ButtonInput<KeyCode>>` + `Events<KeyboardInput>` in an `Update` system; mutate `TextInputState.value` only when `focused`.
- For v1, support: printable ASCII, Backspace, Enter. No selection, no copy/paste, no IME.

Tests:
- Constructing `text_input("hello")` yields state with `value == "hello"`, `focused == false`.
- Typing `'a'` while focused appends `'a'` and emits `TextInputChanged`.
- Pressing Enter while focused emits `TextInputSubmitted`.

Commit: `feat(ui): add TextInput widget with keyboard input and focus`.

---

## Task 20: Implement stateful widget — `Tabs`

API: `tabs(initial_index: usize).tab(label, content).tab(label, content)…`. Renders a row of tab labels above the active tab's content.

State: `TabsState { active: usize }`.
Event: `TabsChanged { entity, active: usize }`.

Implementation:
- Each tab label is a clickable `Div` whose `Pointer<Click>` observer sets `TabsState.active` to its index and emits `TabsChanged`.
- Content rendering: use the despawn-and-respawn policy from the design — when `active` changes, despawn the content subtree and respawn with the new tab's element. Do this in an `Update` system that watches `Changed<TabsState>`.

Tests:
- `tabs(0).tab("A", div()).tab("B", div())` produces state with `active == 0`.
- Clicking the second label sets `active == 1` and emits `TabsChanged`.
- After active changes, querying child entities of the content container shows the new content (verify by tagging each tab's content with a unique marker component).

Commit: `feat(ui): add Tabs widget with despawn-and-respawn content`.

---

## Task 21: Build the `examples/ui_gallery.rs` reference and capture screenshot

**Files:**
- Create: `examples/ui_gallery.rs`
- Create: `docs/plans/2026-04-26-ui-gallery-reference.png` (committed)

**Step 1:** Write `examples/ui_gallery.rs` rendering one of each implemented widget in a vertical scroll view: `btn_primary`, `btn_ghost`, `btn_danger`, `panel`, `card`, all heading levels, `body`, `caption`, `badge`, `divider`, `divider_vertical`, `icon(Icon::Settings)`, `label`, `progress_bar(0.0)`, `progress_bar(0.5)`, `progress_bar(1.0)`, `tooltip`, `checkbox(false)`, `checkbox(true).disabled(true)`, `radio` with two options, `slider(0.5, 0..=1)`, `text_input("hello")`, `tabs(0)` with two tabs.

Group widgets by category with `heading_2` headers.

**Step 2:** Run `cargo run --example ui_gallery`. Verify visually that all widgets render plausibly. Resize the window. Click a checkbox; observe state flip. Click a tab; observe content swap. Type into the text input.

**Step 3:** Take a screenshot via `bevy-eyes-on` (or platform screenshot tool) at default window size. Save to `docs/plans/2026-04-26-ui-gallery-reference.png`.

**Step 4:** Commit:

```bash
git add examples/ui_gallery.rs docs/plans/2026-04-26-ui-gallery-reference.png
git commit -m "test(ui): add gallery example and capture reference screenshot"
```

---

## Phase A completion checklist

Run before declaring Phase A done:

- [ ] `cargo fmt --all -- --check` clean.
- [ ] `cargo clippy --all-targets -- -D warnings` clean.
- [ ] `cargo test` green (all `tests/ui_*.rs` files pass).
- [ ] `cargo run --example ui_gallery` displays every widget without panic; interactive widgets respond.
- [ ] `cargo run --example scroll_view_test` shows scrollbar inside viewport and scroll reaches end. Screenshot pasted in the merge PR.
- [ ] `bevy run` boots the actual game; all existing screens render unchanged (since no screen has migrated yet).
- [ ] `Cargo.toml` still contains `bevy_immediate` (Phase A does not remove it).
- [ ] `src/theme/` is untouched.
- [ ] Phase B planning can begin; the next plan is `docs/plans/YYYY-MM-DD-bevy-declarative-port-phase-b-plan.md` covering screen migrations 7–17.

---

## Notes for the implementer

- **`bevy_declarative` API drift.** The plan assumes specific method names (`px`, `py`, `bg`, `rounded`, `font_size`, `color`, `child`, `spawn`). If any are missing or named differently, read `C:\Users\bullf\dev\games\bevy_declarative\src\` to find the actual name and adapt the call. **Do not modify `bevy_declarative`**; do the adaptation inside `src/ui/`.
- **Observer API in Bevy 0.18.** The observer signature for `Pointer<Click>` may differ from older Bevy. Use whatever is idiomatic in 0.18 — the contract is "clicking the entity triggers the handler"; the specific function signature is implementation detail.
- **Test ergonomics.** If headless `App` testing of widgets becomes painful (e.g. layout calc requires `UiPlugin` which needs windows), downgrade those specific tests to assert on builder data structures instead and rely on the gallery for visual verification.
- **No screen edits in Phase A.** If a task seems to require touching `src/screens/` or `src/game/ui/`, stop — that work belongs to Phase B.
