use super::MenuActionError;

#[tokio::test]
async fn each_error_names_what_failed() {
    let login = MenuActionError::LoginItem("Operation not permitted".to_owned());
    assert_eq!(login.to_string(), "login item: Operation not permitted");
    let record = MenuActionError::FirstLaunch {
        path: "/x/login-item-offered".to_owned(),
        reason: "denied".to_owned(),
    };
    assert_eq!(record.to_string(), "/x/login-item-offered: denied");
    let history = MenuActionError::History {
        path: "/x/requests.log".to_owned(),
        reason: "denied".to_owned(),
    };
    assert_eq!(history.to_string(), "/x/requests.log: denied");
}
