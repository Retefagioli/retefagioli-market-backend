use crate::database::{constant, Database};
use mongodb::{Collection, bson::Document, error::Error};
use serde::{Serialize, de::DeserializeOwned};
use futures::stream::TryStreamExt;

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

    pub async fn save_one(&self, element: T) -> Result<&str, ()> {
        match self.db.insert_one(element, None).await {
            Ok(_) => { 
                Ok("Should be fucking inserted")
            },
            Err(_) => { 
                 println!("Error inserting the fucking element");
                 Err(())
            }
        }
    }

    pub async fn find_one(&self, query: Document) -> Result<Option<T>, ()> {
        match self.db.find_one(query, None).await {
            Ok(Some(T)) => Ok(Some(T)),
            Ok(None) => Ok(None),
            Err(_) => Err(())
        }
    }

    pub async fn find(&self, query: Document) -> Result<Option<Vec<T>>, ()> {
        let cursor = self.db.find(query, None).await.unwrap();
        Ok(Some(cursor.try_collect().await.unwrap()))
    }

    pub async fn delete_one(&self, query: Document) -> Result<(), ()> {
        match self.db.delete_one(query, None).await {
            Ok(_) => Ok(()),
            Err(_) => Err(())

        }
    }

    pub async fn update_one(&self, query: Document, update_info: Document) -> Result<(), ()> {
        self.db.update_one(query, update_info, None).await;
        Ok(())
    }

    pub async fn update_many(&self, query: Document, update_info: Document) -> Result<(), ()> {
        self.db.update_many(query, update_info, None).await;
        Ok(())
    }
}
