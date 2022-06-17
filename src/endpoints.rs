pub enum Endpoint {
    CREATE_PUBLIC_KEY,
}

impl Endpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            Endpoint::CREATE_PUBLIC_KEY => "/public-keys/",
        }
    }
}
