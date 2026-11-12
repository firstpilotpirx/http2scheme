use std::path::PathBuf;
use std::sync::Arc;

use super::OfferLoginItemUseCase;
use crate::core::application::repositories::first_launch_repository::MockFirstLaunchRepository;
use crate::core::application::services::login_item_service::MockLoginItemService;
use crate::core::application::use_cases::offer_login_item::OfferLoginItem;
use crate::core::domain::login_item_state_enum::LoginItemStateEnum;

const INSTALLED: &str = "/Applications/http2scheme.app/Contents/MacOS/http2scheme";
const BUILT: &str = "/src/dist/http2scheme.app/Contents/MacOS/http2scheme";

async fn run(program: &str, login_item: MockLoginItemService, record: MockFirstLaunchRepository) {
    let use_case = OfferLoginItemUseCase::new(
        PathBuf::from(program),
        Arc::new(login_item),
        Arc::new(record),
    );
    assert_eq!(use_case.execute().await, Ok(()));
}

fn record(exists: bool, writes: usize) -> MockFirstLaunchRepository {
    let mut mock = MockFirstLaunchRepository::new();
    mock.expect_exists().returning(move || Ok(exists));
    mock.expect_record().times(writes).returning(|| Ok(()));
    mock
}

#[tokio::test]
async fn the_first_launch_from_applications_registers_and_records() {
    let mut login_item = MockLoginItemService::new();
    login_item
        .expect_status()
        .return_const(LoginItemStateEnum::Off);
    login_item.expect_register().times(1).returning(|| Ok(()));
    run(INSTALLED, login_item, record(false, 1)).await;
}

#[tokio::test]
async fn a_recorded_first_launch_never_registers_again() {
    let mut login_item = MockLoginItemService::new();
    login_item.expect_register().never();
    run(INSTALLED, login_item, record(true, 0)).await;
}

#[tokio::test]
async fn a_launch_from_another_folder_changes_nothing() {
    let mut login_item = MockLoginItemService::new();
    login_item.expect_register().never();
    run(BUILT, login_item, record(false, 0)).await;
}

#[tokio::test]
async fn a_login_item_already_on_is_only_recorded() {
    let mut login_item = MockLoginItemService::new();
    login_item
        .expect_status()
        .return_const(LoginItemStateEnum::On);
    login_item.expect_register().never();
    run(INSTALLED, login_item, record(false, 1)).await;
}
