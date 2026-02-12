use crate::theme::primitives::image::CapabilityUiImage;
use bevy::prelude::*;
use bevy_immediate::ui::CapsUi;
use bevy_immediate::ui::base::CapabilityUiBase;
use bevy_immediate::ui::interaction::CapabilityUiInteraction;
use bevy_immediate::ui::look::CapabilityUiLook;
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::{CapSet, ImmCapAccessRequests, ImplCap};

pub struct ShenjiCaps;

impl CapSet for ShenjiCaps {
    fn initialize<T: CapSet>(_app: &mut App, _cap_req: &mut ImmCapAccessRequests<T>) {}
}

// Foreign core capabilities (LEGAL because ShenjiCaps is local)
impl ImplCap<CapabilityUiBase> for ShenjiCaps {}
impl ImplCap<CapabilityUiText> for ShenjiCaps {}
impl ImplCap<CapabilityUiInteraction> for ShenjiCaps {}
impl ImplCap<CapabilityUiLook> for ShenjiCaps {}
// My local extension capabilities
use crate::theme::behaviors::CapabilityObserver;
use crate::theme::primitives::style::CapabilityUiLayout;
use crate::theme::primitives::text::CapabilityUiTextStyle;
use crate::theme::primitives::visuals::CapabilityUiVisuals;
use crate::theme::widgets::button::CapabilityButton;

impl ImplCap<CapabilityUiLayout> for ShenjiCaps {}
impl ImplCap<CapabilityUiTextStyle> for ShenjiCaps {}
impl ImplCap<CapabilityUiVisuals> for ShenjiCaps {}
impl ImplCap<CapabilityButton> for ShenjiCaps {}
impl ImplCap<CapabilityObserver> for ShenjiCaps {}
impl ImplCap<CapabilityUiImage> for ShenjiCaps {}

// Extend CapsUi (from bevy_immediate) with our local capabilities
impl ImplCap<CapabilityObserver> for CapsUi {}
