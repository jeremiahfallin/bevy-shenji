use bevy::prelude::*;
use bevy::ui::RepeatedGridTrack;
use bevy_immediate::{ImmEntity, ImplCap};

use super::style::{CapabilityUiLayout, ImmUiStyleExt};

// CapabilityUiLayout moved to primitives/style.rs

/// Layout, Sizing, and Spacing utilities matching GPUI syntax.
pub trait ImmUiLayout {
    // Display
    fn flex(self) -> Self;
    fn grid(self) -> Self;
    fn hidden(self) -> Self;

    // Position
    fn absolute(self) -> Self;
    fn relative(self) -> Self;
    fn inset_0(self) -> Self;

    // Flex Container
    fn flex_row(self) -> Self;
    fn flex_col(self) -> Self;
    fn flex_wrap(self) -> Self;
    fn flex_nowrap(self) -> Self;

    // Flex Item
    fn flex_1(self) -> Self;
    fn flex_grow(self) -> Self;
    fn flex_shrink_0(self) -> Self;

    // Alignment
    fn items_center(self) -> Self;
    fn items_start(self) -> Self;
    fn items_end(self) -> Self;
    fn items_stretch(self) -> Self;

    fn justify_center(self) -> Self;
    fn justify_start(self) -> Self;
    fn justify_end(self) -> Self;
    fn justify_between(self) -> Self;
    fn justify_around(self) -> Self;

    // Sizing
    fn w(self, val: impl Into<Val>) -> Self;
    fn h(self, val: impl Into<Val>) -> Self;
    fn w_full(self) -> Self;
    fn h_full(self) -> Self;
    fn w_percent(self, val: f32) -> Self;
    fn h_percent(self, val: f32) -> Self;
    fn min_w(self, val: impl Into<Val>) -> Self;
    fn min_h(self, val: impl Into<Val>) -> Self;

    // Spacing (Padding)
    fn p(self, val: impl Into<Val>) -> Self;
    fn px(self, val: impl Into<Val>) -> Self;
    fn py(self, val: impl Into<Val>) -> Self;
    fn pl(self, val: impl Into<Val>) -> Self;
    fn pr(self, val: impl Into<Val>) -> Self;
    fn pt(self, val: impl Into<Val>) -> Self;
    fn pb(self, val: impl Into<Val>) -> Self;

    // Spacing (Margin)
    fn m(self, val: impl Into<Val>) -> Self;
    fn mx(self, val: impl Into<Val>) -> Self;
    fn my(self, val: impl Into<Val>) -> Self;
    fn ml(self, val: impl Into<Val>) -> Self;
    fn mr(self, val: impl Into<Val>) -> Self;
    fn m_auto(self) -> Self;
    fn mt(self, val: impl Into<Val>) -> Self;
    fn mb(self, val: impl Into<Val>) -> Self;

    // Grid
    fn grid_cols(self, count: u16) -> Self;
    fn col_span(self, span: u16) -> Self;
    fn gap(self, val: f32) -> Self;
    fn row_gap(self, val: f32) -> Self;
    fn column_gap(self, val: f32) -> Self;
    fn justify_self(self, val: JustifySelf) -> Self;
    fn scroll_y(self) -> Self;
    fn grid_template_columns(self, tracks: Vec<RepeatedGridTrack>) -> Self;
}

impl<Cap> ImmUiLayout for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout>,
{
    // --- Display ---
    fn flex(self) -> Self {
        self.style(|s| s.display = Display::Flex)
    }
    fn grid(self) -> Self {
        self.style(|s| s.display = Display::Grid)
    }
    fn hidden(self) -> Self {
        self.style(|s| s.display = Display::None)
    }

    // --- Position ---
    fn absolute(self) -> Self {
        self.style(|s| s.position_type = PositionType::Absolute)
    }
    fn relative(self) -> Self {
        self.style(|s| s.position_type = PositionType::Relative)
    }
    fn inset_0(self) -> Self {
        self.style(|s| {
            s.left = Val::Px(0.0);
            s.right = Val::Px(0.0);
            s.top = Val::Px(0.0);
            s.bottom = Val::Px(0.0);
        })
    }

    // --- Flex Container ---
    fn flex_row(self) -> Self {
        self.style(|s| s.flex_direction = FlexDirection::Row)
    }
    fn flex_col(self) -> Self {
        self.style(|s| s.flex_direction = FlexDirection::Column)
    }
    fn flex_wrap(self) -> Self {
        self.style(|s| s.flex_wrap = FlexWrap::Wrap)
    }
    fn flex_nowrap(self) -> Self {
        self.style(|s| s.flex_wrap = FlexWrap::NoWrap)
    }

    // --- Flex Item ---
    fn flex_1(self) -> Self {
        self.style(|s| {
            s.flex_grow = 1.0;
            s.flex_shrink = 1.0;
        })
    }
    fn flex_grow(self) -> Self {
        self.style(|s| s.flex_grow = 1.0)
    }
    fn flex_shrink_0(self) -> Self {
        self.style(|s| s.flex_shrink = 0.0)
    }

