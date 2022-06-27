use dotenv::dotenv;
use pagseguro_rs::common_types::*;
use pagseguro_rs::*;
use std::env;

pub fn get_sdk() -> PagseguroSDK {
    dotenv().ok();
    PagseguroSDK::new(
        &env::var("PAGSEGURO_API_KEY").expect("PAGSEGURO_API_KEY is not set on .env file"),
        PagseguroEnvironment::Sandbox,
    )
}

pub fn get_mock_address() -> Address {
    Address {
        street: "Rua dos Bobos".to_string(),
        number: "0".to_string(),
        locality: "Distrito Federal".to_string(),
        complement: None,
        city: "cidade muito linda".to_string(),
        region: "Distrito Federal".to_string(),
        region_code: "DF".to_string(),
        country: "Brasil".to_string(),
        postal_code: "12345678".to_string(),
    }
}

pub fn get_mock_credit_card() -> CreditCard {
    CreditCard {
        number: "4111111111111111".to_string(),
        exp_month: "03".to_string(),
        exp_year: "2026".to_string(),
        security_code: "123".to_string(),
        holder: CardHolder {
            name: "matheus".to_string(),
            tax_id: None,
        },
    }
}

pub fn get_mock_boleto() -> Boleto {
    Boleto {
        due_date: "2024-12-31".to_string(),
        holder: BoletoHolder {
            name: "matheus".to_string(),
            tax_id: "12312312312".to_string(),
            email: "teste@mail.com".to_string(),
            address: get_mock_address(),
        },
        instruction_lines: BoletoInstructionLines {
            line_1: "aaa".to_string(),
            line_2: "bbb".to_string(),
        },
    }
}
