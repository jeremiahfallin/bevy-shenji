//! Asserts that ported border tokens match `src/theme/styles/borders.rs`
//! exactly. Removed when `src/theme/` is deleted in Phase C.

use shenji::theme::styles::borders as old;
use shenji::ui::tokens::borders as new;

#[test]
fn border_widths_match() {
    assert_eq!(new::BORDER_WIDTH_0, old::BORDER_WIDTH_0);
    assert_eq!(new::BORDER_WIDTH_DEFAULT, old::BORDER_WIDTH_DEFAULT);
    assert_eq!(new::BORDER_WIDTH_2, old::BORDER_WIDTH_2);
    assert_eq!(new::BORDER_WIDTH_3, old::BORDER_WIDTH_3);
    assert_eq!(new::BORDER_WIDTH_4, old::BORDER_WIDTH_4);
}

#[test]
fn radius_constants_match() {
    assert_eq!(new::RADIUS_NONE, old::RADIUS_NONE);
    assert_eq!(new::RADIUS_SM, old::RADIUS_SM);
    assert_eq!(new::RADIUS_DEFAULT, old::RADIUS_DEFAULT);
    assert_eq!(new::RADIUS_MD, old::RADIUS_MD);
    assert_eq!(new::RADIUS_LG, old::RADIUS_LG);
    assert_eq!(new::RADIUS_XL, old::RADIUS_XL);
    assert_eq!(new::RADIUS_2XL, old::RADIUS_2XL);
    assert_eq!(new::RADIUS_FULL, old::RADIUS_FULL);
}
