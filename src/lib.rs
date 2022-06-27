pub mod common_types;
pub mod payload;

mod endpoints;
mod http;
mod response;

use charges::ChargeClient;
use common_types::SDKError;
use orders::OrderClient;
use public_key::PublicKeyClient;

use reqwest::{header, Response};
pub enum PagseguroEnvironment {
    Sandbox,
    Production,
}

pub struct PagseguroSDK {
    pub public_key: public_key::PublicKeyClient,
    pub orders: orders::OrderClient,
    pub charges: charges::ChargeClient,
}

impl PagseguroSDK {
    fn get_default_headers(token: &str) -> header::HeaderMap {
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
        let headers = PagseguroSDK::get_default_headers(token);
        let environment_url = PagseguroSDK::handle_environment(&environment);

        let http_client = http::HttpClient::new(environment_url.to_string(), headers);
        PagseguroSDK {
            public_key: PublicKeyClient::new(http_client.clone()),
            orders: OrderClient::new(http_client.clone()),
            charges: ChargeClient::new(http_client.clone()),
        }
    }
}

pub mod public_key {

    use crate::{
        common_types::SDKError, endpoints::Endpoint, handle_response_status, http::HttpClient,
        response,
    };

    pub struct PublicKeyClient {
        _client: HttpClient,
    }

    impl PublicKeyClient {
        pub fn new(http_client: HttpClient) -> PublicKeyClient {
            PublicKeyClient {
                _client: http_client,
            }
        }

        pub async fn create_key(self) -> Result<response::CreatePublicKey, SDKError> {
            let mut payload = std::collections::HashMap::new();
            payload.insert("type", "card");
            let response = self
                ._client
                .post(Endpoint::CREATE_PUBLIC_KEY.as_string(), Some(payload))
                .await;
            handle_response_status(response).await
        }

        pub async fn get_public_key(self) -> Result<response::GetPublicKey, SDKError> {
            let response = self
                ._client
                .get(Endpoint::CONSULT_PUBLIC_KEYS.as_string())
                .await;
            handle_response_status(response).await
        }

        pub async fn update_public_keys(self) -> Result<response::UpdatePublicKeys, SDKError> {
            let response = self
                ._client
                .put(Endpoint::UPDATE_PUBLIC_KEYS, None::<serde_json::Value>)
                .await;
            handle_response_status(response).await
        }
    }
}

/// # Orders
///
///	Currently not working
pub mod orders {
    use crate::{
        common_types::{ExistingOrder, Order, SDKError},
        endpoints::Endpoint,
        handle_response_status,
        http::HttpClient,
        payload,
    };

    #[derive(Clone)]
    pub struct OrderClient {
        _client: HttpClient,
    }
    impl OrderClient {
        pub fn new(_client: HttpClient) -> OrderClient {
            OrderClient { _client }
        }

        //	TODO: create better methods to handle different kinds of orders (qr_code, boleto, card, etc)
        pub async fn create_order(self, payload: Order) -> Result<ExistingOrder, SDKError> {
            let response = self
                ._client
                .post(Endpoint::CREATE_ORDER.as_string(), Some(payload))
                .await;
            handle_response_status(response).await
        }
        //	TODO: check with all possible charges
        pub async fn pay_order(
            self,
            payload: payload::PayOrder,
            order_id: &str,
        ) -> Result<ExistingOrder, SDKError> {
            let endpoint = Endpoint::PAY_ORDER
                .as_string()
                .replace(":orderId", order_id);
            let response = self._client.post(endpoint, Some(payload)).await;
            handle_response_status(response).await
        }

        pub async fn get_order(self, order_id: &str) -> Result<ExistingOrder, SDKError> {
            let endpoint = Endpoint::CONSULT_ORDER
                .as_string()
                .replace(":orderId", order_id);
            let response = self._client.get(endpoint).await;
            handle_response_status(response).await
        }
    }
}

pub mod charges {
    use serde::{de::DeserializeOwned, Serialize};

    use crate::{
        common_types::{BoletoCharge, CardCharge, SDKError},
        endpoints::Endpoint,
        handle_response_status,
        http::HttpClient,
        response::{CreateBoletoChargeResponse, CreateCreditCardChargeResponse},
    };

    #[derive(Clone)]
    pub struct ChargeClient {
        _client: HttpClient,
    }

    impl ChargeClient {
        pub fn new(http_client: HttpClient) -> ChargeClient {
            ChargeClient {
                _client: http_client,
            }
        }

        //	TODO: create more types for existing charges
        pub async fn create_boleto_charge(
            self,
            payload: BoletoCharge,
        ) -> Result<CreateBoletoChargeResponse, SDKError> {
            self.create_charge(payload).await
        }

        pub async fn create_credit_card_charge(
            self,
            payload: CardCharge,
        ) -> Result<CreateCreditCardChargeResponse, SDKError> {
            self.create_charge(payload).await
        }

        async fn create_charge<T: Serialize + DeserializeOwned, K: Serialize + DeserializeOwned>(
            self,
            payload: T,
        ) -> Result<K, SDKError> {
            let response = self
                ._client
                .post(Endpoint::CREATE_CHARGE.as_string(), Some(payload))
                .await;
            handle_response_status(response).await
        }
    }
}

async fn handle_response_status<T: serde::Serialize + serde::de::DeserializeOwned>(
    response: Response,
) -> Result<T, SDKError> {
    match response.status() {
        reqwest::StatusCode::CREATED | reqwest::StatusCode::OK => response
            .json::<T>()
            .await
            .or_else(|err| Err(SDKError::RequestError(err))),
        reqwest::StatusCode::UNAUTHORIZED => Err(SDKError::Unauthorized),
        _ => {
            let decoded_error = response.json::<http::PagseguroError>().await.unwrap();
            Err(SDKError::PagseguroError(decoded_error))
        }
    }
}
