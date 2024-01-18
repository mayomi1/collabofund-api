use actix_web::{HttpRequest, HttpResponse};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUserData {
    pub id: Option<ObjectId>,
    pub email: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Option<ObjectId>,
    exp: usize,
    user: AuthUserData,
}

pub fn create_jwt(user_data: AuthUserData) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_data.id.to_owned(),
        exp: expiration as usize,
        user: user_data,
    };

    let header = Header::default();
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret("my_secret".as_ref()),
    )
}

fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("my_secret".as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

async fn protected_route(req: HttpRequest) -> HttpResponse {
    let token = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap();

    match validate_jwt(token) {
        Ok(claims) => HttpResponse::Ok().json(claims),
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}
