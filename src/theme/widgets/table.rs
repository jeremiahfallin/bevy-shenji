use bevy::prelude::*;
use bevy::ui::RepeatedGridTrack;
use bevy_immediate::ui::base::CapabilityUiBase;
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::{CapSet, Imm, ImplCap};

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
        content: impl FnOnce(&mut TableCtx<'_, '_, '_, Cap>),
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
            .min_w(Val::Percent(100.0))
            .row_gap(0.0)
            .column_gap(0.0)
            .style(|n: &mut Node| {
                n.align_content = AlignContent::Start;
            })
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

/// Context passed into the table's render closure.
pub struct TableCtx<'a, 'w, 's, Cap: CapSet> {
    ui: &'a mut Imm<'w, 's, Cap>,
    col_count: u16,
    striped: bool,
    row_index: usize,
}

impl<'w, 's, Cap> TableCtx<'_, 'w, 's, Cap>
where
    Cap: CapSet
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiTextStyle>
        + ImplCap<CapabilityUiText>
        + ImplCap<CapabilityUiBase>,
{
    /// Render a header row. Cells use `th()` for header styling.
    pub fn thead(&mut self, row_fn: impl FnOnce(&mut RowCtx<'_, 'w, 's, Cap>)) {
        let mut row_ctx = RowCtx {
            ui: self.ui,
            row_bg: Some(TABLE_HEADER_BG),
            is_header: true,
        };
        row_fn(&mut row_ctx);
    }

    /// Logical grouping for body rows. Resets the row index for striping.
    pub fn tbody(&mut self, body_fn: impl FnOnce(&mut TableCtx<'_, 'w, 's, Cap>)) {
        self.row_index = 0;
        body_fn(self);
    }

    /// Render a body row. Cells use `td()`.
    pub fn tr(&mut self, row_fn: impl FnOnce(&mut RowCtx<'_, 'w, 's, Cap>)) {
        let row_idx = self.row_index;
        self.row_index += 1;

        let stripe_bg = if self.striped && row_idx % 2 == 1 {
            Some(TABLE_STRIPE_BG)
        } else {
            None
        };

        let mut row_ctx = RowCtx {
            ui: self.ui,
            row_bg: stripe_bg,
            is_header: false,
        };
        row_fn(&mut row_ctx);
    }

    /// Render a column-aligned footer summary row.
    pub fn tfoot_row(&mut self, row_fn: impl FnOnce(&mut RowCtx<'_, 'w, 's, Cap>)) {
        let mut row_ctx = RowCtx {
            ui: self.ui,
            row_bg: Some(TABLE_HEADER_BG),
            is_header: false,
        };
        row_fn(&mut row_ctx);
    }

    /// Render free-form footer content spanning all columns.
    pub fn tfoot(&mut self, content_fn: impl FnOnce(&mut Imm<'w, 's, Cap>)) {
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

/// Context for a single table row. Each `th()`/`td()` call spawns a grid cell.
pub struct RowCtx<'a, 'w, 's, Cap: CapSet> {
    ui: &'a mut Imm<'w, 's, Cap>,
    row_bg: Option<Color>,
    is_header: bool,
}

impl<Cap> RowCtx<'_, '_, '_, Cap>
where
    Cap: CapSet
        + ImplCap<CapabilityUiLayout>
        + ImplCap<CapabilityUiVisuals>
        + ImplCap<CapabilityUiTextStyle>
        + ImplCap<CapabilityUiText>
        + ImplCap<CapabilityUiBase>,
{
    /// Render a header cell. Applies bottom border, optional bg.
    pub fn th(&mut self, content: impl FnOnce(&mut Imm<Cap>)) {
        let mut cell = self
            .ui
            .ch()
            .overflow_clip()
            .py(Val::Px(SPACE_3))
            .px(Val::Px(SPACE_4))
            .style(|n: &mut Node| {
                n.border = UiRect::bottom(Val::Px(BORDER_WIDTH_DEFAULT));
            })
            .border_color(BORDER_DEFAULT);

        if let Some(bg) = self.row_bg {
            cell = cell.bg(bg);
        }

        cell.add(content);
    }

    /// Render a body cell.
    pub fn td(&mut self, content: impl FnOnce(&mut Imm<Cap>)) {
        let mut cell = self.ui.ch().overflow_clip().py(Val::Px(SPACE_3)).px(Val::Px(SPACE_4));

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
