use std::path::Path;
use std::sync::Arc;

use super::{MockSkillRepository, SkillRepository};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockSkillRepository::new();
    mock.expect_find().returning(|_| Ok((None, None)));
    mock.expect_save().times(1).returning(|_, _| Ok(()));
    let port: Arc<dyn SkillRepository> = Arc::new(mock);
    assert_eq!(port.find(Path::new("/c")).await.unwrap(), (None, None));
    assert!(port.save(Path::new("/c"), "text").await.is_ok());
}
