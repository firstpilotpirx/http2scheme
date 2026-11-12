use super::ServerStateEnum;

#[test]
fn each_state_has_its_status_line() {
    assert_eq!(ServerStateEnum::Starting.status_line(), "Starting…");
    let running = ServerStateEnum::Running("http://127.0.0.1:7777".to_owned());
    assert_eq!(running.status_line(), "Running on http://127.0.0.1:7777");
    let stopped = ServerStateEnum::Stopped("port 7777 is taken".to_owned());
    assert_eq!(stopped.status_line(), "Stopped: port 7777 is taken");
    assert_eq!(ServerStateEnum::Off.status_line(), "Off");
}

#[test]
fn only_off_is_not_serving() {
    assert!(!ServerStateEnum::Off.is_serving());
    assert!(ServerStateEnum::Starting.is_serving());
    assert!(ServerStateEnum::Stopped("port 7777 is taken".to_owned()).is_serving());
}
