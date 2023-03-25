use crate::products::models::Product;
use crate::service::CruderService;
use crate::{database::constant, service};
use actix_web::{delete, get, post, put, web, Responder, HttpResponse};
use mongodb::bson::doc;

#[get("/{name}")]
pub async fn get_product(name: web::Path<String>) -> impl Responder {
    if let Some(service) = CruderService::<Product>::new(constant::PRODUCTS_COLLECTION).await {
        let filter = doc! {
            "name": name.to_string(),
        };
        match service.find_one(filter).await {
            Ok(Some(product)) => web::Json(product),
            Ok(None) => web::Json(Product::new()),
            Err(_) => web::Json(Product::new()),
        }
    } else {
        web::Json(Product::new())
    }
}

#[post("")]
pub async fn add_product(product: web::Json<Product>) -> impl Responder {
    if let Some(service) = CruderService::<Product>::new(constant::PRODUCTS_COLLECTION).await {
        match service.save_one(product.clone()).await {
            Ok(result) => result.to_owned(),
            Err(_) => "Non worka".to_string(),
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
