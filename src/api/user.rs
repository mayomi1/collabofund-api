use bcrypt::{hash, DEFAULT_COST};

use crate::{lib, models::user::User, repository::mongodb_repo::MongoRepo};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

use lib::auth::{create_jwt, AuthUserData};

#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login")]
pub async fn login_user(db: Data<MongoRepo>, login_request: Json<LoginRequest>) -> HttpResponse {
    let user_detail = db
        .user_repo
        .get_user_by_email(login_request.email.to_string())
        .await;

    match user_detail {
        Ok(user) => match bcrypt::verify(&login_request.password, &user.password) {
            Ok(matching) => {
                if matching {
                    let user_data = AuthUserData {
                        email: user.email,
                        title: user.title,
                        id: user.id,
                    };

                    match create_jwt(user_data) {
                        Ok(token) => HttpResponse::Ok().json(token),
                        Err(_) => HttpResponse::InternalServerError().finish(),
                    }
                } else {
                    HttpResponse::Unauthorized().json("Invalid password")
                }
            }
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(_) => HttpResponse::Unauthorized().json("Invalid username or password"),
    }
}

#[post("/register")]
pub async fn register_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let hashed_password = match hash(&new_user.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let data = User {
        id: None,
        email: new_user.email.to_owned(),
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
        password: hashed_password,
    };
    let user_detail = db.user_repo.create_user(data).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadGateway().body("invalid ID");
    }

    let user_detail = db.user_repo.get_user(&id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/{id}")]
pub async fn update_user(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_user: Json<User>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid Id");
    };
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        title: new_user.title.to_owned(),
        location: new_user.location.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
    };
    let update_result = db.user_repo.update_user(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.user_repo.get_user(&id).await;
                return match updated_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No user found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/{id}")]
pub async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse {
            message: "Invalid ID".to_string(),
        });
    };
    let result = db.user_repo.delete_user(&id).await;

    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json(ApiResponse {
                    message: "User successfully deleted".to_string(),
                });
            } else {
                return HttpResponse::NotFound().json(ApiResponse {
                    message: "User not found".to_string(),
                });
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
