use super::TerminalStyleConfig;

fn detect(flag: bool, terminal: bool, vars: &[(&str, &str)]) -> bool {
    let vars: Vec<(String, String)> = vars
        .iter()
        .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
        .collect();
    TerminalStyleConfig::detect(flag, terminal, |name| {
        vars.iter().find(|(k, _)| k == name).map(|(_, v)| v.clone())
    })
    .color()
}

#[test]
fn a_terminal_gets_color() {
    assert!(detect(false, true, &[]));
}

#[test]
fn a_pipe_gets_no_color_unless_forced() {
    assert!(!detect(false, false, &[]));
    assert!(detect(false, false, &[("FORCE_COLOR", "1")]));
}

#[test]
fn every_off_switch_wins() {
    assert!(!detect(true, true, &[("FORCE_COLOR", "1")]));
    assert!(!detect(
        false,
        true,
        &[("NO_COLOR", "1"), ("FORCE_COLOR", "1")]
    ));
    assert!(!detect(false, true, &[("TERM", "dumb")]));
}

#[test]
fn an_empty_no_color_is_ignored() {
    assert!(detect(false, true, &[("NO_COLOR", "")]));
}

#[test]
fn plain_output_uses_bracket_marks() {
    let plain = TerminalStyleConfig::new(false);
    assert_eq!(plain.ok("a"), "[ok] a");
    assert_eq!(plain.done("b"), "[ok] b");
    assert_eq!(plain.fail("c"), "[fail] c");
    assert_eq!(plain.title("d"), "[..] d");
    assert_eq!(plain.path("/p"), "/p");
}

#[test]
fn colored_output_uses_symbols_and_escapes() {
    let colored = TerminalStyleConfig::new(true);
    assert!(colored.ok("a").contains('✓') && colored.ok("a").contains("\u{1b}["));
    assert!(colored.fail("c").contains('✗'));
    assert!(colored.title("d").contains('●'));
}

#[test]
fn colored_lines_keep_their_text() {
    let style = TerminalStyleConfig::new(true);
    assert!(style.done("installed").contains("✓ installed"));
    assert!(style.mark(false).contains('✗'));
    assert!(style.detail("a detail").contains("a detail"));
}
