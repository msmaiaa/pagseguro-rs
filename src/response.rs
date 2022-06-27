use serde::{Deserialize, Serialize};

use crate::common_types::{
    Amount, BoletoHolder, BoletoInstructionLines, BoletoPayment, CardHolder, Link, PaymentResponse,
};

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
    pub description: String,
    pub amount: Amount,
    pub payment_response: PaymentResponse,
    pub payment_method: BoletoPayment,
    pub links: Vec<Link>,
    pub notification_urls: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCreditCardChargeResponse {
    pub id: String,
    pub reference_id: String,
    pub status: String,
    pub created_at: String,
    pub description: String,
    pub amount: Amount,
    pub payment_response: PaymentResponse,
    pub payment_method: CreditCardPaymentResponse,
    pub notification_urls: Option<Vec<String>>,
    pub links: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreditCardResponse {
    pub brand: String,
    pub exp_month: String,
    pub exp_year: String,
    pub first_digits: String,
    pub holder: CardHolder,
    pub last_digits: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoletoResponse {
    pub id: String,
    pub barcode: String,
    pub formatted_barcode: String,
    pub due_date: String,
    pub instruction_lines: BoletoInstructionLines,
    pub holder: BoletoHolder,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreditCardPaymentResponse {
    #[serde(rename = "type")]
    pub _type: String,
    pub installments: u8,
    pub capture_before: String,
    pub capture: bool,
    pub soft_descriptor: String,
    pub card: CreditCardResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoletoPaymentResponse {
    #[serde(rename = "type")]
    pub _type: String,
    pub boleto: BoletoResponse,
}
