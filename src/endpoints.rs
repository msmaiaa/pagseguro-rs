#[allow(non_camel_case_types)]
pub enum Endpoint {
    CREATE_PUBLIC_KEY,
    CONSULT_PUBLIC_KEYS,
    UPDATE_PUBLIC_KEYS,
    CREATE_ORDER,
    PAY_ORDER,
    CONSULT_ORDER,
    CREATE_CHARGE,
}

impl Endpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            Endpoint::CREATE_PUBLIC_KEY => "/public-keys/",
            Endpoint::CONSULT_PUBLIC_KEYS => "/public-keys/card",
            Endpoint::UPDATE_PUBLIC_KEYS => "/public-keys/card",
            Endpoint::CREATE_ORDER => "/orders/",
            Endpoint::PAY_ORDER => "/orders/:orderId/pay",
            Endpoint::CONSULT_ORDER => "/orders/:orderId",
            Endpoint::CREATE_CHARGE => "/charges",
        }
    }
}
