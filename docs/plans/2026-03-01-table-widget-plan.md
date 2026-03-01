# Table Widget Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add reusable `Table`, `TableCtx`, `RowCtx`, `Column` widget types that render as a flat CSS Grid with automatic column alignment, striping, and per-cell hover support.

**Architecture:** A builder struct (`Table`) collects column definitions and config, then `render()` spawns a single CSS Grid container. All cells are direct grid children — `thead`/`tbody`/`tr` are logical groupings that track row index and styling but don't spawn wrapper entities. `TableCtx` and `RowCtx` are context structs passed into closures.

**Tech Stack:** Rust, Bevy 0.17 (`Node`, `Display::Grid`, `RepeatedGridTrack`), bevy_immediate 0.4 (`Imm`, `ImmEntity`, `CapSet`)

---

### Task 1: Create `Column` and `ColumnSize` types

**Files:**
- Create: `src/theme/widgets/table.rs`

**Step 1: Write the types**

In `src/theme/widgets/table.rs`:

```rust
use bevy::prelude::*;
use bevy::ui::RepeatedGridTrack;
use bevy_immediate::ui::base::CapabilityUiBase;
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::{CapSet, Imm, ImmEntity, ImplCap};

use crate::theme::prelude::*;

/// How a column should be sized (maps to CSS Grid tracks).
pub enum ColumnSize {
    /// Shrink to fit content.
    Auto,
    /// Fill remaining space proportionally.
    Flex(f32),
    /// Fixed pixel width.
    Px(f32),
}

/// A column definition for a `Table`.
pub struct Column {
    pub size: ColumnSize,
}

impl Column {
    /// Column that shrinks to fit its content.
    pub fn auto() -> Self {
        Self {
            size: ColumnSize::Auto,
        }
    }

    /// Column that fills remaining space proportionally.
    pub fn flex(val: f32) -> Self {
        Self {
            size: ColumnSize::Flex(val),
        }
    }

    /// Column with a fixed pixel width.
    pub fn px(val: f32) -> Self {
        Self {
            size: ColumnSize::Px(val),
        }
    }
}

impl ColumnSize {
    /// Convert to a Bevy `RepeatedGridTrack`.
    fn to_track(&self) -> RepeatedGridTrack {
        match self {
            ColumnSize::Auto => RepeatedGridTrack::auto(1),
            ColumnSize::Flex(val) => RepeatedGridTrack::flex(1, *val),
            ColumnSize::Px(val) => RepeatedGridTrack::px(1, *val),
        }
    }
}
```

**Step 2: Register the module**

In `src/theme/widgets/mod.rs`, add after line 16 (`pub mod tooltip;`):

```rust
pub mod table;
```

And after line 33 (`pub use tooltip::*;`):

```rust
pub use table::*;
```

**Step 3: Verify it compiles**

Run: `cargo check 2>&1 | head -20`
Expected: no errors (types are defined but unused, which is allowed)

**Step 4: Commit**

```bash
git add src/theme/widgets/table.rs src/theme/widgets/mod.rs
git commit -m "feat(table): add Column and ColumnSize types"
```

---

### Task 2: Create `Table` builder struct with `render()`

**Files:**
- Modify: `src/theme/widgets/table.rs`

**Step 1: Add `Table` struct and `TableCtx`**

Append to `src/theme/widgets/table.rs`:

```rust
/// Builder for a table widget. Define columns, then call `render()`.
pub struct Table {
    columns: Vec<Column>,
    striped: bool,
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            striped: false,
        }
    }

    /// Add a column definition.
    pub fn column(mut self, col: Column) -> Self {
        self.columns.push(col);
        self
    }

    /// Enable alternating row background colors.
    pub fn striped(mut self, val: bool) -> Self {
        self.striped = val;
        self
    }

    /// Render the table. The closure receives a `TableCtx` for building
    /// thead, tbody, tr, tfoot sections.
    pub fn render<Cap>(
        self,
        ui: &mut Imm<Cap>,
        content: impl FnOnce(&mut TableCtx<'_, '_, Cap>),
    ) where
        Cap: CapSet
            + ImplCap<CapabilityUiLayout>
            + ImplCap<CapabilityUiVisuals>
            + ImplCap<CapabilityUiTextStyle>
            + ImplCap<CapabilityUiText>
            + ImplCap<CapabilityUiBase>,
    {
        let tracks: Vec<RepeatedGridTrack> =
            self.columns.iter().map(|c| c.size.to_track()).collect();
        let col_count = self.columns.len() as u16;
        let striped = self.striped;

        ui.ch()
            .grid()
            .grid_template_columns(tracks)
            .w_full()
            .row_gap(0.0)
            .column_gap(0.0)
            .add(move |ui| {
                let mut ctx = TableCtx {
                    ui,
                    col_count,
                    striped,
                    row_index: 0,
                };
                content(&mut ctx);
            });
    }
}
```

