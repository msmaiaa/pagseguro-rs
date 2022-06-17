use serde::{Deserialize, Serialize};

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
