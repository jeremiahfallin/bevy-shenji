use shenji::theme::styles::typography as old;
use shenji::ui::tokens::typography as new;

#[test]
fn font_size_scale_matches() {
    assert_eq!(new::TEXT_XS, old::TEXT_XS);
    assert_eq!(new::TEXT_BASE, old::TEXT_BASE);
    assert_eq!(new::TEXT_3XL, old::TEXT_3XL);
    assert_eq!(new::TEXT_5XL, old::TEXT_5XL);
}

#[test]
fn line_heights_match() {
    assert_eq!(new::LEADING_TIGHT, old::LEADING_TIGHT);
    assert_eq!(new::LEADING_NORMAL, old::LEADING_NORMAL);
}
