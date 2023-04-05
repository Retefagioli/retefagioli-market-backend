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
        let query = doc! {
            "user_id": "user",
            "state": "Open",
        };

        match service.find_one(query).await {
            Ok(Some(_)) => {
                HttpResponse::BadRequest().body("A shopping cart is already open for this user")
            }
            Ok(None) => {
                let shoppingcart = ShoppingCart::new("user");

                let mut location_cart = String::new();
                write!(location_cart, "/cart/{}", shoppingcart.get_id()).unwrap();

                match service.save_one(shoppingcart).await {
                    Ok(_) => HttpResponse::Created()
                        .append_header(("Location", location_cart))
                        .finish(),
                    Err(message) => HttpResponse::InternalServerError().body(message),
                }
            }
            Err(msg) => HttpResponse::InternalServerError().body(msg),
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
        let query_duplicated = doc! {
            "_id": ObjectId::parse_str(cart_id.as_ref()).unwrap(),
            "products.product_id": product.product_id(),
        };

        match service.find_one(query_duplicated).await {
            Ok(Some(_)) => HttpResponse::BadRequest().body("product already in cart"),
            Ok(None) => {
                let query = doc! {
                    "_id": ObjectId::parse_str(cart_id.as_ref()).unwrap(),
                };

                let update_info = doc! {
                    "$push": {
                        "products": {
                        "product_id": product.product_id(),
                        "quantity": product.quantity(),
                        "price": product.price(),
                        },
                    },
                };

                match service.update_one(query, update_info).await {
                    Ok(_) => HttpResponse::Ok().finish(),
                    Err(message) => HttpResponse::InternalServerError().body(message),
                }
            }
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

        match service.find_one_and_update(query, update_info).await {
            Ok(Some(_)) => HttpResponse::Ok().finish(),
            Ok(None) => HttpResponse::BadRequest().body("No product in the cart with that id"),
            Err(message) => HttpResponse::InternalServerError().body(message),
        }
    } else {
        HttpResponse::InternalServerError().body("Internal server error")
    }
}

#[delete("/{cart_id}/{product_id}")]
pub async fn delete_product_from_cart(
    path_params: web::Path<(String, String)>,
) -> impl Responder {
    if let Some(service) =
        CruderService::<ShoppingCart>::new(constant::SHOPPINGCART_COLLECTION).await
    {
        let (cart_id, product_id) = path_params.into_inner();
        let query = doc! {
            "_id": ObjectId::parse_str(&cart_id).unwrap(),
            "products.product_id": product_id.to_string(),
        };

        let delete_info = doc! {
            "$pull": {
                "products" : { "product_id": product_id.to_string(),}
            },
        };

        match service.find_one_and_update(query, delete_info).await {
            Ok(Some(_)) => HttpResponse::NoContent().finish(),
            Ok(None) => HttpResponse::BadRequest().body("No product with that id"),
            Err(message) => HttpResponse::InternalServerError().body(message),
        }
    } else {
        HttpResponse::InternalServerError().body("Internal server error")
    }
}
