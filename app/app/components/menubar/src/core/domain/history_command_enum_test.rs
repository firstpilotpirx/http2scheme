use super::HistoryCommandEnum;

#[test]
fn a_replay_names_its_link() {
    assert_ne!(
        HistoryCommandEnum::Replay("idea://a".to_owned()),
        HistoryCommandEnum::Replay("idea://b".to_owned())
    );
    assert_ne!(
        HistoryCommandEnum::Show,
        HistoryCommandEnum::Replay(String::new())
    );
}
