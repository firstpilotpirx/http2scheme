use super::AgentSkillValueObject;

const PROGRAM: &str = "/Applications/http2scheme.app/Contents/MacOS/http2scheme";

#[test]
fn keeps_the_header_naming_the_skill() {
    let text = AgentSkillValueObject::for_program(PROGRAM)
        .text()
        .to_owned();
    assert!(
        text.starts_with("---\nname: http2scheme\ndescription: "),
        "{text}"
    );
}

#[test]
fn names_the_program_everywhere() {
    let text = AgentSkillValueObject::for_program(PROGRAM)
        .text()
        .to_owned();
    assert!(!text.contains("{{program}}"), "{text}");
    assert!(
        text.contains(&format!("'{PROGRAM}' link 'idea://")),
        "{text}"
    );
}

#[test]
fn shows_a_file_link_followed_by_an_ide_link() {
    let skill = AgentSkillValueObject::for_program(PROGRAM);
    assert!(
        skill
            .text()
            .contains("(app/src/server.rs:42) [IDE](http://127.0.0.1:7777/open?")
    );
}

#[test]
fn two_programs_give_two_texts() {
    assert_ne!(
        AgentSkillValueObject::for_program("/a/http2scheme"),
        AgentSkillValueObject::for_program("/b/http2scheme")
    );
}
