//! `LsAppInfoAppOrderGateway` with the real `lsappinfo`, which only reads, and a recording `open`.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

mod support;

use std::path::PathBuf;

use linkserver::adapters::secondary::apporder::lsappinfo::ls_app_info_app_order_gateway::LsAppInfoAppOrderGateway;
use linkserver::core::application::exceptions::open_link_error::OpenLinkError;
use support::RecordingProgram;
use tempfile::TempDir;

const LSAPPINFO: &str = "/usr/bin/lsappinfo";

#[tokio::test]
async fn the_real_list_either_names_no_left_app_or_activates_one_by_its_bundle_id() {
    let dir = TempDir::new().unwrap();
    let open = RecordingProgram::new(dir.path(), "open", 0);
    let gateway = LsAppInfoAppOrderGateway::new(PathBuf::from(LSAPPINFO), open.program.clone());
    match gateway.bring_back_now().await {
        Err(OpenLinkError::ReorderFailed { reason }) => {
            assert!(open.calls().is_empty(), "{reason}");
        }
        Ok(()) => {
            let calls = open.calls();
            assert_eq!(calls.len(), 1);
            assert_eq!(calls[0].len(), 2);
            assert_eq!(calls[0][0], "-b");
            assert!(calls[0][1].contains('.'), "{:?}", calls[0]);
        }
        Err(other) => panic!("{other}"),
    }
}

#[tokio::test]
async fn a_missing_lsappinfo_is_a_reorder_failure_and_opens_nothing() {
    let dir = TempDir::new().unwrap();
    let open = RecordingProgram::new(dir.path(), "open", 0);
    let gateway = LsAppInfoAppOrderGateway::new(dir.path().join("lsappinfo"), open.program.clone());
    let error = gateway.bring_back_now().await.unwrap_err();
    assert!(
        matches!(error, OpenLinkError::ReorderFailed { .. }),
        "{error}"
    );
    assert_eq!(open.calls(), Vec::<Vec<String>>::new());
}
