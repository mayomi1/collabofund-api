mod api;
mod models;
mod providers;
mod repository;
mod utils;

mod clients;

use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, web::ServiceConfig};
use api::{
    collabo::{create_collabo, fetch_collabos, generate_account, get_collabo, update_collabo},
    user::{delete_user, get_user, login_user, register_user, update_user},
};
use shuttle_actix_web::ShuttleActixWeb;

use repository::mongodb_repo::MongoRepo;

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/user")
                .wrap(Logger::default())
                .service(register_user)
                .service(login_user)
                .service(get_user)
                .service(update_user)
                .service(delete_user)
                .app_data(db_data.clone()),
        )
        .service(
            web::scope("/collabo")
                .wrap(Logger::default())
                .service(create_collabo)
                .service(generate_account)
                .service(fetch_collabos)
                .service(get_collabo)
                .service(update_collabo)
                .app_data(db_data.clone()),
        );
    };

    Ok(config.into())
}
