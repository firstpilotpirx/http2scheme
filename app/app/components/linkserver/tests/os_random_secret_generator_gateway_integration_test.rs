//! `OsRandomSecretGeneratorGateway` against the real OS random source.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

use linkserver::adapters::secondary::secret::osrandom::os_random_secret_generator_gateway::OsRandomSecretGeneratorGateway;
use linkserver::core::application::services::secret_generator_service::SecretGeneratorService;

#[tokio::test]
async fn gives_64_hex_characters_that_differ_per_call() {
    let gateway = OsRandomSecretGeneratorGateway;
    let first = gateway.generate_key().await.unwrap().to_hex();
    let second = gateway.generate_key().await.unwrap().to_hex();
    assert_eq!(first.len(), 64);
    assert_ne!(first, second);
}
