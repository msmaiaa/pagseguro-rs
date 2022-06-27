mod fixtures;
use fixtures::{get_mock_boleto, get_mock_credit_card, get_sdk};
use pagseguro_rs::common_types::*;

#[tokio::test]
async fn create_credit_card_charge() {
    let card_charge = CardCharge {
        reference_id: "123".to_string(),
        description: "description".to_string(),
        amount: Amount {
            value: 1000,
            currency: "BRL".to_string(),
            summary: None,
        },
        payment_method: CreditCardPayment {
            _type: "CREDIT_CARD".to_string(),
            installments: 1,
            capture: false,
            capture_before: None,
            soft_descriptor: "loja".to_string(),
            card: get_mock_credit_card(),
        },
        notification_urls: None,
    };
    let result = get_sdk()
        .charges
        .create_credit_card_charge(card_charge)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn create_boleto_charge() {
    let boleto_charge = BoletoCharge {
        reference_id: "123".to_string(),
        description: "description".to_string(),
        amount: Amount {
            value: 1000,
            currency: "BRL".to_string(),
            summary: None,
        },
        payment_method: BoletoPayment {
            _type: "BOLETO".to_string(),
            boleto: get_mock_boleto(),
        },
        notification_urls: None,
    };
    let result = get_sdk().charges.create_boleto_charge(boleto_charge).await;
    assert!(result.is_ok());
}
