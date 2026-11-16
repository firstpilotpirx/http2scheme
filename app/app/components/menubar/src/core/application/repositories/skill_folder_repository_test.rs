use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::{MockSkillFolderRepository, SkillFolderRepository};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockSkillFolderRepository::new();
    mock.expect_find().returning(|| Ok(Some(PathBuf::from("/c"))));
    mock.expect_save().times(1).returning(|_| Ok(()));
    let port: Arc<dyn SkillFolderRepository> = Arc::new(mock);
    assert_eq!(port.find().await.unwrap(), Some(PathBuf::from("/c")));
    assert!(port.save(Path::new("/c")).await.is_ok());
}
