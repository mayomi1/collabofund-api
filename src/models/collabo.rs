use crate::models::user::User;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collabo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub balance: f64,
    // pub user: User,
    pub collabo_account: Option<CollaboAccount>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollaboAccount {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub account_name: String,
    pub account_number: String,
    pub account_bank_code: String,
}
