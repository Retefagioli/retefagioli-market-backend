use crate::database::constant;
use crate::products::models::Product;
use crate::service::CruderService;
use actix_web::{delete, get, post, put, web, Responder};

#[get("/{name}")]
pub async fn get_product(name: web::Path<String>) -> impl Responder {
    let something = Product::new();
    "ciao"
}

#[post("")]
pub async fn add_product(product: web::Json<Product>) -> impl Responder {
    if let Some(service) = CruderService::<Product>::new(constant::PRODUCTS_COLLECTION).await {
        match service.save(product.clone()).await {
            Ok(result) => result.to_owned(),
            Err(_) => "Non worka".to_string()
        }
    } else {
        "Doesn't work".to_string()
    }
}

#[delete("/{id}")]
pub async fn delete_product() -> impl Responder {
    "ciao"
}

#[put("")]
pub async fn update_product() -> impl Responder {
    "ciao"
}
