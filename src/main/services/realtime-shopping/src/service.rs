use crate::database::{constant, Database};
use futures::stream::TryStreamExt;
use mongodb::{bson::Document, Collection};
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;

pub struct CruderService<T> {
    db: Collection<T>,
}

impl<T> CruderService<T>
where
    T: Serialize + Sized + DeserializeOwned + Unpin + Sync + Send,
{
    pub async fn new(collection_name: &str) -> Option<CruderService<T>> {
        if let Some(client) = Database::get_client().await {
            let database = client.database(constant::DATABASE_NAME);
            let collection = database.collection::<T>(collection_name);
            Some(CruderService::<T> { db: collection })
        } else {
            None
        }
    }

    pub async fn save_one(&self, element: T) -> Result<(), String> {
        match self.db.insert_one(element, None).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.source().unwrap().to_string()),
        }
    }

    pub async fn find_one(&self, query: Document) -> Result<Option<T>, String> {
        match self.db.find_one(query, None).await {
            Ok(Some(element)) => Ok(Some(element)),
            Ok(None) => Ok(None),
            Err(e) => Err(e.source().unwrap().to_string()),
        }
    }

    pub async fn find_many(&self, query: Document) -> Result<Option<Vec<T>>, String> {
        let cursor = self.db.find(query, None).await.unwrap();
        Ok(Some(cursor.try_collect().await.unwrap()))
    }

    pub async fn find_one_and_update(&self, query: Document, update_info: Document) -> Result<Option<T>, String> {
        match self.db.find_one_and_update(query, update_info, None).await  {
            Ok(Some(element)) => Ok(Some(element)),
            Ok(None) => Ok(None),
            Err(e) => Err(e.source().unwrap().to_string())
        }
    }

    pub async fn find_one_and_delete(&self, query: Document ) -> Result<Option<T>, String> {
        match self.db.find_one_and_delete(query, None).await {
            Ok(Some(element)) => Ok(Some(element)),
            Ok(None) => Ok(None),
            Err(e) => Err(e.source().unwrap().to_string())
        }
    }

    pub async fn delete_one(&self, query: Document) -> Result<(), String> {
        match self.db.delete_one(query, None).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.source().unwrap().to_string()),
        }
    }

    pub async fn update_one(&self, query: Document, update_info: Document) -> Result<(), String> {
        match self.db.update_one(query, update_info, None).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.source().unwrap().to_string()),
        }
    }

    pub async fn update_many(&self, query: Document, update_info: Document) -> Result<(), String> {
        match self.db.update_many(query, update_info, None).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.source().unwrap().to_string()),
        }
    }

}
