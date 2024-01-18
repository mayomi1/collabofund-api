use crate::models::collabo::Collabo;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error as MongoError,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

pub struct CollaboRepo {
    pub collabo: Collection<Collabo>,
}

impl CollaboRepo {
    pub async fn create_collabo(
        &self,
        new_collabo: Collabo,
    ) -> Result<InsertOneResult, MongoError> {
        let collabo = self.collabo.insert_one(new_collabo, None).await?;
        Ok(collabo)
    }
}
