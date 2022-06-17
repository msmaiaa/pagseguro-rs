#[allow(non_camel_case_types)]
pub enum Endpoint {
    CREATE_PUBLIC_KEY,
    CONSULT_PUBLIC_KEYS,
}

impl Endpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            Endpoint::CREATE_PUBLIC_KEY => "/public-keys/",
            Endpoint::CONSULT_PUBLIC_KEYS => "/public-keys/card",
        }
    }
}
