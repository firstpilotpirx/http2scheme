//! What the app hands the component: folders, names, the address and the system programs.

use std::path::PathBuf;

use crate::adapters::primary::linkopening::http::open_link_response::OpenLinkResponse;
use crate::core::domain::agent_definition_value_object::AgentDefinitionValueObject;
use crate::linkserver_programs::LinkserverPrograms;

const SETTINGS_FILE: &str = "settings.toml";
const REQUEST_LOG: &str = "requests.log";

/// The folders, names and programs the app hands in; the component holds no machine literal.
#[derive(Debug, Clone)]
pub struct LinkserverSetup {
    /// The folder of `settings.toml` and `key`.
    pub settings_dir: PathBuf,
    /// The folder of the request log and the agent's output.
    pub log_dir: PathBuf,
    /// The folder of the per-user agent definitions.
    pub agents_dir: PathBuf,
    /// The program the agent runs.
    pub program: PathBuf,
    /// The agent label.
    pub label: String,
    /// The address the server binds and the printed links name.
    pub host: String,
    /// The host names the server serves and whose tabs it closes.
    pub hosts: Vec<String>,
    /// The program version.
    pub version: String,
    /// The user id of the agent's launchd domain.
    pub uid: u32,
    /// The system programs the adapters run.
    pub programs: LinkserverPrograms,
}

impl LinkserverSetup {
    pub(crate) fn settings_file(&self) -> String {
        self.settings_dir.join(SETTINGS_FILE).display().to_string()
    }

    pub(crate) fn request_log(&self) -> PathBuf {
        self.log_dir.join(REQUEST_LOG)
    }

    pub(crate) fn definition(&self) -> AgentDefinitionValueObject {
        AgentDefinitionValueObject::new(
            self.label.clone(),
            self.program.clone(),
            self.log_dir.clone(),
        )
    }

    pub(crate) fn pages(&self) -> OpenLinkResponse {
        OpenLinkResponse::new(
            self.settings_file(),
            self.request_log().display().to_string(),
        )
    }
}

#[cfg(test)]
#[path = "linkserver_setup_test.rs"]
mod linkserver_setup_test;
