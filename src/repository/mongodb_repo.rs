use std::env;

extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, oid::ObjectId,},
    results::{ InsertOneResult, DeleteResult, UpdateResult },
    Client, Collection, error::Error as MongoError
};

use crate::models::user::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = env::var("MONGOURI").expect("Error loading env variable MONGOURI");
        let client = Client::with_uri_str(&uri).await.expect("Error connecting to MongoDB");
        let db = client.database("collabofund-db");
        let col :Collection<User> = db.collection("Users");
        MongoRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, MongoError> {
        let user = self.col.insert_one(new_user, None).await?;
        Ok(user)
    }

    pub async fn get_user(&self, id: &String) -> Result<User, MongoError> {
        let obj_id = ObjectId::parse_str(id).expect("error parsing id");
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub async fn update_user (&self, id: &String, new_user: User) -> Result<UpdateResult, MongoError> {
        let obj_id = ObjectId::parse_str(id).expect("error parsing id");


        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set": {
                "id": new_user.id,
                "name": new_user.name,
                "location": new_user.location,
                "title": new_user.title
            },
        };
        let update_doc = self.col.update_one(filter, new_doc, None).await.ok().expect("Error updating user");
        Ok(update_doc)
    }

    pub async fn delete_user (&self, id: &String) -> Result<DeleteResult, MongoError> {
        let obj_id = ObjectId::parse_str(id).expect("error parsing id");
        let filter = doc! {"_id": obj_id};
        let user_detail = self.col.delete_one(filter, None).await.ok().expect("Error deleting user");
        Ok(user_detail)
    }
}


