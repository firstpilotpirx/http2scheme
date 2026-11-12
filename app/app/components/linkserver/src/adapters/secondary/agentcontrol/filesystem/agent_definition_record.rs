//! The `LaunchAgent` property list as it is stored.

use std::fmt::Write as _;

use crate::core::domain::agent_definition_value_object::AgentDefinitionValueObject;

/// The property list file name for a label.
#[must_use]
pub fn file_name(label: &str) -> String {
    format!("{label}.plist")
}

/// The XML property list that runs the agent at login and keeps it alive.
#[must_use]
pub fn to_plist(definition: &AgentDefinitionValueObject) -> String {
    let mut arguments = String::new();
    let program = definition.program().display().to_string();
    for argument in std::iter::once(program.as_str()).chain(definition.arguments()) {
        let _ = writeln!(arguments, "        <string>{}</string>", escape(argument));
    }
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
    <key>Label</key>
    <string>{label}</string>
    <key>ProgramArguments</key>
    <array>
{arguments}    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>{stdout}</string>
    <key>StandardErrorPath</key>
    <string>{stderr}</string>
</dict>
</plist>
",
        label = escape(definition.label()),
        stdout = escape(&definition.stdout_log().display().to_string()),
        stderr = escape(&definition.stderr_log().display().to_string()),
    )
}

/// The text with the XML special characters escaped.
#[must_use]
pub fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
#[path = "agent_definition_record_test.rs"]
mod agent_definition_record_test;
