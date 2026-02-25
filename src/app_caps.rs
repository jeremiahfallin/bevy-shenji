//! Application-specific capability set for bevy_immediate.
//!
//! This module defines `AppCaps`, the aggregate capability set that enables
//! all theme and framework capabilities for the application's UI.
//! This is intentionally outside the `theme` module so the theme stays
//! reusable across projects — each project defines its own `CapSet`.

use bevy::prelude::*;
use bevy_immediate::ui::base::CapabilityUiBase;
use bevy_immediate::ui::interaction::CapabilityUiInteraction;
use bevy_immediate::ui::look::CapabilityUiLook;
use bevy_immediate::ui::text::CapabilityUiText;
use bevy_immediate::{CapSet, ImmCapAccessRequests, ImmCapability, ImplCap};

use crate::theme::behaviors::CapabilityObserver;
use crate::theme::primitives::image::CapabilityUiImage;
use crate::theme::primitives::style::CapabilityUiLayout;
use crate::theme::primitives::text::CapabilityUiTextStyle;
use crate::theme::primitives::visuals::CapabilityUiVisuals;
use crate::theme::widgets::button::CapabilityButton;

/// The application's aggregate capability set.
///
/// Combines all bevy_immediate core capabilities with the theme's
/// extension capabilities. Each new project should define its own
/// `CapSet` type and implement `ImplCap` for all capabilities it needs.
pub struct AppCaps;

impl CapSet for AppCaps {
    fn initialize<T: CapSet>(app: &mut App, cap_req: &mut ImmCapAccessRequests<T>) {
        // bevy_immediate core capabilities
        CapabilityUiBase::build(app, cap_req);
        CapabilityUiText::build(app, cap_req);
        CapabilityUiInteraction::build(app, cap_req);
        CapabilityUiLook::build(app, cap_req);

        // Theme extension capabilities
        CapabilityUiLayout::build(app, cap_req);
        CapabilityUiTextStyle::build(app, cap_req);
        CapabilityUiVisuals::build(app, cap_req);
        CapabilityButton::build(app, cap_req);
        CapabilityObserver::build(app, cap_req);
        CapabilityUiImage::build(app, cap_req);
    }
}

// bevy_immediate core capabilities
impl ImplCap<CapabilityUiBase> for AppCaps {}
impl ImplCap<CapabilityUiText> for AppCaps {}
impl ImplCap<CapabilityUiInteraction> for AppCaps {}
impl ImplCap<CapabilityUiLook> for AppCaps {}

// Theme extension capabilities
impl ImplCap<CapabilityUiLayout> for AppCaps {}
impl ImplCap<CapabilityUiTextStyle> for AppCaps {}
impl ImplCap<CapabilityUiVisuals> for AppCaps {}
impl ImplCap<CapabilityButton> for AppCaps {}
impl ImplCap<CapabilityObserver> for AppCaps {}
impl ImplCap<CapabilityUiImage> for AppCaps {}
