mod fixtures;
use fixtures::*;
use pagseguro_rs::payload::CreateApplication;

#[tokio::test]
async fn create_application() {
    let result = get_sdk()
        .connect
        .create_application(CreateApplication {
            logo: None,
            name: "aplicacao legal".to_string(),
            redirect_uri: None,
            redirect_url: None,
            webhook_url: None,
        })
        .await;
    assert!(result.is_ok());
}

// #[tokio::test]
// async fn consult_application() {
//     let result = get_sdk().connect.consult_application("12345").await;
//     assert!(result.is_ok());
// }
