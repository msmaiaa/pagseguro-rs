use public_key::PublicKeyClient;

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
    pub fn new(token: &str, environment: PagseguroEnvironment) -> PagseguroSDK {
        let _client = reqwest::Client::builder()
            .http1_title_case_headers()
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
                .header("Authorization", format!("{}", self._token))
                .header("Content-type", "application/json")
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
