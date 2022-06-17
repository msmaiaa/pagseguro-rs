use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePublicKey {
    pub public_key: String,
    pub created_at: u64,
}
