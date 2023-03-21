use crate::database::{constant, Database};
use crate::products::models::Product;
use actix_web::{delete, get, post, put, web, Responder};

#[get("/{id}")]
pub async fn get_product(id: web::Path<String>) -> impl Responder {
    let something = Product::new();
    "ciao"
}

#[post("")]
pub async fn add_product(product: web::Json<Product>) -> impl Responder {
    let database = Database::get_client()
        .await
        .unwrap()
        .database(constant::DATABASE_NAME);
    let collection = database.collection::<Product>("products");
    collection.insert_one(product.clone(), None).await;
    "CIAO"
}

#[delete("/{id}")]
pub async fn delete_product() -> impl Responder {
    "ciao"
}

#[put("")]
pub async fn update_product() -> impl Responder {
    "ciao"
}
