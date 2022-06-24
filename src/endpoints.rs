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
    pub fn as_string(&self) -> String {
        match self {
            Endpoint::CREATE_PUBLIC_KEY => "/public-keys/".to_string(),
            Endpoint::CONSULT_PUBLIC_KEYS => "/public-keys/card".to_string(),
            Endpoint::UPDATE_PUBLIC_KEYS => "/public-keys/card".to_string(),
            Endpoint::CREATE_ORDER => "/orders/".to_string(),
            Endpoint::PAY_ORDER => "/orders/:orderId/pay".to_string(),
            Endpoint::CONSULT_ORDER => "/orders/:orderId".to_string(),
            Endpoint::CREATE_CHARGE => "/charges".to_string(),
        }
    }
}
