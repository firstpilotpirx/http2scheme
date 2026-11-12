use super::backticked;

#[test]
fn backticked_takes_the_first_quoted_name() {
    assert_eq!(
        backticked("missing field `allowed_schemes` here"),
        Some("allowed_schemes".to_owned())
    );
    assert_eq!(backticked("no name"), None);
}
