use crate::models::collabo::{Collabo};
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error as MongoError,
    results::{InsertOneResult, UpdateResult}, Collection,
};


pub struct CollaboRepo {
    pub collabo: Collection<Collabo>,
}

pub struct UpdateCollaboAccountId {
    pub collabo_account_id: String,
}

impl CollaboRepo {
    pub async fn create_collabo(
        &self,
        new_collabo: Collabo,
    ) -> Result<InsertOneResult, MongoError> {
        let collabo = self.collabo.insert_one(new_collabo, None).await?;
        Ok(collabo)
    }

    pub async fn get_collabo_by_id (&self, id: &str) -> Result<Collabo, MongoError> {
        let obj_id = ObjectId::parse_str(id).expect("error parsing id");
        let filter = doc! {"_id": obj_id};

        let collabo = self
            .collabo
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");

        Ok(collabo.unwrap())
    }

    pub async fn update_collabo_account_id(
        &self,
        id: &str,
        new_collabo: UpdateCollaboAccountId,
    ) -> Result<UpdateResult, MongoError> {
        let obj_id = ObjectId::parse_str(id).expect("error parsing id");

        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set": {
                "collabo_account_id": new_collabo.collabo_account_id,
            },
        };
        let update_doc = self
            .collabo
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating collabo");
        Ok(update_doc)
    }

    pub async fn fetch_collabos(&self) -> Result<Vec<Collabo>, MongoError> {

    }
}
