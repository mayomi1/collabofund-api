use crate::models::collabo::Collabo;
use crate::repository::mongodb_repo::MongoRepo;
use actix_web::{
    get, post, put,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};

use crate::models::user::User;
use crate::providers::pooler_api::PoolerProvider;
use crate::repository::collabo::UpdateCollabo;
use crate::utils::auth::protected_route;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CollaboRequest {
    name: String,
    balance: Option<f64>,
}

#[derive(Deserialize)]
pub struct GenerateAccountRequest {
    bvn: String,
    mobile_number: String,
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
    mobile_number: String,
}

struct GenerateAccountResponse {
    status: bool,
    message: String,
    first_name: String,
}

#[post("/new")]
pub async fn create_collabo(
    req: HttpRequest,
    db: Data<MongoRepo>,
    collabo_request: Json<CollaboRequest>,
) -> HttpResponse {
    let user = match protected_route(req).await {
        Ok(usr) => usr,
        Err(_) => return HttpResponse::Unauthorized().finish().into(),
    };

    println!("user data: {:?}", user);
    let data = Collabo {
        id: None,
        name: collabo_request.name.to_owned(),
        balance: 0.00,
        user_id: user.id.map(|oid| oid.to_string()).unwrap_or_default(),
        collabo_account_id: None,
    };
    let collabo_details = db.collabo_repo.create_collabo(data).await;
    match collabo_details {
        Ok(collabo) => HttpResponse::Ok().json(collabo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/generate_account/{collabo_id}")]
pub async fn generate_account(
    req: HttpRequest,
    db: Data<MongoRepo>,
    params: Json<GenerateAccountRequest>,
    path: Path<String>,
) -> HttpResponse {
    let user = match protected_route(req).await {
        Ok(usr) => usr,
        Err(_) => return HttpResponse::Unauthorized().finish().into(),
    };

    let collabo_id = path.into_inner();
    if collabo_id.is_empty() {
        return HttpResponse::BadRequest().body("Collabo id is required");
    }

    let collabo = db.collabo_repo.get_collabo_by_id(&collabo_id).await;
    match collabo {
        Ok(col) => {
            let data = GenerateAccountParams {
                account_type: "individual".to_string(),
                account_name: col.name.to_string(), // Collabo name
                first_name: user.first_name.to_string(), // user first name
                last_name: user.last_name.to_string(), // user last name
                display_name: col.name.to_string(), // collabo name
                email: user.email.to_string(),      // user email
                bvn: params.bvn.to_owned(),
                mobile_number: params.mobile_number.to_owned(),
            };

            PoolerProvider::generate_account(&data, db, &collabo_id).await
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/list")]
pub async fn fetch_collabos(req: HttpRequest, db: Data<MongoRepo>) -> HttpResponse {
    let user = match protected_route(req).await {
        Ok(usr) => usr,
        Err(_) => return HttpResponse::Unauthorized().finish().into(),
    };

    let user_id = user.id.map(|oid| oid.to_string()).unwrap_or_default();

    let collabos = db.collabo_repo.fetch_collabos(user_id).await;

    match collabos {
        Ok(collabos) => HttpResponse::Ok().json(collabos),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/{collabo_id}")]
pub async fn get_collabo(
    req: HttpRequest,
    db: Data<MongoRepo>,
    path: Path<String>,
) -> HttpResponse {
    match protected_route(req).await {
        Ok(usr) => usr,
        Err(_) => return HttpResponse::Unauthorized().finish().into(),
    };

    let collabo_id = path.into_inner();
    if collabo_id.is_empty() {
        return HttpResponse::BadRequest().body("Collabo id is required");
    }

    match db.collabo_repo.get_collabo_by_id(&collabo_id).await {
        Ok(collabos) => HttpResponse::Ok().json(collabos),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/{collabo_id}")]
pub async fn update_collabo(
    req: HttpRequest,
    db: Data<MongoRepo>,
    path: Path<String>,
    update_collabo_request: Json<UpdateCollabo>,
) -> HttpResponse {
    match protected_route(req).await {
        Ok(usr) => usr,
        Err(_) => return HttpResponse::Unauthorized().finish().into(),
    };

    let data = UpdateCollabo {
        name: update_collabo_request.name.to_owned(),
    };

    let collabo_id = path.into_inner();
    if collabo_id.is_empty() {
        return HttpResponse::BadRequest().body("Collabo id is required");
    }

    match db.collabo_repo.update_collabo(&collabo_id, data).await {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_collabo_info = db.collabo_repo.get_collabo_by_id(&collabo_id).await;
                return match updated_collabo_info {
                    Ok(collabo) => HttpResponse::Ok().json(collabo),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No Collabo found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
