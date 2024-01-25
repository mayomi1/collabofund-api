use crate::models::collabo::Collabo;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error as MongoError,
    results::{InsertOneResult, UpdateResult},
    Collection,
};
use serde::Deserialize;

use tokio_stream::StreamExt;

pub struct CollaboRepo {
    pub collabo: Collection<Collabo>,
}

pub struct UpdateCollaboAccountId {
    pub collabo_account_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCollabo {
    pub name: String,
    // TODO: add start_date and end_date
}

impl CollaboRepo {
    pub async fn create_collabo(
        &self,
        new_collabo: Collabo,
    ) -> Result<InsertOneResult, MongoError> {
        let collabo = self.collabo.insert_one(new_collabo, None).await?;
        Ok(collabo)
    }

    pub async fn get_collabo_by_id(&self, id: &str) -> Result<Collabo, MongoError> {
        let obj_id = ObjectId::parse_str(id).expect("error parsing id");
        let filter = doc! {"_id": obj_id};

        println!("filters {:?} ", filter);

        let collabo = self.collabo.find_one(filter, None).await?;

        println!("collabo {:?} ", collabo);

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

    pub async fn fetch_collabos(&self, user_id: String) -> Result<Vec<Collabo>, MongoError> {
        let filter = doc! {"user_id": user_id};
        let mut cursors = self.collabo.find(filter, None).await?;
        let mut collabos: Vec<Collabo> = Vec::new();

        while let Some(item) = cursors.try_next().await? {
            collabos.push(item);
        }

        Ok(collabos)
    }

    pub async fn update_collabo(
        &self,
        id: &str,
        new_collabo: UpdateCollabo,
    ) -> Result<UpdateResult, MongoError> {
        let obj_id = ObjectId::parse_str(id).expect("error parsing id");

        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set": {
                "name": new_collabo.name,
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
}