    // --- Alignment ---
    fn items_center(self) -> Self {
        self.style(|s| s.align_items = AlignItems::Center)
    }
    fn items_start(self) -> Self {
        self.style(|s| s.align_items = AlignItems::FlexStart)
    }
    fn items_end(self) -> Self {
        self.style(|s| s.align_items = AlignItems::FlexEnd)
    }
    fn items_stretch(self) -> Self {
        self.style(|s| s.align_items = AlignItems::Stretch)
    }

    fn justify_center(self) -> Self {
        self.style(|s| s.justify_content = JustifyContent::Center)
    }
    fn justify_start(self) -> Self {
        self.style(|s| s.justify_content = JustifyContent::FlexStart)
    }
    fn justify_end(self) -> Self {
        self.style(|s| s.justify_content = JustifyContent::FlexEnd)
    }
    fn justify_between(self) -> Self {
        self.style(|s| s.justify_content = JustifyContent::SpaceBetween)
    }
    fn justify_around(self) -> Self {
        self.style(|s| s.justify_content = JustifyContent::SpaceAround)
    }

    // --- Sizing ---
    fn w(self, val: impl Into<Val>) -> Self {
        self.style(|s| s.width = val.into())
    }
    fn h(self, val: impl Into<Val>) -> Self {
        self.style(|s| s.height = val.into())
    }
    fn w_full(self) -> Self {
        self.w(Val::Percent(100.0))
    }
    fn h_full(self) -> Self {
        self.h(Val::Percent(100.0))
    }
    fn w_percent(self, val: f32) -> Self {
        self.w(Val::Percent(val))
    }
    fn h_percent(self, val: f32) -> Self {
        self.h(Val::Percent(val))
    }
    fn min_w(self, val: impl Into<Val>) -> Self {
        self.style(|s| s.min_width = val.into())
    }
    fn min_h(self, val: impl Into<Val>) -> Self {
        self.style(|s| s.min_height = val.into())
    }

    // --- Spacing (Padding) ---
    fn p(self, val: impl Into<Val>) -> Self {
        let v = val.into();
        self.style(move |s| s.padding = UiRect::all(v))
    }
    fn px(self, val: impl Into<Val>) -> Self {
        let v = val.into();
        self.style(|s| {
            s.padding.left = v;
            s.padding.right = v;
        })
    }
    fn py(self, val: impl Into<Val>) -> Self {
        let v = val.into();
        self.style(|s| {
            s.padding.top = v;
            s.padding.bottom = v;
        })
    }
    fn pl(self, val: impl Into<Val>) -> Self {
        self.style(|s| s.padding.left = val.into())
    }
    fn pr(self, val: impl Into<Val>) -> Self {
        self.style(|s| s.padding.right = val.into())
    }
    fn pt(self, val: impl Into<Val>) -> Self {
        self.style(|s| s.padding.top = val.into())
    }
    fn pb(self, val: impl Into<Val>) -> Self {
        self.style(|s| s.padding.bottom = val.into())
    }

    // --- Spacing (Margin) ---
    fn m(self, val: impl Into<Val>) -> Self {
        let v = val.into();
        self.style(|s| s.margin = UiRect::all(v))
    }
    fn mx(self, val: impl Into<Val>) -> Self {
        let v = val.into();
        self.style(|s| {
            s.margin.left = v;
            s.margin.right = v;
        })
    }
    fn my(self, val: impl Into<Val>) -> Self {
        let v = val.into();
        self.style(|s| {
            s.margin.top = v;
            s.margin.bottom = v;
        })
    }
    fn ml(self, val: impl Into<Val>) -> Self {
        self.style(|s| s.margin.left = val.into())
    }
    fn mr(self, val: impl Into<Val>) -> Self {
        self.style(|s| s.margin.right = val.into())
    }
    fn m_auto(self) -> Self {
        self.style(|s| s.margin = UiRect::all(Val::Auto))
    }
    fn mt(self, val: impl Into<Val>) -> Self {
        self.style(|s| s.margin.top = val.into())
    }
    fn mb(self, val: impl Into<Val>) -> Self {
        self.style(|s| s.margin.bottom = val.into())
    }

    // --- Grid ---
    fn grid_cols(self, count: u16) -> Self {
        self.style(|s| {
            s.display = Display::Grid;
            s.grid_template_columns = vec![RepeatedGridTrack::flex(1, 1.0); count as usize];
        })
    }

    fn col_span(self, span: u16) -> Self {
        self.style(|s| s.grid_column = GridPlacement::span(span))
    }

    fn gap(self, val: f32) -> Self {
        self.style(|s| {
            s.row_gap = Val::Px(val);
            s.column_gap = Val::Px(val);
        })
    }

    fn row_gap(self, val: f32) -> Self {
        self.style(|s| s.row_gap = Val::Px(val))
    }

    fn column_gap(self, val: f32) -> Self {
        self.style(|s| s.column_gap = Val::Px(val))
    }

    fn justify_self(self, val: JustifySelf) -> Self {
        self.style(move |s| s.justify_self = val)
    }

    fn scroll_y(self) -> Self {
        self.style(|s| s.overflow.y = OverflowAxis::Scroll)
    }

    fn grid_template_columns(self, tracks: Vec<RepeatedGridTrack>) -> Self {
        self.style(move |s| s.grid_template_columns = tracks)
    }
}
