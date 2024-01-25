use crate::api::collabo::GenerateAccountParams;
use crate::clients::pooler_api_client::PoolerApiClient;
use crate::models::collabo::CollaboAccount;
use crate::repository::collabo::UpdateCollaboAccountId;
use crate::repository::mongodb_repo::MongoRepo;
use actix_web::web::Data;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

pub struct PoolerProvider;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneratedAccountResponse {
    account_no: String,
    bvn: String,
    bank_name: String,
    parent_id: i32,
    currency_code: String,
    display_name: String,
    status: bool,
    first_name: String,
    last_name: String,
    mobile_number: String,
    amount: Option<f64>,
    email: String,
    account_type: String,
    checkout_account: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct PoolerError {
    status: String,
    message: String,
    data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoolerResponse<T> {
    #[serde(rename = "status")]
    status: String,
    #[serde(rename = "message")]
    message: String,
    #[serde(rename = "data")]
    data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ApiResponse {
    Error(PoolerError),
    Success(PoolerResponse<GeneratedAccountResponse>),
}

impl PoolerProvider {
    pub async fn generate_account(
        body: &GenerateAccountParams,
        db: Data<MongoRepo>,
        collabo_id: &str,
    ) -> HttpResponse {
        let pooler_client = PoolerApiClient::new();

        println!("Generating account body: {:?}", body);
        match pooler_client
            .post::<ApiResponse, GenerateAccountParams>("/business/accounts", &body)
            .await
        {
            Ok(api_response) => {
                match api_response {
                    ApiResponse::Success(response) => {
                        println!("Success Response: {:?}", response);

                        let generated_account = response.data.clone().unwrap();
                        let saved_collabo_data = db
                            .collabo_account_repo
                            .create_collabo_account(CollaboAccount {
                                id: None,
                                account_name: generated_account.display_name,
                                account_number: generated_account.account_no,
                                provider: "pooler".to_string(),
                                account_status: true,
                                collabo_id: String::from(collabo_id),
                                provider_bank: generated_account.bank_name,
                                metadata: response.clone(),
                            })
                            .await
                            .expect("Error creating collabo account");

                        println!("saved_collabo_data {:?} ", saved_collabo_data);
                        // Update collabo with accoun Id
                        db.collabo_repo
                            .update_collabo_account_id(
                                collabo_id,
                                UpdateCollaboAccountId {
                                    collabo_account_id: saved_collabo_data
                                        .inserted_id
                                        .as_object_id()
                                        .unwrap_or_default()
                                        .to_string(),
                                },
                            )
                            .await
                            .expect("Error updating collabo");

                        HttpResponse::Ok().json(response)
                    }
                    ApiResponse::Error(error_response) => {
                        eprintln!("Error Response: {:?}", error_response);
                        let error_message = error_response.data.unwrap_or_default();
                        HttpResponse::InternalServerError().body(error_message)
                    }
                }
            }
            Err(e) => {
                eprintln!("Request failed >>>>>>>>: {:?}", e);
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }
}
