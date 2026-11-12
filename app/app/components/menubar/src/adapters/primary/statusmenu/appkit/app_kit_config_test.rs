use super::AppKitConfig;

#[test]
fn a_test_thread_is_not_the_main_thread() {
    assert!(AppKitConfig::main_thread().is_none());
}
