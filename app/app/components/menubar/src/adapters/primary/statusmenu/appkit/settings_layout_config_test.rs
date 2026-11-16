use super::{BAR, GENERAL_HEIGHT, ROW_HEIGHT, tab_height};

#[test]
fn each_tab_has_its_own_height() {
    assert_eq!(tab_height(0), BAR + GENERAL_HEIGHT);
    assert_eq!(tab_height(1), BAR + 400.0);
    assert_eq!(tab_height(2), BAR + GENERAL_HEIGHT + ROW_HEIGHT);
    assert_eq!(tab_height(7), tab_height(2));
}
