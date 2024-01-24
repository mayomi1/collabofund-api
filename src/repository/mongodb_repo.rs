use std::env;

extern crate dotenv;
use dotenv::dotenv;

use crate::models::{collabo::Collabo, user::User};
use crate::repository::collabo::CollaboRepo;
use crate::repository::user::UserRepo;
use crate::repository::collabo_account::CollaboAccountRepo;

use mongodb::{
    Client, Collection,
};
use crate::models::collabo::CollaboAccount;

pub struct MongoRepo {
    pub user_repo: UserRepo,
    pub collabo_repo: CollaboRepo,
    pub collabo_account_repo: CollaboAccountRepo,
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
        let collabo_account: Collection<CollaboAccount> = db.collection("CollaboAccounts");

        let user_repo = UserRepo { user };
        let collabo_repo = CollaboRepo { collabo };
        let collabo_account_repo = CollaboAccountRepo { collabo_account };

        MongoRepo {
            user_repo,
            collabo_repo,
            collabo_account_repo
        }
    }
}
