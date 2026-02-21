//! Capability extensions for bevy_immediate's built-in CapsUi.
//!
//! The theme extends `CapsUi` with its own capabilities so that internal
//! widgets using `CapsUi` can access theme-provided features.
//! The application-specific aggregate `CapSet` (e.g., `AppCaps`) is
//! defined outside the theme — see `src/app_caps.rs`.

use bevy_immediate::ui::CapsUi;
use bevy_immediate::ImplCap;

use crate::theme::behaviors::CapabilityObserver;

// Extend CapsUi (from bevy_immediate) with our local capabilities
impl ImplCap<CapabilityObserver> for CapsUi {}
