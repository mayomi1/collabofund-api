use crate::providers::pooler_api::{GeneratedAccountResponse, PoolerResponse};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collabo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub balance: f64,
    pub user_id: String,
    pub collabo_account_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollaboAccount {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub account_name: String,
    pub account_number: String,
    pub provider: String,
    pub account_status: bool,
    pub provider_bank: String,
    pub collabo_id: String,
    pub(crate) metadata: PoolerResponse<GeneratedAccountResponse>,
}
