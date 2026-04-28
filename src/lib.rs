//! Library facade for the `shenji` crate.
//!
//! Exposes select modules so integration tests under `tests/` can reference
//! them without duplicating compilation. Uses `#[path]` to point at the same
//! source files declared by `src/main.rs`, avoiding a parallel module tree.

#[path = "theme/mod.rs"]
pub mod theme;

#[path = "ui/mod.rs"]
pub mod ui;
