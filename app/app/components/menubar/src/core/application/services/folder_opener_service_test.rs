use std::path::Path;
use std::sync::Arc;

use super::{FolderOpenerService, MockFolderOpenerService};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockFolderOpenerService::new();
    mock.expect_open()
        .withf(|folder| folder == Path::new("/x/Logs"))
        .returning(|_| Ok(()));
    let port: Arc<dyn FolderOpenerService> = Arc::new(mock);
    assert_eq!(port.open(Path::new("/x/Logs")).await, Ok(()));
}
