use super::{button_frame, columns};

#[test]
fn the_columns_follow_the_mock_and_fill_the_row() {
    let [time, result, link, button] = columns(660.0);
    assert_eq!(time, (8.0, 130.0));
    assert_eq!(result, (146.0, 110.0));
    assert_eq!(button, (616.0, 36.0));
    assert_eq!(link.0, 264.0);
    assert_eq!(link.0 + link.1 + 8.0, button.0);
}

#[test]
fn the_play_button_sits_in_its_row() {
    let frame = button_frame(660.0, 260.0, 1);
    assert_eq!(frame.origin.x, 616.0);
    assert_eq!(frame.origin.y, 260.0 - 2.0 * 26.0 + 3.0);
    assert_eq!(frame.size.height, 20.0);
}
