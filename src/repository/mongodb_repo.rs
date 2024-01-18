use std::env;

extern crate dotenv;
use dotenv::dotenv;

use crate::models::{collabo::Collabo, user::User};
use crate::repository::collabo::CollaboRepo;
use crate::repository::user::UserRepo;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error as MongoError,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

pub struct MongoRepo {
    pub user_repo: UserRepo,
    pub collabo_repo: CollaboRepo,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = env::var("MONGOURI").expect("Error loading env variable MONGOURI");
        let client = Client::with_uri_str(&uri)
            .await
            .expect("Error connecting to MongoDB");
        let db = client.database("collabofund-db");

        let user: Collection<User> = db.collection("Users");
        let collabo: Collection<Collabo> = db.collection("Collabos");

        let user_repo = UserRepo { user };
        let collabo_repo = CollaboRepo { collabo };

        MongoRepo {
            user_repo,
            collabo_repo,
        }
    }
}
