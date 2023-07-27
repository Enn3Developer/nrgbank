use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum AccountType {
    Business,
    Personal,
}
