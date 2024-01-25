use crate::models::user::User;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error as MongoError,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};

pub struct UserRepo {
    pub user: Collection<User>,
}

impl UserRepo {
    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, MongoError> {
        let user = self.user.insert_one(new_user, None).await?;
        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<User, MongoError> {
        let filter = doc! {"email": email};
        let user_detail = self
            .user
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub async fn get_user(&self, id: &String) -> Result<User, MongoError> {
        let obj_id = ObjectId::parse_str(id).expect("error parsing id");
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .user
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub async fn update_user(
        &self,
        id: &String,
        new_user: User,
    ) -> Result<UpdateResult, MongoError> {
        let obj_id = ObjectId::parse_str(id).expect("error parsing id");

        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set": {
                "id": new_user.id,
                "first_name": new_user.first_name,
                "last_name": new_user.last_name,
                "location": new_user.location,
                "title": new_user.title
            },
        };
        let update_doc = self
            .user
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(update_doc)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, MongoError> {
        let obj_id = ObjectId::parse_str(id).expect("error parsing id");
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .user
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }
}
