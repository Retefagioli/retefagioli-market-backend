use crate::database::{constant, Database};
use crate::products::models::Product;
use crate::products::service::ProductService;
use crate::service::CrudService;
use actix_web::{delete, get, post, put, web, Responder};


#[get("/{id}")]
pub async fn get_product(id: web::Path<String>) -> impl Responder {
    let something = Product::new();
    "ciao"
}

#[post("")]
pub async fn add_product(product: web::Json<Product>) -> impl Responder {
    ProductService::save(product.clone()).await.unwrap(); 
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
