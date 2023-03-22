use actix_web::Error;
use crate::products::models::Product;
use crate::service::CrudService;
use crate::database::{Database, constant};
use async_trait::async_trait;


pub struct ProductService;

#[async_trait]
impl CrudService<String, Product> for ProductService{
    async fn save(element: Product) -> Result<(), Error> {
        let client = Database::get_client().await.unwrap();
        let database = client.database(constant::PRODUCTS_COLLECTION);
        let collection = database.collection::<Product>("products"); 
        collection.insert_one(element, None).await.unwrap();
        Ok(())
    }
}
