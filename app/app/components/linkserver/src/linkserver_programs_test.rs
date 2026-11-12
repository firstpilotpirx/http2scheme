use std::path::PathBuf;

use super::LinkserverPrograms;

#[test]
fn a_copy_keeps_every_program() {
    let programs = LinkserverPrograms {
        open: PathBuf::from("/usr/bin/open"),
        launchctl: PathBuf::from("/bin/launchctl"),
        osascript: PathBuf::from("/usr/bin/osascript"),
        lsappinfo: PathBuf::from("/usr/bin/lsappinfo"),
    };
    let copy = programs.clone();
    assert_eq!(copy.open, programs.open);
    assert_eq!(copy.launchctl, programs.launchctl);
    assert_eq!(copy.osascript, programs.osascript);
    assert_eq!(copy.lsappinfo, programs.lsappinfo);
}
