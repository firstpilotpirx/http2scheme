//! `http2scheme serve`.

use std::future::Future;
use std::io::{ErrorKind, Write};
use std::sync::Arc;

use axum::Router;

use crate::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use crate::adapters::primary::linkopening::http::axum_config::AxumConfig;
use crate::core::application::use_cases::prepare_settings::PrepareSettings;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;
use crate::core::domain::scheme_name_value_object::SchemeNameValueObject;
use crate::core::domain::server_settings_value_object::ServerSettingsValueObject;

/// Builds the HTTP server for the prepared settings and key.
pub type ServerFactory = Arc<
    dyn Fn(ServerSettingsValueObject, AccessKeyValueObject) -> (AxumConfig, Router) + Send + Sync,
>;

/// Prepares the settings and the key, then serves until stopped.
pub struct ServeCommand {
    use_case: Arc<dyn PrepareSettings>,
    factory: ServerFactory,
    version: String,
    log_file: String,
    style: TerminalStyleConfig,
}

impl ServeCommand {
    /// A command that builds its server with `factory` and names `version` and the request `log_file`.
    #[must_use]
    pub fn new(
        use_case: Arc<dyn PrepareSettings>,
        factory: ServerFactory,
        version: String,
        log_file: String,
        style: TerminalStyleConfig,
    ) -> Self {
        Self {
            use_case,
            factory,
            version,
            log_file,
            style,
        }
    }

    /// Runs the server until `shutdown` completes; the exit status is 0 after a clean stop.
    pub async fn run(
        &self,
        shutdown: impl Future<Output = ()> + Send + 'static,
        out: &mut (dyn Write + Send),
        err: &mut (dyn Write + Send),
    ) -> u8 {
        let (settings, key) = match self.use_case.execute().await {
            Ok(prepared) => prepared,
            Err(error) => return self.fail(err, &error.to_string()),
        };
        let port = settings.port().value();
        let schemes = Self::schemes(&settings);
        let (http, router) = (self.factory)(settings, key);
        let listener = match http.listen().await {
            Ok(listener) => listener,
            Err(error) if error.kind() == ErrorKind::AddrInUse => {
                return self.fail(err, &format!("port {port} is taken"));
            }
            Err(error) => {
                let reason = format!("cannot listen on {}: {error}", http.address());
                return self.fail(err, &reason);
            }
        };
        self.banner(out, &http.address(), &schemes);
        match AxumConfig::serve(listener, router, shutdown).await {
            Ok(()) => 0,
            Err(error) => self.fail(err, &error.to_string()),
        }
    }

    fn schemes(settings: &ServerSettingsValueObject) -> String {
        let names: Vec<&str> = settings
            .allowlist()
            .schemes()
            .iter()
            .map(SchemeNameValueObject::as_str)
            .collect();
        names.join(", ")
    }

    fn banner(&self, out: &mut (dyn Write + Send), address: &str, schemes: &str) {
        let title = self.style.title(&format!("http2scheme {}", self.version));
        let _ = writeln!(out, "{title} listening on {}", self.style.path(address));
        let detail = format!("  allowed schemes: {schemes} · log: {}", self.log_file);
        let _ = writeln!(out, "{}", self.style.detail(&detail));
        let _ = out.flush();
    }

    fn fail(&self, err: &mut (dyn Write + Send), reason: &str) -> u8 {
        let _ = writeln!(err, "{}", self.style.fail(reason));
        1
    }
}
