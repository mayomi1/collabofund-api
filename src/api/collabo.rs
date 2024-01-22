use crate::models::collabo::Collabo;
use crate::repository::mongodb_repo::MongoRepo;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::results::InsertOneResult;
use serde::{Deserialize, Serialize};
use crate::providers::pooler_api::PoolerProvider;

#[derive(Deserialize)]
struct CollaboRequest {
    name: String,
    balance: Option<f64>,
}


#[derive(Deserialize)]
struct GenerateAccountRequest {
    bvn: String,
    mobile_number: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateAccountParams {
    account_type: String,
    account_name: String,
    first_name: String,
    last_name: String,
    display_name: String, // Collabo name
    email: String,
    bvn: String,
    mobile_number: String
}

struct GenerateAccountResponse {
    status: bool,
    message: String,
    first_name: String,
}

#[post("/new")]
pub async fn create_collabo(
    db: Data<MongoRepo>,
    collabo_request: Json<CollaboRequest>,
) -> HttpResponse {
    let data = Collabo {
        id: None,
        name: collabo_request.name.to_owned(),
        balance: 0.00,
        collabo_account: None,
    };
    let collabo_details = db.collabo_repo.create_collabo(data).await;
    match collabo_details {
        Ok(collabo) => HttpResponse::Ok().json(collabo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/generate_account")]
pub async fn generate_account (db: Data<MongoRepo>, params: Json<GenerateAccountRequest>) -> HttpResponse {

    // TODO:: fetch collabo

    // TODO:: fetch user

    let data = GenerateAccountParams {
        account_type: "individual".to_string(),
        account_name: "Test contribution collabo".to_string(), // Collabo name
        first_name: "May".to_string(), // user first name
        last_name: "soh".to_string(), // user last name
        display_name: "May soh Limited".to_string(), // collabo name
        email: "anitatom20@gmail.com".to_string(), // user email
        bvn: params.bvn.to_owned(),
        mobile_number: params.mobile_number.to_owned(),
    };

    PoolerProvider::generate_account(&data).await
}
