pub mod common_types;
mod endpoints;
mod http;
pub mod payload;
mod response;

use orders::OrderClient;
use public_key::PublicKeyClient;

use reqwest::header;
pub enum PagseguroEnvironment {
    Sandbox,
    Production,
}

pub struct PagseguroSDK {
    pub public_key: public_key::PublicKeyClient,
    pub orders: orders::OrderClient,
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
            public_key: PublicKeyClient::new(_client.clone()),
            orders: OrderClient::new(_client.clone()),
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
        pub fn new(_client: HttpClient) -> PublicKeyClient {
            PublicKeyClient { _client }
        }

        pub async fn create_key(self) -> Result<response::CreatePublicKey, HttpError> {
            let mut payload = std::collections::HashMap::new();
            payload.insert("type", "card");
            let response = self
                ._client
                .post(
                    Endpoint::CREATE_PUBLIC_KEY.as_str().to_string(),
                    Some(payload),
                )
                .await;

            match response.status() {
                reqwest::StatusCode::OK => {
                    Ok(response.json::<response::CreatePublicKey>().await.unwrap())
                }

                _ => Err(HttpError {
                    status: response.status().as_u16(),
                    message: response.json().await.unwrap(),
                }),
            }
        }
        pub async fn get_public_key(self) -> Result<response::GetPublicKey, HttpError> {
            let response = self._client.get(Endpoint::CONSULT_PUBLIC_KEYS).await;
            match response.status() {
                reqwest::StatusCode::OK => {
                    Ok(response.json::<response::GetPublicKey>().await.unwrap())
                }
                _ => Err(HttpError {
                    status: response.status().as_u16(),
                    message: response.json().await.unwrap(),
                }),
            }
        }
        pub async fn update_public_keys(self) -> Result<response::UpdatePublicKeys, HttpError> {
            let response = self
                ._client
                .put(Endpoint::UPDATE_PUBLIC_KEYS, None::<serde_json::Value>)
                .await;
            match response.status() {
                reqwest::StatusCode::OK => {
                    Ok(response.json::<response::UpdatePublicKeys>().await.unwrap())
                }
                _ => Err(HttpError {
                    status: response.status().as_u16(),
                    message: response.json().await.unwrap(),
                }),
            }
        }
    }
}

pub mod orders {
    use crate::{
        common_types::{ExistingOrder, Order},
        endpoints::Endpoint,
        http::{HttpClient, HttpError},
        payload,
    };
    pub struct OrderClient {
        _client: HttpClient,
    }
    impl OrderClient {
        pub fn new(_client: HttpClient) -> OrderClient {
            OrderClient { _client }
        }

        //	TODO: create better methods to handle different kinds of orders (qr_code, boleto, card, etc)
        pub async fn create_order(self, payload: Order) -> Result<ExistingOrder, HttpError> {
            let response = self
                ._client
                .post(Endpoint::CREATE_ORDER.as_str().to_string(), Some(payload))
                .await;
            match response.status() {
                reqwest::StatusCode::OK | reqwest::StatusCode::CREATED => {
                    Ok(response.json::<ExistingOrder>().await.unwrap())
                }
                _ => Err({
                    HttpError {
                        status: response.status().as_u16(),
                        message: response.json().await.unwrap(),
                    }
                }),
            }
        }
        //	TODO: check with all possible charges
        pub async fn pay_order(
            self,
            payload: payload::PayOrder,
            order_id: &str,
        ) -> Result<ExistingOrder, HttpError> {
            let endpoint = Endpoint::PAY_ORDER.as_str().replace(":orderId", order_id);
            let response = self._client.post(endpoint, Some(payload)).await;
            match response.status() {
                reqwest::StatusCode::OK | reqwest::StatusCode::CREATED => {
                    Ok(response.json::<ExistingOrder>().await.unwrap())
                }
                _ => Err({
                    HttpError {
                        status: response.status().as_u16(),
                        message: response.json().await.unwrap(),
                    }
                }),
            }
        }
    }
}
