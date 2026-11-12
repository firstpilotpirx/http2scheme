use super::{BAR, GENERAL_HEIGHT, tab_height};

#[test]
fn the_general_tab_is_short_and_the_history_tab_is_tall() {
    assert_eq!(tab_height(0), BAR + GENERAL_HEIGHT);
    assert_eq!(tab_height(1), BAR + 400.0);
    assert_eq!(tab_height(7), tab_height(1));
}
