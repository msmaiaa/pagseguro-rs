use serde::{Deserialize, Serialize};

use crate::common_types::{Amount, BoletoPayment, CreditCardPayment, Link, PaymentResponse};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePublicKey {
    pub public_key: String,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPublicKey {
    pub public_key: String,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePublicKeys {
    pub public_key: String,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateBoletoChargeResponse {
    pub id: String,
    pub reference_id: String,
    pub status: String,
    pub created_at: String,
    pub paid_at: String,
    pub description: String,
    pub amount: Amount,
    pub payment_reponse: PaymentResponse,
    pub payment_method: BoletoPayment,
    pub links: Vec<Link>,
    pub notification_url: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCreditCardPaymentResponse {
    pub id: String,
    pub reference_id: String,
    pub status: String,
    pub created_at: String,
    pub paid_at: String,
    pub description: String,
    pub amount: Amount,
    pub payment_response: PaymentResponse,
    pub payment_method: CreditCardPayment,
    pub notification_urls: Option<Vec<String>>,
    pub links: Vec<Link>,
}
