use crate::data::account::AccountType;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LoginData {
    username: String,
    password: String,
}

impl LoginData {
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Deserialize, Serialize)]
pub struct RegisterData {
    username: String,
    password: String,
    account_type: AccountType,
}
