//! `http2scheme link <scheme link>`.

use std::io::Write;
use std::sync::Arc;

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};

use crate::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use crate::core::application::exceptions::prepare_link_error::PrepareLinkError;
use crate::core::application::use_cases::prepare_link::PrepareLink;
use crate::core::domain::link_rejected_error::LinkRejectedError;
use crate::core::domain::prepared_link_value_object::PreparedLinkValueObject;

/// Every byte but the RFC 3986 unreserved characters is percent-encoded.
const QUERY_VALUE: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

/// Prints the open link for a scheme link, or the reason it is refused.
pub struct PrepareLinkCommand {
    use_case: Arc<dyn PrepareLink>,
    host: String,
    settings_file: String,
    style: TerminalStyleConfig,
}

impl PrepareLinkCommand {
    /// A command whose links point at `host`; its hint names `settings_file`.
    #[must_use]
    pub fn new(
        use_case: Arc<dyn PrepareLink>,
        host: String,
        settings_file: String,
        style: TerminalStyleConfig,
    ) -> Self {
        Self {
            use_case,
            host,
            settings_file,
            style,
        }
    }

    /// Runs the command; the exit status is 0 when a link was printed.
    pub async fn run(
        &self,
        link: &str,
        out: &mut (dyn Write + Send),
        err: &mut (dyn Write + Send),
    ) -> u8 {
        match self.use_case.execute(link).await {
            Ok(prepared) => {
                let _ = writeln!(out, "{}", self.style.path(&open_url(&self.host, &prepared)));
                0
            }
            Err(PrepareLinkError::Rejected(LinkRejectedError::SchemeNotAllowed { scheme })) => {
                let _ = writeln!(
                    err,
                    "{}",
                    self.style.fail(&format!("scheme not allowed: {scheme}"))
                );
                let hint = format!("  add \"{scheme}\" to allowed_schemes in ");
                let _ = writeln!(
                    err,
                    "{}{}",
                    self.style.detail(&hint),
                    self.style.path(&self.settings_file)
                );
                1
            }
            Err(error) => {
                let _ = writeln!(err, "{}", self.style.fail(&error.to_string()));
                1
            }
        }
    }
}

/// `http://<host>:<port>/open?key=<key>&url=<percent-encoded link>`.
#[must_use]
pub fn open_url(host: &str, prepared: &PreparedLinkValueObject) -> String {
    format!(
        "http://{host}:{}/open?key={}&url={}",
        prepared.port().value(),
        prepared.key().to_hex(),
        utf8_percent_encode(prepared.link().as_str(), QUERY_VALUE)
    )
}

#[cfg(test)]
#[path = "prepare_link_command_test.rs"]
mod prepare_link_command_test;
