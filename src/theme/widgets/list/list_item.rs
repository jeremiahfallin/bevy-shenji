use bevy::prelude::*;
use bevy_immediate::ui::base::CapabilityUiBase;
use bevy_immediate::ui::interaction::CapabilityUiInteraction;
use bevy_immediate::ui::layout_order::CapabilityUiLayoutOrder;
use bevy_immediate::ui::look::CapabilityUiLook;
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::ui::text::ImmUiText;
use bevy_immediate::ui::*;

use crate::theme::primitives::{CapabilityUiLayout, CapabilityUiTextStyle};
use bevy_immediate::{CapSet, Imm, ImmEntity, ImplCap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum ListItemSpacing {
    #[default]
    Dense,
    ExtraDense,
    Sparse,
}

impl ListItemSpacing {
    fn padding_y(&self) -> f32 {
        match self {
            ListItemSpacing::ExtraDense => 0.0,
            ListItemSpacing::Dense => 4.0,
            ListItemSpacing::Sparse => 8.0,
        }
    }
}

pub struct ListItem<'a, Cap: CapSet> {
    id: Option<String>,
    disabled: bool,
    selected: bool,
    spacing: ListItemSpacing,
    indent_level: usize,
    indent_step_size: f32,
    start_slot: Option<Box<dyn FnOnce(&mut Imm<Cap>) + 'a>>,
    end_slot: Option<Box<dyn FnOnce(&mut Imm<Cap>) + 'a>>,
    toggle: Option<bool>,
    on_toggle: Option<Box<dyn FnMut() + 'a>>,
    on_click: Option<Box<dyn FnMut() + 'a>>,
}

impl<'a, Cap: CapSet> ListItem<'a, Cap> {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: Some(id.into()),
            disabled: false,
            selected: false,
            spacing: ListItemSpacing::Dense,
            indent_level: 0,
            indent_step_size: 12.0,
            start_slot: None,
            end_slot: None,
            toggle: None,
            on_toggle: None,
            on_click: None,
        }
    }

    pub fn spacing(mut self, spacing: ListItemSpacing) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn indent_level(mut self, level: usize) -> Self {
        self.indent_level = level;
        self
    }

    pub fn toggle(mut self, is_open: impl Into<Option<bool>>) -> Self {
        self.toggle = is_open.into();
        self
    }

    pub fn on_toggle(mut self, handler: impl FnMut() + 'a) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }

    pub fn on_click(mut self, handler: impl FnMut() + 'a) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn start_slot(mut self, slot: impl FnOnce(&mut Imm<Cap>) + 'a) -> Self {
        self.start_slot = Some(Box::new(slot));
        self
    }

    pub fn end_slot(mut self, slot: impl FnOnce(&mut Imm<Cap>) + 'a) -> Self {
        self.end_slot = Some(Box::new(slot));
        self
    }

    pub fn render(mut self, ui: &mut Imm<Cap>, children: impl FnOnce(&mut Imm<Cap>))
    where
        Cap: ImplCap<CapabilityUiLayout>
            + ImplCap<CapabilityUiLook>
            + ImplCap<CapabilityUiLayoutOrder>
            + ImplCap<CapabilityUiInteraction>
            + ImplCap<CapabilityUiTextStyle>
            + ImplCap<CapabilityUiBase>
            + ImplCap<CapabilityUiText>,
    {
        let mut container = ui.ch();
        let mut container_entity = container;

        container_entity.add(|ui| {
            if let Some(start_slot) = self.start_slot {
                ui.ch().add(start_slot);
            }

            ui.ch().add(children);

            if let Some(end_slot) = self.end_slot {
                ui.ch().add(end_slot);
            }
        });
    }
}
