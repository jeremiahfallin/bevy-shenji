//! One-stop import for screens consuming the new UI.
//!
//! Usage: `use crate::ui::prelude::*;`
//!
//! ## Glob collisions with `bevy::prelude::*`
//!
//! When a screen has both `use bevy::prelude::*;` and `use crate::ui::prelude::*;`
//! in scope, names that exist in both preludes trigger Rust's
//! `ambiguous_glob_imports` warning (a future-incompat lint). Rust's
//! "explicit beats glob" resolution only applies when the disambiguating
//! `use` is at the *consumer's* site, not inside a re-exporting prelude —
//! so the explicit `pub use` lines below do not silence the lint at call
//! sites by themselves.
//!
//! Policy: call sites that hit a collision should add an explicit
//! `use bevy_declarative::prelude::<name>;` (or the equivalent) alongside
//! their globs. The explicit re-exports below document which side this
//! prelude considers canonical.
//!
//! Currently known collisions (canonical source listed):
//!
//! - `px`: `bevy_declarative::prelude::px` (the value-construction helper
//!   used by the new UI's builder API).

pub use bevy_declarative::prelude::*;

// Document the canonical source for `px` against `bevy::prelude::px`.
// Note: this re-export does not by itself silence the consumer-site
// `ambiguous_glob_imports` lint — call sites must add their own explicit
// `use bevy_declarative::prelude::px;` alongside `use bevy::prelude::*;`.
pub use bevy_declarative::prelude::px;

pub use crate::ui::tokens::borders::*;
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
