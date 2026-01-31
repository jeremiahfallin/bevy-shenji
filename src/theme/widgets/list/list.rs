use bevy::prelude::*;
use bevy_immediate::ui::base::CapabilityUiBase;
use bevy_immediate::ui::layout_order::CapabilityUiLayoutOrder;
use bevy_immediate::ui::look::CapabilityUiLook;
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::ui::text::ImmUiText;
use bevy_immediate::ui::*;

use crate::theme::primitives::{CapabilityUiLayout, CapabilityUiTextStyle};
use crate::theme::widgets::label::ImmUiLabel;
use bevy_immediate::{CapSet, Imm, ImmEntity, ImplCap};

pub enum EmptyMessage {
    Text(String),
}

impl From<&str> for EmptyMessage {
    fn from(s: &str) -> Self {
        EmptyMessage::Text(s.to_string())
    }
}

impl From<String> for EmptyMessage {
    fn from(s: String) -> Self {
        EmptyMessage::Text(s)
    }
}

pub struct List {
    empty_message: EmptyMessage,
    header: Option<String>,
    toggle: Option<bool>,
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl List {
    pub fn new() -> Self {
        Self {
            empty_message: EmptyMessage::Text("No items".to_string()),
            header: None,
            toggle: None,
        }
    }

    pub fn empty_message(mut self, message: impl Into<EmptyMessage>) -> Self {
        self.empty_message = message.into();
        self
    }

    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    pub fn toggle(mut self, toggle: impl Into<Option<bool>>) -> Self {
        self.toggle = toggle.into();
        self
    }

    pub fn render<Cap, F>(self, ui: &mut Imm<Cap>, has_items: bool, children: F)
    where
        Cap: CapSet
            + ImplCap<CapabilityUiLayout>
            + ImplCap<CapabilityUiLayoutOrder>
            + ImplCap<CapabilityUiLook>
            + ImplCap<CapabilityUiBase>
            + ImplCap<CapabilityUiTextStyle>
            + ImplCap<CapabilityUiText>,
        F: FnOnce(&mut Imm<Cap>),
    {
        let mut container = ui.ch();
        container.add(|ui| {
            if let Some(header_text) = self.header {
                ui.ch().label(header_text);
            }

            if has_items {
                children(ui);
            } else {
                let show_empty = match self.toggle {
                    Some(false) => false,
                    _ => true,
                };

                if show_empty {
                    match self.empty_message {
                        EmptyMessage::Text(text) => {
                            ui.ch().label(text).color(Color::srgb(0.5, 0.5, 0.5));
                        }
                    }
                }
            }
        });
    }
}

pub trait ImmUiListExtension {
    fn list() -> List;
}

impl<Cap: CapSet> ImmUiListExtension for Imm<'_, '_, Cap> {
    fn list() -> List {
        List::new()
    }
}
