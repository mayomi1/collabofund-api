use crate::models::collabo::Collabo;
use crate::repository::mongodb_repo::MongoRepo;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::results::InsertOneResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CollaboRequest {
    name: String,
    balance: Option<f64>,
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