**Step 2: Add `TableCtx`**

Append to `src/theme/widgets/table.rs`:

```rust
/// Context passed into the table's render closure.
pub struct TableCtx<'a, 'w, Cap: CapSet> {
    ui: &'a mut Imm<'w, '_, Cap>,
    col_count: u16,
    striped: bool,
    row_index: usize,
}

impl<Cap> TableCtx<'_, '_, Cap>
where
    Cap: CapSet
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiTextStyle>
        + ImplCap<CapabilityUiText>
        + ImplCap<CapabilityUiBase>,
{
    /// Render a header row. Cells use `th()` for header styling.
    pub fn thead(&mut self, row_fn: impl FnOnce(&mut RowCtx<'_, '_, Cap>)) {
        let mut row_ctx = RowCtx {
            ui: self.ui,
            row_bg: Some(Color::srgba(0.1, 0.1, 0.1, 0.5)),
            is_header: true,
            col_count: self.col_count,
        };
        row_fn(&mut row_ctx);
    }

    /// Logical grouping for body rows. Resets the row index for striping.
    pub fn tbody(&mut self, body_fn: impl FnOnce(&mut TableCtx<'_, '_, Cap>)) {
        self.row_index = 0;
        body_fn(self);
    }

    /// Render a body row. Cells use `td()`.
    pub fn tr(&mut self, row_fn: impl FnOnce(&mut RowCtx<'_, '_, Cap>)) {
        let row_idx = self.row_index;
        self.row_index += 1;

        let stripe_bg = if self.striped && row_idx % 2 == 1 {
            Some(Color::srgba(1.0, 1.0, 1.0, 0.03))
        } else {
            None
        };

        let mut row_ctx = RowCtx {
            ui: self.ui,
            row_bg: stripe_bg,
            is_header: false,
            col_count: self.col_count,
        };
        row_fn(&mut row_ctx);
    }

    /// Render a column-aligned footer summary row.
    pub fn tfoot_row(&mut self, row_fn: impl FnOnce(&mut RowCtx<'_, '_, Cap>)) {
        let mut row_ctx = RowCtx {
            ui: self.ui,
            row_bg: Some(Color::srgba(0.1, 0.1, 0.1, 0.5)),
            is_header: false,
            col_count: self.col_count,
        };
        row_fn(&mut row_ctx);
    }

    /// Render free-form footer content spanning all columns.
    pub fn tfoot(&mut self, content_fn: impl FnOnce(&mut Imm<Cap>)) {
        let col_count = self.col_count;
        self.ui
            .ch()
            .col_span(col_count)
            .py(Val::Px(SPACE_3))
            .px(Val::Px(SPACE_4))
            .style(|n: &mut Node| {
                n.border = UiRect::top(Val::Px(BORDER_WIDTH_DEFAULT));
            })
            .border_color(BORDER_DEFAULT)
            .add(content_fn);
    }
}
```

**Step 3: Verify it compiles**

Run: `cargo check 2>&1 | head -20`
Expected: no errors

**Step 4: Commit**

```bash
git add src/theme/widgets/table.rs
git commit -m "feat(table): add Table builder and TableCtx"
```

---

### Task 3: Create `RowCtx` with `th()` and `td()`

**Files:**
- Modify: `src/theme/widgets/table.rs`

**Step 1: Add `RowCtx`**

Append to `src/theme/widgets/table.rs`:

