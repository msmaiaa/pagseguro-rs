use serde::{Deserialize, Serialize};

use crate::http::PagseguroError;

#[derive(Debug)]
pub enum SDKError {
    PagseguroError(PagseguroError),
    RequestError(reqwest::Error),
    Unauthorized,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Phone {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Customer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub email: String,
    pub tax_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Vec<Phone>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderItem {
    pub name: String,
    pub quantity: u32,
    pub unit_amount: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub street: String,
    pub number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complement: Option<String>,
    pub locality: String,
    pub city: String,
    pub region_code: String,
    pub region: String,
    ///	ISO 3166-2 format (https://en.wikipedia.org/wiki/ISO_3166-2)
    pub country: String,
    pub postal_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderShipping {
    pub address: Address,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QRcode {
    pub amount: Amount,
    #[serde(rename = "amount.value")]
    pub amount_value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Summary {
    pub total: u32,
    pub paid: u32,
    pub refunded: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Amount {
    pub value: u32,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Summary>,
}

// TODO: create a more specific payment response for each payment method?
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentResponse {
    pub code: String,
    pub message: String,
    ///	NSU number, in the case that the payment has been approved by the issuer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardHolder {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoletoHolder {
    pub name: String,
    pub tax_id: String,
    pub email: String,
    pub address: Address,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    /// only available on credit cards
    pub id: String,
    pub number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_token: Option<String>,
    pub exp_month: u8,
    pub exp_year: u8,
    pub security_code: String,
    pub store: bool,
    pub brand: String,
    pub first_digits: u32,
    pub last_digits: u16,
    pub holder: CardHolder,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreditCard {
    pub number: String,
    pub exp_month: String,
    pub exp_year: String,
    pub security_code: String,
    pub holder: CardHolder,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenData {
    pub requestor_id: String,
    pub wallet: String,
    pub cryptogram: String,
    pub ecommerce_domain: String,
    pub assurance_level: i8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticationMethod {
    #[serde(rename = "type")]
    pub _type: String,
    pub cavv: String,
    pub eci: String,
    pub xid: String,
    pub version: String,
    pub dstrans_id: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoletoInstructionLines {
    pub line_1: String,
    pub line_2: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Boleto {
    pub due_date: String,
    pub instruction_lines: BoletoInstructionLines,
    pub holder: BoletoHolder,
}

// TODO: refactor me
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentMethod {
    #[serde(rename = "type")]
    pub _type: String,
    /// Required it the type is CREDIT_CARD
    pub installments: Option<u8>,
    /// Required it the type is CREDIT_CARD
    pub capture: Option<bool>,
    /// Only when using a credit card
    pub soft_descriptor: Option<String>,
    /// Required when using card or network token
    pub card: Option<Card>,
    ///	Additional network tokenization data. Must be sent when a credit or debit card tokenized by the Visa or Mastercard brands
    pub token_data: Option<TokenData>,
    /// Required when type == DEBIT_CARD
    pub authentication_method: Option<AuthenticationMethod>,
    /// Required when the payment method is boleto
    pub boleto: Option<Boleto>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreditCardPayment {
    #[serde(rename = "type")]
    pub _type: String,
    pub installments: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_before: Option<String>,
    pub capture: bool,
    pub soft_descriptor: String,
    pub card: CreditCard,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoletoPayment {
    #[serde(rename = "type")]
    pub _type: String,
    pub boleto: Boleto,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recurring {
    #[serde(rename = "type")]
    _type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubMerchant {
    pub reference_id: String,
    pub name: String,
    pub tax_id: String,
    pub mcc: String,
    pub address: Address,
    pub hones: Vec<Phone>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub rel: String,
    pub href: String,
    pub media: String,
    #[serde(rename = "type")]
    pub _type: String,
}

/// TODO: refactor me
#[derive(Serialize, Deserialize, Debug)]
pub struct Charge {
    pub id: String,
    ///	- AUTHORIZED : Indicates that the charge is pre-authorized
    ///	- PAID : Indicates that the charge is paid (captured).
    /// - DECLINED : Indicates that the charge was denied by Pagseguro or by the issuer
    /// - CANCELED : Indicates that the charge was canceled
    pub status: String,
    //TODO: handle date, should be Datetime
    pub created_at: String,
    pub paid_at: String,
    pub reference_id: String,
    pub description: String,
    pub amount: Amount,
    pub payment_response: PaymentResponse,
    pub payment_method: PaymentMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<Recurring>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_merchant: Option<SubMerchant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoletoCharge {
    pub reference_id: String,
    pub description: String,
    pub amount: Amount,
    pub payment_method: BoletoPayment,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_urls: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardCharge {
    pub reference_id: String,
    pub description: String,
    pub amount: Amount,
    pub payment_method: CreditCardPayment,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_urls: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    pub customer: Customer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<OrderItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<OrderShipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_codes: Option<QRcode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges: Option<Vec<Charge>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExistingOrder {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<OrderShipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<OrderItem>>,
    pub customer: Customer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges: Option<Vec<Charge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_codes: Option<Vec<QRcode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_urls: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Application {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub account_id: String,
    pub client_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
}
