use crate::endpoints::Endpoint;
use reqwest::header;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct HttpClient {
    _client: reqwest::Client,
    _base_url: String,
}
impl HttpClient {
    pub fn new(base_url: String, headers: header::HeaderMap) -> HttpClient {
        let _client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        HttpClient {
            _client,
            _base_url: base_url,
        }
    }

    pub async fn post<T: serde::Serialize>(
        self,
        endpoint: Endpoint,
        body: Option<T>,
    ) -> reqwest::Response {
        let mut response = self
            ._client
            .post(format!("{}{}", self._base_url, endpoint.as_str()));
        if let Some(body) = body {
            response = response.json(&body);
        };
        response.send().await.unwrap()
    }

    pub async fn get(self, endpoint: Endpoint) -> reqwest::Response {
        self._client
            .get(format!("{}{}", self._base_url, endpoint.as_str()))
            .send()
            .await
            .unwrap()
    }

    pub async fn put<T: serde::Serialize>(
        self,
        endpoint: Endpoint,
        body: Option<T>,
    ) -> reqwest::Response {
        let mut response = self
            ._client
            .put(format!("{}{}", self._base_url, endpoint.as_str()));
        if let Some(body) = body {
            response = response.json(&body);
        }
        response.send().await.unwrap()
    }
}
#[derive(Debug)]
pub struct HttpError {
    pub status: u16,
    pub message: PagseguroError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PagseguroErrorMessage {
    pub code: String,
    pub description: String,
    pub parameter_name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PagseguroError {
    pub error_messages: Vec<PagseguroErrorMessage>,
}
