use actix_web::{ HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthUserData {
    pub id: Option<ObjectId>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Claims {
    pub exp: i64,
    pub iat: i64,
    pub user: AuthUserData,
}

impl Claims {
    pub fn new(user: AuthUserData) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);

        Self {
            iat: iat.timestamp(),
            exp: exp.timestamp(),
            user: user,
        }
    }
}

pub fn create_jwt(user_data: AuthUserData) -> Result<String, jsonwebtoken::errors::Error> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(user_data),
        &EncodingKey::from_secret("some_secret".as_bytes()),
    )?)
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    Ok(jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret("some_secret".as_bytes()),
        &Validation::default(),
    )
        .map(|data| data.claims)?)
}

pub async fn protected_route(req: HttpRequest) -> Result<AuthUserData, HttpResponse> {
    let token = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap();

    match validate_jwt(token) {
        Ok(claims) => Ok(claims.user),
        Err(e) => {
            eprintln!("error response ===>>>>> {:?}", e);
            Err(HttpResponse::Unauthorized().finish())
        }
    }
}