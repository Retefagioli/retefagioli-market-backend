use actix_web::{delete, get, post, put, web, Responder};
use crate::products::models::Products;

#[get("/{id}")]
pub async fn get_product(id: web::Path<String>) -> impl Responder {
    let something = Products::new();
    "ciao"
}

#[post("/")]
pub async fn add_product() -> impl Responder {
    "ciao"
}

#[delete("/{id}")]
pub async fn delete_product() -> impl Responder {
    "ciao"
}

#[put("/")]
pub async fn update_product() -> impl Responder {
    "ciao"
}
