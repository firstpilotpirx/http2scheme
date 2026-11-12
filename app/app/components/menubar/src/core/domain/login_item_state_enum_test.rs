use super::LoginItemStateEnum::{self, NeedsApproval, Off, On, Unavailable};

#[test]
fn only_an_enabled_login_item_is_checked() {
    let checked: Vec<bool> = [On, Off, NeedsApproval, Unavailable]
        .map(LoginItemStateEnum::is_checked)
        .into();
    assert_eq!(checked, [true, false, false, false]);
}

#[test]
fn only_a_program_outside_a_bundle_cannot_choose_it() {
    let available: Vec<bool> = [On, Off, NeedsApproval, Unavailable]
        .map(LoginItemStateEnum::is_available)
        .into();
    assert_eq!(available, [true, true, true, false]);
}

#[test]
fn a_login_item_waiting_for_approval_says_so() {
    assert_eq!(
        NeedsApproval.title(),
        "Start at Login (approve in System Settings)"
    );
    assert_eq!(On.title(), "Start at Login");
    assert_eq!(Unavailable.title(), "Start at Login");
}

#[test]
fn each_login_state_has_its_note() {
    let notes: Vec<&str> = [On, Off, NeedsApproval, Unavailable]
        .map(LoginItemStateEnum::note)
        .into();
    assert_eq!(
        notes,
        [
            "The app starts when you log in to the Mac.",
            "The app starts when you log in to the Mac.",
            "Approve the app in System Settings › Login Items.",
            "Only the app installed in Applications can start at login.",
        ]
    );
}
