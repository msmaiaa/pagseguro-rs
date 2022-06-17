mod endpoints;
mod http;
mod response;

use public_key::PublicKeyClient;
use reqwest::header;
pub enum PagseguroEnvironment {
    Sandbox,
    Production,
}

pub struct PagseguroSDK {
    pub public_key: public_key::PublicKeyClient,
}

impl PagseguroSDK {
    fn handle_headers(token: &str) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(token).unwrap(),
        );
        headers
    }

    fn handle_environment(environment: &PagseguroEnvironment) -> &'static str {
        match environment {
            PagseguroEnvironment::Sandbox => return "https://sandbox.api.pagseguro.com",
            PagseguroEnvironment::Production => return "https://api.pagseguro.com",
        };
    }

    pub fn new(token: &str, environment: PagseguroEnvironment) -> PagseguroSDK {
        let headers = PagseguroSDK::handle_headers(token);
        let environment_url = PagseguroSDK::handle_environment(&environment);

        let _client = http::HttpClient::new(environment_url.to_string(), headers);
        let _base_url = PagseguroSDK::handle_environment(&environment);
        PagseguroSDK {
            public_key: PublicKeyClient::new(_client, _base_url.to_string()),
        }
    }
}

pub mod public_key {

    use crate::{
        endpoints::Endpoint,
        http::{HttpClient, HttpError},
        response,
    };
    pub struct PublicKeyClient {
        _client: HttpClient,
    }

    impl PublicKeyClient {
        pub fn new(_client: HttpClient, _base_url: String) -> PublicKeyClient {
            PublicKeyClient { _client }
        }

        pub async fn create_key(self) -> Result<response::CreatePublicKey, HttpError> {
            let mut payload = std::collections::HashMap::new();
            payload.insert("type", "card");
            let response = self
                ._client
                .post(Endpoint::CREATE_PUBLIC_KEY, Some(payload))
                .await;

            match response.status() {
                reqwest::StatusCode::OK => {
                    Ok(response.json::<response::CreatePublicKey>().await.unwrap())
                }

                _ => Err(HttpError {
                    status: response.status().as_u16(),
                }),
            }
        }
    }
}
