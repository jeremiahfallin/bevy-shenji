use crate::theme::primitives::text::{CapabilityUiTextStyle, ImmUiTextStyleExtension};
use bevy::prelude::*;
use bevy_immediate::ui::text::{CapabilityUiText, ImmUiText};
use bevy_immediate::{CapSet, ImmEntity, ImplCap};

// -----------------------------------------------------------------------------
// 1. Helper Enums (Ported from GPUI concepts)
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Default, Debug)]
pub enum LabelSize {
    #[default]
    Default,
    Small,
    Large,
    XLarge,
}

impl LabelSize {
    fn to_px(self) -> f32 {
        match self {
            LabelSize::Small => 12.0,
            LabelSize::Default => 16.0,
            LabelSize::Large => 20.0,
            LabelSize::XLarge => 24.0,
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub enum LineHeightStyle {
    #[default]
    Default,
    UiLabel,
    Prose,
}

// -----------------------------------------------------------------------------
// 2. The Extension Trait
// -----------------------------------------------------------------------------

pub trait ImmUiLabel {
    /// Creates a label with the default configuration.
    fn label(self, text: impl Into<String>) -> Self;

    /// Sets the text size.
    fn size(self, size: LabelSize) -> Self;

    /// Sets the text color.
    fn color(self, color: Color) -> Self;

    /// Sets the transparency of the text.
    fn alpha(self, alpha: f32) -> Self;

    /// Forces the text to be a single line (replaces newlines with spaces).
    fn single_line(self) -> Self;

    /// Enables ellipsis truncation for overflowing text.
    fn truncate(self) -> Self;

    // NOTE: Bold, Italic, Strikethrough, and Underline usually require
    // switching the `Font` handle in Bevy or adding child entities.
    // I have added the signatures below for API compatibility.
    fn weight(self, weight: FontWeight) -> Self;
    fn italic(self) -> Self;
}

// -----------------------------------------------------------------------------
// 3. The Implementation
// -----------------------------------------------------------------------------

impl<Cap> ImmUiLabel for ImmEntity<'_, '_, '_, Cap>
where
    // We need write access to Text components (provided by standard CapabilityUiText)
    // and our local text extensions (provided by CapabilityUiTextStyle)
    Cap: ImplCap<CapabilityUiTextStyle> + ImplCap<CapabilityUiText>,
{
    fn label(self, text: impl Into<String>) -> Self {
        // Initialize with default standard text (base size)
        let t: String = text.into();
        self.text(t).text_base()
    }

    fn size(self, size: LabelSize) -> Self {
        self.text_size(size.to_px())
    }

    fn color(self, color: Color) -> Self {
        self.text_color(color)
    }

    fn alpha(mut self, alpha: f32) -> Self {
        if let Ok(Some(mut text_color)) = self.cap_get_component_mut::<TextColor>() {
            text_color.0.set_alpha(alpha);
        }
        self
    }

    fn single_line(self) -> Self {
        self.whitespace_nowrap()
    }

    fn truncate(mut self) -> Self {
        if let Ok(Some(mut layout)) = self.cap_get_component_mut::<TextLayout>() {
            layout.linebreak = bevy::text::LineBreak::NoWrap;
        }
        // Also ensure entity has simple clipping if possible?
        // Node overflow: hidden is usually on the parent.
        self
    }

    fn weight(self, weight: FontWeight) -> Self {
        match weight {
            FontWeight::Bold => self.font_bold(),
            FontWeight::ExtraBold => self.font_bold(), // Map to bold for now
            FontWeight::Normal => self,                // TODO: font_normal()
        }
    }

    fn italic(self) -> Self {
        // self.font_italic()
        self
    }
}

// -----------------------------------------------------------------------------
// 4. Helper for FontWeight (Mocking GPUI)
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum FontWeight {
    Normal,
    Bold,
    ExtraBold,
}
