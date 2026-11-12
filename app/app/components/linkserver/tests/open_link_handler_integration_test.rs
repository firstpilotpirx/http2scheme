//! `OpenLinkHandler` on a real axum server with a fake open-link use case.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

use std::net::{IpAddr, Ipv4Addr, TcpListener as StdListener};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use linkserver::adapters::primary::linkopening::http::axum_config::AxumConfig;
use linkserver::adapters::primary::linkopening::http::open_link_handler::OpenLinkHandler;
use linkserver::adapters::primary::linkopening::http::open_link_response::OpenLinkResponse;
use linkserver::core::application::use_cases::open_link::OpenLink;
use linkserver::core::application::use_cases::open_link_command::OpenLinkCommand;
use linkserver::core::domain::link_rejected_error::LinkRejectedError;
use linkserver::core::domain::open_outcome_enum::OpenOutcomeEnum;
use linkserver::core::domain::port_value_object::PortValueObject;

struct FakeOpenLink {
    outcome: OpenOutcomeEnum,
    commands: Mutex<Vec<OpenLinkCommand>>,
}

#[async_trait]
impl OpenLink for FakeOpenLink {
    async fn execute(&self, command: OpenLinkCommand) -> OpenOutcomeEnum {
        self.commands.lock().unwrap().push(command);
        self.outcome.clone()
    }
}

struct Server {
    port: u16,
    use_case: Arc<FakeOpenLink>,
}

impl Server {
    async fn start(outcome: OpenOutcomeEnum) -> Self {
        let port = StdListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let use_case = Arc::new(FakeOpenLink {
            outcome,
            commands: Mutex::new(Vec::new()),
        });
        let pages = OpenLinkResponse::new(
            "/home/settings.toml".to_owned(),
            "/home/requests.log".to_owned(),
        );
        let handler = OpenLinkHandler::new(use_case.clone(), pages);
        let hosts = vec!["127.0.0.1".to_owned(), "localhost".to_owned()];
        let config = AxumConfig::new(
            "127.0.0.1".to_owned(),
            hosts,
            PortValueObject::new(port).unwrap(),
        );
        let listener = config.listen().await.unwrap();
        assert_eq!(
            listener.local_addr().unwrap().ip(),
            IpAddr::V4(Ipv4Addr::LOCALHOST)
        );
        tokio::spawn(AxumConfig::serve(
            listener,
            config.router(handler),
            std::future::pending(),
        ));
        Self { port, use_case }
    }

    fn url(&self, path: &str) -> String {
        format!("http://127.0.0.1:{}{path}", self.port)
    }

    fn commands(&self) -> Vec<OpenLinkCommand> {
        self.use_case.commands.lock().unwrap().clone()
    }

    async fn get(&self, path: &str) -> (u16, String) {
        answer(
            reqwest::Client::new()
                .get(self.url(path))
                .send()
                .await
                .unwrap(),
        )
        .await
    }
}

async fn answer(response: reqwest::Response) -> (u16, String) {
    let status = response.status().as_u16();
    let headers = response.headers().clone();
    assert_eq!(headers["cache-control"], "no-store");
    assert_eq!(headers["referrer-policy"], "no-referrer");
    (status, response.text().await.unwrap())
}

const OPEN: &str = "/open?key=k1&url=idea%3A%2F%2Fopen%3Ffile%3D%2Fa%2520b%26line%3D17";

#[tokio::test]
async fn an_opened_link_gets_the_opened_page() {
    let server = Server::start(OpenOutcomeEnum::Opened).await;
    let (status, body) = server.get(OPEN).await;
    assert_eq!(status, 200);
    assert!(body.contains("Opened"), "{body}");
    assert!(
        body.contains("idea://open?file=/a%20b&amp;line=17"),
        "{body}"
    );
    assert!(body.contains("window.close()"), "{body}");
    assert_eq!(
        server.commands(),
        [OpenLinkCommand::new(
            Some("k1".to_owned()),
            Some("idea://open?file=/a%20b&line=17".to_owned())
        )]
    );
}

#[tokio::test]
async fn each_refusal_gets_its_status_and_page() {
    let not_allowed = OpenOutcomeEnum::Rejected(LinkRejectedError::SchemeNotAllowed {
        scheme: "<b>".to_owned(),
    });
    let cases = [
        (OpenOutcomeEnum::WrongKey, 403, "Wrong key"),
        (
            OpenOutcomeEnum::Rejected(LinkRejectedError::NoScheme),
            400,
            "Malformed link",
        ),
        (not_allowed, 400, "Scheme not allowed: &lt;b&gt;"),
        (OpenOutcomeEnum::OpenFailed, 502, "Could not open the app"),
    ];
    for (outcome, expected_status, expected_text) in cases {
        let server = Server::start(outcome).await;
        let (status, body) = server.get(OPEN).await;
        assert_eq!(status, expected_status, "{body}");
        assert!(
            body.contains(expected_text),
            "{expected_text} missing in {body}"
        );
        assert!(!body.contains("<b>"), "{body}");
        assert!(!body.contains("window.close()"), "{body}");
    }
}

#[tokio::test]
async fn a_request_without_parameters_reaches_the_use_case_empty() {
    let server = Server::start(OpenOutcomeEnum::WrongKey).await;
    server.get("/open").await;
    assert_eq!(server.commands(), [OpenLinkCommand::new(None, None)]);
}

#[tokio::test]
async fn an_unknown_address_is_not_found() {
    let server = Server::start(OpenOutcomeEnum::Opened).await;
    let (status, body) = server.get("/nope").await;
    assert_eq!(status, 404);
    assert!(body.contains("Not found"), "{body}");
    assert_eq!(server.commands().len(), 0);
}

#[tokio::test]
async fn another_method_is_not_allowed() {
    let server = Server::start(OpenOutcomeEnum::Opened).await;
    let (status, body) = answer(
        reqwest::Client::new()
            .post(server.url(OPEN))
            .send()
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, 405);
    assert!(body.contains("Method not allowed"), "{body}");
    assert_eq!(server.commands().len(), 0);
}

#[tokio::test]
async fn a_foreign_host_is_misdirected_and_localhost_is_served() {
    let server = Server::start(OpenOutcomeEnum::Opened).await;
    let foreign = reqwest::Client::new()
        .get(server.url(OPEN))
        .header("host", format!("evil.example:{}", server.port));
    let (status, _) = answer(foreign.send().await.unwrap()).await;
    assert_eq!(status, 421);
    let wrong_port = reqwest::Client::new()
        .get(server.url(OPEN))
        .header("host", "127.0.0.1:1");
    assert_eq!(answer(wrong_port.send().await.unwrap()).await.0, 421);
    assert_eq!(server.commands().len(), 0);
    let local = reqwest::Client::new()
        .get(server.url(OPEN))
        .header("host", format!("localhost:{}", server.port));
    assert_eq!(answer(local.send().await.unwrap()).await.0, 200);
}

#[tokio::test]
async fn health_answers_without_a_key() {
    let server = Server::start(OpenOutcomeEnum::Opened).await;
    let response = reqwest::Client::new()
        .get(server.url("/health"))
        .send()
        .await
        .unwrap();
    assert_eq!(
        response.headers()["content-type"],
        "text/plain; charset=utf-8"
    );
    let (status, body) = answer(response).await;
    assert_eq!((status, body.as_str()), (200, "ok"));
    assert_eq!(server.commands().len(), 0);
}
