use super::MenuCommandEnum;

#[test]
fn each_command_differs_from_the_others() {
    assert_ne!(MenuCommandEnum::Show, MenuCommandEnum::ToggleLoginItem);
    assert_ne!(MenuCommandEnum::Show, MenuCommandEnum::ToggleServer);
    assert_ne!(
        MenuCommandEnum::ToggleServer,
        MenuCommandEnum::ToggleLoginItem
    );
}
