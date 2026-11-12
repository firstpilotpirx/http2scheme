use super::OsRandomSecretGeneratorGateway;
use crate::core::application::services::secret_generator_service::SecretGeneratorService;

#[tokio::test]
async fn two_keys_differ() {
    let first = OsRandomSecretGeneratorGateway.generate_key().await.unwrap();
    let second = OsRandomSecretGeneratorGateway.generate_key().await.unwrap();
    assert_eq!(first.to_hex().len(), 64);
    assert_ne!(first.to_hex(), second.to_hex());
}
