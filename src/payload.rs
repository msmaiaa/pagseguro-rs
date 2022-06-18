use serde::{Deserialize, Serialize};

use crate::common_types::Charge;

#[derive(Serialize, Deserialize, Debug)]
pub struct PayOrder {
    pub charges: Vec<Charge>,
}
