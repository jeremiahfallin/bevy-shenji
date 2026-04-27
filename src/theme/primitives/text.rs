use bevy::prelude::*;
use bevy::text::LineHeight;
use bevy_immediate::ui::CapsUi;
use bevy_immediate::{CapSet, ImmCapAccessRequests, ImmCapability, ImmEntity, ImplCap};

/// Default LineHeight for Bevy 0.18 — required component on Text spawns.
pub const DEFAULT_LINE_HEIGHT: LineHeight = LineHeight::RelativeToFont(1.5);

// 1. Define the Capability
pub struct CapabilityUiTextStyle;

impl ImmCapability for CapabilityUiTextStyle {
    fn build<Cap: CapSet>(app: &mut bevy::app::App, cap_req: &mut ImmCapAccessRequests<Cap>) {
        cap_req.request_component_write::<TextFont>(app.world_mut());
        cap_req.request_component_write::<TextColor>(app.world_mut());
        cap_req.request_component_write::<TextLayout>(app.world_mut());
    }
}

// 2. Define the Fluent API
pub trait ImmUiTextStyleExtension {
    fn text_size(self, size: f32) -> Self;
    fn text_color(self, color: impl Into<Color>) -> Self;
    fn text_align(self, align: Justify) -> Self;
    fn text_left(self) -> Self;
    fn text_center(self) -> Self;
    fn text_right(self) -> Self;

    // Helpers matching styles.rs tokens
    fn text_xs(self) -> Self;
    fn text_sm(self) -> Self;
    fn text_base(self) -> Self;
    fn text_lg(self) -> Self;
    fn text_xl(self) -> Self;
    fn text_2xl(self) -> Self;

    fn font(self, path: &str) -> Self;

    // Whitespace / Overflow
    fn whitespace_nowrap(self) -> Self;

    // Font Weights
    fn font_bold(self) -> Self;
}

// Assumes you defined these constants in src/theme/styles.rs
// or replace with raw values
const TEXT_XS: f32 = 12.0;
const TEXT_SM: f32 = 14.0;
const TEXT_BASE: f32 = 16.0;
const TEXT_LG: f32 = 18.0;
const TEXT_XL: f32 = 20.0;
const TEXT_2XL: f32 = 24.0;

use bevy_immediate::ui::text::CapabilityUiText;

impl<Cap> ImmUiTextStyleExtension for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiTextStyle>,
{
    fn text_size(mut self, size: f32) -> Self {
        if let Ok(Some(mut font)) = self.cap_get_component_mut::<TextFont>() {
            font.font_size = size;
        } else {
            self.entity_commands().insert(TextFont {
                font_size: size,
                ..default()
            });
        }
        self
    }

    fn text_color(mut self, color: impl Into<Color>) -> Self {
        let c = color.into();
        if let Ok(Some(mut text_color)) = self.cap_get_component_mut::<TextColor>() {
            text_color.0 = c;
        } else {
            self.entity_commands().insert(TextColor(c));
        }
        self
    }

    fn text_align(mut self, align: Justify) -> Self {
        if let Ok(Some(mut layout)) = self.cap_get_component_mut::<TextLayout>() {
            layout.justify = align;
        } else {
            self.entity_commands().insert(TextLayout {
                justify: align,
                ..default()
            });
        }
        self
    }

    fn text_left(self) -> Self {
        self.text_align(Justify::Left)
    }
    fn text_center(self) -> Self {
        self.text_align(Justify::Center)
    }
    fn text_right(self) -> Self {
        self.text_align(Justify::Right)
    }

    fn text_xs(self) -> Self {
        self.text_size(TEXT_XS)
    }
    fn text_sm(self) -> Self {
        self.text_size(TEXT_SM)
    }
    fn text_base(self) -> Self {
        self.text_size(TEXT_BASE)
    }
    fn text_lg(self) -> Self {
        self.text_size(TEXT_LG)
    }
    fn text_xl(self) -> Self {
        self.text_size(TEXT_XL)
    }
    fn text_2xl(self) -> Self {
        self.text_size(TEXT_2XL)
    }

    fn font(mut self, path: &str) -> Self {
        // Note: This requires access to AssetServer, usually passed in Params or available in World.
        // For immediate mode without direct AssetServer access, you might need a globally resource
        // that holds handles, or request AssetServer in the capability build method.
        // Simplified placeholder:
        self
    }

    fn whitespace_nowrap(mut self) -> Self {
        if let Ok(Some(mut layout)) = self.cap_get_component_mut::<TextLayout>() {
            layout.linebreak = bevy::text::LineBreak::NoWrap;
        }
        self
    }

    fn font_bold(self) -> Self {
        // Placeholder: Needs AssetServer or a resource with font handles.
        // For now, we'll leave it as a no-op or TODO until asset management is clarified.
        // warn_once!("font_bold not fully implemented - requires font handle");
        self
    }
}

impl ImplCap<CapabilityUiTextStyle> for CapsUi {}