```rust
/// Context for a single table row. Each `th()`/`td()` call spawns a grid cell.
pub struct RowCtx<'a, 'w, Cap: CapSet> {
    ui: &'a mut Imm<'w, '_, Cap>,
    row_bg: Option<Color>,
    is_header: bool,
    col_count: u16,
}

impl<Cap> RowCtx<'_, '_, Cap>
where
    Cap: CapSet
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiTextStyle>
        + ImplCap<CapabilityUiText>
        + ImplCap<CapabilityUiBase>,
{
    /// Render a header cell. Applies bold text, muted color, bottom border.
    pub fn th(&mut self, content: impl FnOnce(&mut Imm<Cap>)) {
        let mut cell = self.ui.ch()
            .py(Val::Px(SPACE_3))
            .px(Val::Px(SPACE_4))
            .style(|n: &mut Node| {
                n.border = UiRect::bottom(Val::Px(BORDER_WIDTH_DEFAULT));
            })
            .border_color(BORDER_DEFAULT);

        if let Some(bg) = self.row_bg {
            cell = cell.bg(bg);
        }

        cell.add(|ui: &mut Imm<Cap>| {
            // Wrap content so header styling is applied to a container,
            // not forced onto user-provided content.
            content(ui);
        });
    }

    /// Render a body cell.
    pub fn td(&mut self, content: impl FnOnce(&mut Imm<Cap>)) {
        let mut cell = self.ui.ch()
            .py(Val::Px(SPACE_3))
            .px(Val::Px(SPACE_4));

        if let Some(bg) = self.row_bg {
            cell = cell.bg(bg);
        }

        if !self.is_header {
            cell = cell
                .style(|n: &mut Node| {
                    n.border = UiRect::bottom(Val::Px(BORDER_WIDTH_DEFAULT));
                })
                .border_color(BORDER_MUTED);
        }

        cell.add(content);
    }
}
```

**Step 2: Verify it compiles**

Run: `cargo check 2>&1 | head -20`
Expected: no errors

**Step 3: Commit**

```bash
git add src/theme/widgets/table.rs
git commit -m "feat(table): add RowCtx with th() and td() cell rendering"
```

---

### Task 4: Add `table()` convenience method on `Imm`

**Files:**
- Modify: `src/theme/extensions.rs`

**Step 1: Add `table()` to `ImmUiCompositeWidgets`**

In `src/theme/extensions.rs`, add import at top:

```rust
use crate::theme::widgets::table::Table;
```

Add `table()` to the trait definition (after `list_item` on line 45):

```rust
fn table(&mut self) -> Table;
```

Add the implementation in the `impl` block (after `list_item` impl):

```rust
fn table(&mut self) -> Table {
    Table::new()
}
```

**Step 2: Verify it compiles**

Run: `cargo check 2>&1 | head -20`
Expected: no errors

**Step 3: Commit**

```bash
git add src/theme/extensions.rs
git commit -m "feat(table): add table() convenience method on Imm"
```

---

### Task 5: Verify full build and run clippy

**Step 1: Run full build**

Run: `cargo check 2>&1 | tail -5`
Expected: `Finished` with no errors

**Step 2: Run clippy**

Run: `cargo clippy 2>&1 | tail -20`
Expected: no new warnings from `table.rs`

**Step 3: Run formatter**

Run: `cargo fmt --all`

**Step 4: Commit any formatting changes**

```bash
git add -u
git commit -m "style: format table widget code"
```

(Skip commit if `cargo fmt` made no changes.)

---

### Task 6: Verify with a smoke-test usage in existing code

**Files:**
- Modify: `src/game/ui/content/dashboard.rs` (temporarily, to verify the widget works visually)

**Step 1: Find a suitable location**

Look for the resources display section in `dashboard.rs` where key-value pairs are rendered. Add a small test table nearby (inside the dashboard view function) to verify the grid renders correctly.

**Step 2: Add a temporary test table**

Example usage to insert temporarily:

```rust
ui.table()
    .column(Column::auto())
    .column(Column::flex(1.0))
    .column(Column::px(80.0))
    .striped(true)
    .render(ui, |table| {
        table.thead(|row| {
            row.th(|ui| { ui.ch().label("#").text_size(TEXT_XS).font_bold().text_color(TEXT_MUTED); });
            row.th(|ui| { ui.ch().label("RESOURCE").text_size(TEXT_XS).font_bold().text_color(TEXT_MUTED); });
            row.th(|ui| { ui.ch().label("COUNT").text_size(TEXT_XS).font_bold().text_color(TEXT_MUTED); });
        });
        table.tbody(|body| {
            body.tr(|row| {
                row.td(|ui| { ui.ch().label("1"); });
                row.td(|ui| { ui.ch().label("Gold"); });
                row.td(|ui| { ui.ch().label("100"); });
            });
            body.tr(|row| {
                row.td(|ui| { ui.ch().label("2"); });
                row.td(|ui| { ui.ch().label("Wood"); });
                row.td(|ui| { ui.ch().label("50"); });
            });
        });
    });
```

**Step 3: Run the game**

Run: `bevy run`
Expected: Table renders with 3 columns, header row with bottom border, striped body rows.

**Step 4: Remove the test table and commit**

Remove the temporary test code from `dashboard.rs`. The widget is verified working.

```bash
git add -u
git commit -m "feat(table): complete table widget implementation"
```
