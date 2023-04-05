use crate::database::constant;
use crate::products::models::Product;
use crate::service::CruderService;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use mongodb::bson::doc;

#[get("/{barcode}")]
pub async fn get_product(barcode: web::Path<String>) -> impl Responder {
    if let Some(service) = CruderService::<Product>::new(constant::PRODUCTS_COLLECTION).await {
        let filter = doc! {
            "barcode": barcode.to_string(),
        };
        match service.find_one(filter).await {
            Ok(Some(product)) => HttpResponse::Found().json(product),
            Ok(None) => HttpResponse::NotFound().finish(),
            Err(message) => HttpResponse::InternalServerError().body(message),
        }
    } else {
        HttpResponse::InternalServerError().body("internal server error")
    }
}

#[post("")]
pub async fn add_product(product: web::Json<Product>) -> impl Responder {
    if let Some(service) = CruderService::<Product>::new(constant::PRODUCTS_COLLECTION).await {
        match service.save_one(product.clone()).await {
            Ok(()) => HttpResponse::Created().finish(),
            Err(message) => HttpResponse::InternalServerError().body(message),
        }
    } else {
        HttpResponse::InternalServerError().body("internal server error")
    }
}

#[delete("/{barcode}")]
pub async fn delete_product(barcode: web::Path<String>) -> impl Responder {
    if let Some(service) = CruderService::<Product>::new(constant::PRODUCTS_COLLECTION).await {
        let filter = doc! {
            "barcode": barcode.to_string(),
        };
        match service.find_one(filter.clone()).await {
            Ok(Some(_)) => match service.delete_one(filter).await {
                Ok(()) => HttpResponse::NoContent().finish(),
                Err(message) => HttpResponse::InternalServerError().body(message),
            },
            Ok(None) => HttpResponse::NotFound().finish(),
            Err(message) => HttpResponse::InternalServerError().body(message),
        }
    } else {
        HttpResponse::InternalServerError().body("internal server error")
    }
}

#[put("/{barcode}")]
pub async fn update_product(
    product: web::Json<Product>,
    barcode: web::Path<String>,
) -> impl Responder {
    if let Some(service) = CruderService::<Product>::new(constant::PRODUCTS_COLLECTION).await {
        let query = doc! {
            "barcode": barcode.to_string(),
        };
        match service.find_one(query.clone()).await {
            Ok(Some(_)) => match service.update_one(query, product.get_doc()).await {
                Ok(()) => HttpResponse::Ok().finish(),
                Err(message) => HttpResponse::InternalServerError().body(message),
            },
            Ok(None) => HttpResponse::NotFound().finish(),
            Err(message) => HttpResponse::InternalServerError().body(message),
        }
    } else {
        HttpResponse::InternalServerError().body("internal server error")
    }
}
