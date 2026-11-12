use super::columns;

#[test]
fn the_link_column_fills_the_row_between_the_result_and_the_button() {
    let [time, result, link] = columns(628.0);
    assert_eq!(time, (0.0, 150.0));
    assert_eq!(result, (158.0, 130.0));
    assert_eq!(link.0, 296.0);
    assert!((link.0 + link.1 + 8.0 + 40.0 - 628.0).abs() < f64::EPSILON);
}
