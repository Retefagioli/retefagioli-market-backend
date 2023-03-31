use crate::database::constant;
use crate::service::CruderService;
use crate::shoppingcart::models::{ProductCart, ShoppingCart};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use mongodb::bson::{doc, oid::ObjectId};
use std::fmt::Write;

#[get("/{cart_id}")]
pub async fn get_shoppingcart(cart_id: web::Path<String>) -> impl Responder {
    if let Some(service) =
        CruderService::<ShoppingCart>::new(constant::SHOPPINGCART_COLLECTION).await
    {
        let filter = doc! {
            "_id": ObjectId::parse_str(cart_id.as_ref()).unwrap(),
        };
        match service.find_one(filter).await {
            Ok(Some(element)) => HttpResponse::Found().json(element),
            Ok(None) => HttpResponse::NotFound().finish(),
            Err(message) => HttpResponse::InternalServerError().body(message),
        }
    } else {
        HttpResponse::InternalServerError().body("Internal server error")
    }
}

#[post("")]
pub async fn init_shoppingcart() -> impl Responder {
    if let Some(service) =
        CruderService::<ShoppingCart>::new(constant::SHOPPINGCART_COLLECTION).await
    {
        let shoppingcart = ShoppingCart::new("user");

        let mut location_cart = String::new(); 
        write!(location_cart, "/cart/{}", shoppingcart.get_id()).unwrap();

        match service.save_one(shoppingcart).await {
            Ok(_) => HttpResponse::Created()
                .append_header(("Location",location_cart))
                .finish(),
            Err(message) => HttpResponse::InternalServerError().body(message),
        }
    } else {
        HttpResponse::InternalServerError().body("Internal server error")
    }
}

#[post("/{cart_id}")]
pub async fn add_product_to_cart(
    cart_id: web::Path<String>,
    product: web::Json<ProductCart>,
) -> impl Responder {
    if let Some(service) =
        CruderService::<ShoppingCart>::new(constant::SHOPPINGCART_COLLECTION).await
    {
        let query = doc! {
            "_id": ObjectId::parse_str(cart_id.as_ref()).unwrap(),
        };

        let update_info = doc! {
            "$push": {
                "products": {
                "product_id": product.product_id(),
                "quantity": product.quantity(),
                "price": product.price(),
            }
            }
        };

        match service.update_one(query, update_info).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(message) => HttpResponse::InternalServerError().body(message),
        }
    } else {
        HttpResponse::InternalServerError().body("Internal server error")
    }
}

#[put("/{cart_id}")]
pub async fn update_product_in_cart(
    cart_id: web::Path<String>,
    product: web::Json<ProductCart>,
) -> impl Responder {
    if let Some(service) =
        CruderService::<ShoppingCart>::new(constant::SHOPPINGCART_COLLECTION).await
    {
        let query = doc! {
            "_id": ObjectId::parse_str(cart_id.as_ref()).unwrap(),
            "products.product_id": product.product_id(),
        };

        let update_info = doc! {
            "$set": {
                "products.$.quantity": product.quantity(),
                "products.$.price": product.price(),
            }
        };

        match service.update_one(query, update_info).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(message) => HttpResponse::InternalServerError().body(message),
        }
    } else {
        HttpResponse::InternalServerError().body("Internal server error")
    }
}

#[delete("/{cart_id}/{product_id}")]
pub async fn delete_product_from_cart(
    cart_id: web::Path<String>,
    product_id: web::Path<String>,
) -> impl Responder {
    if let Some(service) =
        CruderService::<ShoppingCart>::new(constant::SHOPPINGCART_COLLECTION).await
    {
        let query = doc! {
            "_id": ObjectId::parse_str(cart_id.as_ref()).unwrap(),
        };

        let delete_info = doc! {
            "$pull": { "products.product_id": product_id.to_string()},
        };

        match service.update_one(query, delete_info).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(message) => HttpResponse::InternalServerError().body(message),
        }
    } else {
        HttpResponse::InternalServerError().body("Internal server error")
    }
}
