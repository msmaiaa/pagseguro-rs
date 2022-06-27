use serde::{Deserialize, Serialize};

use crate::common_types::Charge;

#[derive(Serialize, Deserialize, Debug)]
pub struct PayOrder {
    pub charges: Vec<Charge>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateApplication {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
}
