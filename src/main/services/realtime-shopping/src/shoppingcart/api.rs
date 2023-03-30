use crate::shoppingcart::models::ShoppingCart;
use crate::service::CruderService;
use crate::database::constant;
use actix_web::{get, post, delete, put, web, Responder, HttpResponse};
use mongodb::bson::doc;


#[get("/{cart_id}")]
pub async fn get_shoppingcart(cart_id: web::Path<String>) -> impl Responder {
   if let Some(service) = CruderService::<ShoppingCart>::new(constant::SHOPPINGCART_COLLECTION).await {
        let filter = doc! {
            "cart_id": cart_id.to_string(),
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
   if let Some(service) = CruderService::<ShoppingCart>::new(constant::SHOPPINGCART_COLLECTION).await {

        let shoppingcart = ShoppingCart::new("user"); 
        
        match service.save_one(shoppingcart).await {
            Ok(_) => HttpResponse::Created().append_header(("Location", "/cart/{cart_id}")).finish(),
            Err(message) => HttpResponse::InternalServerError().body(message),
        }
   } else {
        HttpResponse::InternalServerError().body("Internal server error")
   }
}
