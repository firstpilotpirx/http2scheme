use std::sync::Arc;

use crate::core::application::use_cases::open_link_command::OpenLinkCommand;
use crate::core::domain::open_outcome_enum::OpenOutcomeEnum;

use super::{MockOpenLink, OpenLink};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockOpenLink::new();
    mock.expect_execute()
        .returning(|_| OpenOutcomeEnum::WrongKey);
    let port: Arc<dyn OpenLink> = Arc::new(mock);
    let command = OpenLinkCommand::new(None, None);
    assert_eq!(port.execute(command).await.label(), "wrong key");
}
