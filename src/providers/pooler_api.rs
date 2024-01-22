use std::fmt;
use actix_web::error::DispatchError::HandlerDroppedPayload;
use actix_web::error::HttpError;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use crate::api::collabo::GenerateAccountParams;
use crate::clients::pooler_api_client::PoolerApiClient;

pub struct PoolerProvider;

#[derive(Debug, Serialize, Deserialize)]
struct GeneratedAccountResponse {
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

#[derive(Debug, Serialize, Deserialize)]
struct PoolerResponse<T> {
    #[serde(rename = "status")]
    status: String,
    #[serde(rename = "message")]
    message: String,
    #[serde(rename = "data")]
    data: Option<T>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ApiResponse {
    Error(PoolerError),
    Success(PoolerResponse<GeneratedAccountResponse>),
}

impl PoolerProvider {
    pub async fn generate_account(body: &GenerateAccountParams) -> HttpResponse {
        // make api request

        // TODO:: save the result in the db;

        // TODO:: return the response

        let pooler_client = PoolerApiClient::new();

        match pooler_client.post::<ApiResponse, GenerateAccountParams>("/business/accounts", &body).await {
            Ok(api_response) => {
                match api_response {
                    ApiResponse::Success(response) => {
                        println!("Success Response: {:?}", response);
                        HttpResponse::Ok().json(response)
                    }
                    ApiResponse::Error(error_response) => {
                        eprintln!("Error Response: {:?}", error_response);
                        let error_message = error_response.data.unwrap_or_default();
                        HttpResponse::InternalServerError().body(error_message)
                    }
                }
            },
            Err(e) => {
                eprintln!("Request failed >>>>>>>>: {:?}", e);
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }



    }
}