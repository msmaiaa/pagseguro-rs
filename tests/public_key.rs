mod fixtures;
use fixtures::get_sdk;

#[tokio::test]
async fn create_public_key() {
    let result = get_sdk().public_key.create_key().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn get_public_key() {
    let result = get_sdk().public_key.get_public_key().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn update_public_keys() {
    let result = get_sdk().public_key.update_public_keys().await;
    assert!(result.is_ok());
}
