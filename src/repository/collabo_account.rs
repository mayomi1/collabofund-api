use crate::models::collabo::{Collabo, CollaboAccount};
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error as MongoError,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

pub struct CollaboAccountRepo {
    pub collabo_account: Collection<CollaboAccount>}

impl CollaboAccountRepo {
    pub async fn create_collabo_account(
        &self,
        new_account: CollaboAccount,
    ) -> Result<InsertOneResult, MongoError> {
        let account = self.collabo_account.insert_one(new_account, None).await?;
        Ok(account)
    }

    pub async fn get_collabo_account_by_id (&self, id: &str) -> Result<CollaboAccount, MongoError> {
        let obj_id = ObjectId::parse_str(id).expect("error parsing id");
        let filter = doc! {"_id": obj_id};

        let collabo_account = self
            .collabo_account
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting collabo account detail");

        Ok(collabo_account.unwrap())
    }

}
