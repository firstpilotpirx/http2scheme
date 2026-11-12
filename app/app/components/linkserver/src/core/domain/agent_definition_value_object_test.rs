use std::path::PathBuf;

use super::AgentDefinitionValueObject;

#[test]
fn keeps_its_parts_and_runs_serve() {
    let definition = AgentDefinitionValueObject::new(
        "io.example.agent".to_owned(),
        PathBuf::from("/bin/tool"),
        PathBuf::from("/logs"),
    );
    assert_eq!(definition.label(), "io.example.agent");
    assert_eq!(definition.program(), PathBuf::from("/bin/tool").as_path());
    assert_eq!(definition.arguments(), ["serve"]);
    assert_eq!(
        definition.stdout_log(),
        PathBuf::from("/logs/agent.out.log")
    );
    assert_eq!(
        definition.stderr_log(),
        PathBuf::from("/logs/agent.err.log")
    );
}
