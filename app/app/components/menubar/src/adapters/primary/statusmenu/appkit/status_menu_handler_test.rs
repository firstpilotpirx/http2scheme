use std::future::ready as now;

use super::ready;

#[test]
fn a_ready_future_answers_at_its_first_poll() {
    assert_eq!(ready(now(7)), 7);
}
