use super::AgentStatusValueObject;

#[test]
fn healthy_only_when_installed_loaded_and_answering() {
    assert!(AgentStatusValueObject::new(true, true, true).healthy());
    assert!(!AgentStatusValueObject::new(false, true, true).healthy());
    assert!(!AgentStatusValueObject::new(true, false, true).healthy());
    assert!(!AgentStatusValueObject::new(true, true, false).healthy());
}

#[test]
fn exposes_each_fact() {
    let status = AgentStatusValueObject::new(true, false, true);
    assert!(status.installed());
    assert!(!status.loaded());
    assert!(status.answering());
}
