use std::path::PathBuf;

use super::{escape, file_name, to_plist};
use crate::core::domain::agent_definition_value_object::AgentDefinitionValueObject;

#[test]
fn escapes_xml_characters() {
    assert_eq!(escape("a & <b> \"c\""), "a &amp; &lt;b&gt; &quot;c&quot;");
}

#[test]
fn the_file_is_named_after_the_label() {
    assert_eq!(file_name("io.example.agent"), "io.example.agent.plist");
}

#[test]
fn the_plist_runs_the_program_with_serve() {
    let definition = AgentDefinitionValueObject::new(
        "a.b".to_owned(),
        PathBuf::from("/bin/<x>"),
        PathBuf::from("/l"),
    );
    let plist = to_plist(&definition);
    assert!(plist.contains("<array>\n        <string>/bin/&lt;x&gt;</string>\n        <string>serve</string>\n    </array>"), "{plist}");
    assert!(
        plist.contains("<key>StandardErrorPath</key>\n    <string>/l/agent.err.log</string>"),
        "{plist}"
    );
}
