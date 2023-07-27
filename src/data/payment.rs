use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PayData {
    token: String,
    amount: String,
    reason: String,
    destination: String,
}
