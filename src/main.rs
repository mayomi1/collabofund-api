mod api;
mod models;
mod repository;

use actix_web::{web, web::ServiceConfig};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use shuttle_actix_web::ShuttleActixWeb;
use api::user::{create_user, get_user, update_user, delete_user};
use repository::mongodb_repo::MongoRepo;

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(web::scope("/user")
                        .wrap(Logger::default())
                        .service(create_user)
                        .service(get_user)
                        .service(update_user)
                        .service(delete_user)
                        .app_data(db_data),
        );
    };

    Ok(config.into())
}