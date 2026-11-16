use std::path::PathBuf;

use super::AgentsCommandEnum;

#[test]
fn each_command_differs_from_the_others() {
    let folder = PathBuf::from("/c");
    assert_ne!(
        AgentsCommandEnum::Show(Some(folder.clone())),
        AgentsCommandEnum::Install(folder.clone())
    );
    assert_ne!(
        AgentsCommandEnum::Install(folder.clone()),
        AgentsCommandEnum::Copy(folder)
    );
}
