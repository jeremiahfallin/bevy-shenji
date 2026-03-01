# Table Widget Design

## Summary

Add reusable table widget components (`Table`, `TableCtx`, `RowCtx`) to the theme widget system. Uses a flat CSS Grid layout so column alignment is automatic across header, body, and footer rows. Follows the builder struct + `.render()` pattern established by `List`/`ListItem`.

## Requirements

- HTML-like API: `table`, `thead`, `th`, `tbody`, `tr`, `td`, `tfoot`, `tfoot_row`
- Column widths defined upfront via `Column` defs (auto, flex, px)
- Automatic column alignment across all rows (no manual width synchronization)
- Interactive rows: hover highlighting, striping
- Footer supports both column-aligned summary rows and free-form content

## Approach: Flat CSS Grid

All cells (header, body, footer) are direct children of a single CSS Grid container. `thead`, `tbody`, `tr` are logical groupings only — they don't spawn wrapper entities. They track row index and styling, then delegate to `th()`/`td()` which spawn the actual grid cell entities.

### Data Types

```rust
pub enum ColumnSize {
    Auto,        // RepeatedGridTrack::auto(1)
    Flex(f32),   // RepeatedGridTrack::flex(1, val)
    Px(f32),     // RepeatedGridTrack::px(1, val)
}

pub struct Column {
    pub size: ColumnSize,
}

impl Column {
    pub fn auto() -> Self;
    pub fn flex(val: f32) -> Self;
    pub fn px(val: f32) -> Self;
}
```

### Core Structs

```rust
pub struct Table {
    columns: Vec<Column>,
    striped: bool,
}

impl Table {
    pub fn new() -> Self;
    pub fn column(mut self, col: Column) -> Self;
    pub fn striped(mut self, val: bool) -> Self;
    pub fn render<Cap>(self, ui: &mut Imm<Cap>, content: impl FnOnce(&mut TableCtx<Cap>));
}
```

`render()` spawns a CSS Grid container with `grid_template_columns` built from `self.columns`, then passes a `TableCtx` into the closure.

```rust
pub struct TableCtx<'a, 'w, 's, Cap> {
    ui: &'a mut Imm<'w, 's, Cap>,
    col_count: u16,
    striped: bool,
    row_index: usize,
}

impl TableCtx {
    pub fn thead(&mut self, row_fn: impl FnOnce(&mut RowCtx<Cap>));
    pub fn tbody(&mut self, body_fn: impl FnOnce(&mut TableCtx<Cap>));
    pub fn tr(&mut self, row_fn: impl FnOnce(&mut RowCtx<Cap>));
    pub fn tfoot_row(&mut self, row_fn: impl FnOnce(&mut RowCtx<Cap>));
    pub fn tfoot(&mut self, content_fn: impl FnOnce(&mut Imm<Cap>));
}
```

```rust
pub struct RowCtx<'a, 'w, 's, Cap> {
    ui: &'a mut Imm<'w, 's, Cap>,
    row_bg: Option<Color>,
}

impl RowCtx {
    pub fn th(&mut self, content: impl FnOnce(&mut Imm<Cap>));
    pub fn td(&mut self, content: impl FnOnce(&mut Imm<Cap>));
}
```

### Rendering Details

- **Grid container**: `grid_template_columns` from column defs, `row_gap(0.0)`, `w_full()`
- **`th()` cells**: bold text, bottom border separator, muted text color, smaller font
- **`td()` cells**: standard padding, optional stripe background
- **`thead()`**: resets row_index, delegates to RowCtx with header styling flag
- **`tbody()`**: passes self so `tr()` can be called in a loop
- **`tr()`**: increments row_index, computes stripe bg, creates RowCtx
- **`tfoot_row()`**: column-aligned summary row (same grid, distinct styling)
- **`tfoot()`**: free-form content that `col_span`s the full width

### Interactivity (v1)

Per-cell `Interaction` with `row_gap(0.0)` so gaps don't break hover feel. Each `td` cell in a body row reads hover state. Striping via per-cell background on even/odd rows.

### File Location

`src/theme/widgets/table.rs`, registered in `src/theme/widgets/mod.rs`.

Convenience constructor on `Imm` via `ImmUiCompositeWidgets` in `src/theme/extensions.rs`:
```rust
fn table(&mut self) -> Table { Table::new() }
```

## Usage Example

```rust
ui.table()
    .column(Column::auto())
    .column(Column::flex(1.0))
    .column(Column::px(80.0))
    .striped(true)
    .render(ui, |table| {
        table.thead(|row| {
            row.th(|ui| { ui.ch().label("#"); });
            row.th(|ui| { ui.ch().label("NAME"); });
            row.th(|ui| { ui.ch().label("STATUS"); });
        });
        table.tbody(|body| {
            for (i, item) in items.iter().enumerate() {
                body.tr(|row| {
                    row.td(|ui| { ui.ch().label(format!("{}", i + 1)); });
                    row.td(|ui| { ui.ch().label(&item.name); });
                    row.td(|ui| { ui.ch().label(&item.status); });
                });
            }
        });
        table.tfoot_row(|row| {
            row.td(|ui| {});
            row.td(|ui| { ui.ch().label(format!("Total: {}", items.len())); });
            row.td(|ui| {});
        });
    });
```
