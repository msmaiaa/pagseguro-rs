use public_key::PublicKeyClient;
use reqwest::header;
pub enum PagseguroEnvironment {
    Sandbox,
    Production,
}

pub struct PagseguroSDK {
    _base_url: String,
    _token: String,
    _environment: PagseguroEnvironment,
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
    pub fn new(self, token: &str, environment: PagseguroEnvironment) -> PagseguroSDK {
        let headers = self::PagseguroSDK::handle_headers(token);

        let _client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let _base_url = match environment {
            PagseguroEnvironment::Sandbox => "https://sandbox.api.pagseguro.com",
            PagseguroEnvironment::Production => "https://api.pagseguro.com",
        };
        PagseguroSDK {
            _base_url: _base_url.to_string(),
            _token: token.to_string(),
            _environment: environment,
            public_key: PublicKeyClient::new(_client, _base_url.to_string(), token.to_string()),
        }
    }
}

pub mod public_key {
    use serde::{Deserialize, Serialize};
    pub struct PublicKeyClient {
        _client: reqwest::Client,
        _base_url: String,
        _token: String,
    }

    impl PublicKeyClient {
        pub fn new(_client: reqwest::Client, _base_url: String, _token: String) -> PublicKeyClient {
            PublicKeyClient {
                _client,
                _base_url,
                _token,
            }
        }

        pub async fn create_key(self) -> Result<CreatePublicKeyResponse, reqwest::Error> {
            let mut payload = std::collections::HashMap::new();
            payload.insert("type", "card");
            match self
                ._client
                .post(format!("{}/{}/", self._base_url, "public-keys"))
                .json(&payload)
                .send()
                .await
            {
                Ok(response) => Ok(response.json::<CreatePublicKeyResponse>().await?),
                Err(err) => Err(err),
            }
        }
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreatePublicKeyResponse {
        pub public_key: String,
        pub created_at: u64,
    }
}
