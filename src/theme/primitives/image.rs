// theme/primitives/image.rs
use bevy::prelude::*;
use bevy_immediate::ui::CapsUi;
use bevy_immediate::{CapSet, ImmCapAccessRequests, ImmCapability, ImmEntity, ImplCap};

use super::style::{CapabilityUiLayout, ImmUiStyleExt};

pub struct CapabilityUiImage;

impl ImmCapability for CapabilityUiImage {
    fn build<Cap: CapSet>(app: &mut bevy::app::App, cap_req: &mut ImmCapAccessRequests<Cap>) {
        cap_req.request_component_write::<ImageNode>(app.world_mut());
    }
}

impl ImplCap<CapabilityUiImage> for CapsUi {}

pub trait ImmUiImageExt {
    fn image(self, handle: Handle<Image>) -> Self;
    fn image_color(self, color: impl Into<Color>) -> Self;
}

impl<Cap> ImmUiImageExt for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout> + ImplCap<CapabilityUiImage>,
{
    fn image(mut self, handle: Handle<Image>) -> Self {
        if let Ok(Some(mut img)) = self.cap_get_component_mut::<ImageNode>() {
            img.image = handle;
        } else {
            self.entity_commands().insert(ImageNode {
                image: handle,
                ..default()
            });
            self = self.style(|_s| {});
        }
        self
    }

    fn image_color(mut self, color: impl Into<Color>) -> Self {
        if let Ok(Some(mut img)) = self.cap_get_component_mut::<ImageNode>() {
            img.color = color.into();
        }
        self
    }
}
