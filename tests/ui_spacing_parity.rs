use shenji::theme::styles::spacing as old;
use shenji::ui::tokens::spacing as new;

#[test]
fn spacing_scale_matches() {
    assert_eq!(new::SPACE_0, old::SPACE_0);
    assert_eq!(new::SPACE_1, old::SPACE_1);
    assert_eq!(new::SPACE_2, old::SPACE_2);
    assert_eq!(new::SPACE_4, old::SPACE_4);
    assert_eq!(new::SPACE_8, old::SPACE_8);
    assert_eq!(new::SPACE_24, old::SPACE_24);
}
