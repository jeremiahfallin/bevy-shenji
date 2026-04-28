//! One-stop import for screens consuming the new UI.
//!
//! Usage: `use crate::ui::prelude::*;`

pub use bevy_declarative::prelude::*;

pub use crate::ui::tokens::palette::*;
pub use crate::ui::tokens::spacing::*;
pub use crate::ui::tokens::typography::*;

pub use crate::ui::presets::buttons::*;
pub use crate::ui::presets::containers::*;
pub use crate::ui::presets::text::*;

pub use crate::ui::widgets::badge::*;
pub use crate::ui::widgets::checkbox::*;
pub use crate::ui::widgets::divider::*;
pub use crate::ui::widgets::icon::*;
pub use crate::ui::widgets::label::*;
pub use crate::ui::widgets::progress_bar::*;
pub use crate::ui::widgets::radio::*;
pub use crate::ui::widgets::slider::*;
pub use crate::ui::widgets::tabs::*;
pub use crate::ui::widgets::text_input::*;
pub use crate::ui::widgets::tooltip::*;

pub use crate::ui::components::scroll_view::*;
