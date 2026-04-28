//! Asserts that ported palette constants match `src/theme/styles/palette.rs`
//! exactly. Removed when `src/theme/` is deleted in Phase C.

use shenji::theme::styles::palette as old;
use shenji::ui::tokens::palette as new;

#[test]
fn primary_constants_match() {
    assert_eq!(new::PRIMARY_400, old::PRIMARY_400);
    assert_eq!(new::PRIMARY_500, old::PRIMARY_500);
    assert_eq!(new::PRIMARY_600, old::PRIMARY_600);
    assert_eq!(new::PRIMARY_700, old::PRIMARY_700);
}

#[test]
fn gray_constants_match() {
    assert_eq!(new::GRAY_50, old::GRAY_50);
    assert_eq!(new::GRAY_900, old::GRAY_900);
    assert_eq!(new::GRAY_950, old::GRAY_950);
}

#[test]
fn semantic_text_constants_match() {
    assert_eq!(new::TEXT_PRIMARY, old::TEXT_PRIMARY);
    assert_eq!(new::TEXT_SECONDARY, old::TEXT_SECONDARY);
    assert_eq!(new::TEXT_MUTED, old::TEXT_MUTED);
}

#[test]
fn surface_constants_match() {
    assert_eq!(new::SURFACE_BASE, old::SURFACE_BASE);
    assert_eq!(new::SURFACE_RAISED, old::SURFACE_RAISED);
}
