mod api;
pub mod models;

use actix_web::{web, Scope};

pub fn get_shoppingcart_api() -> Scope {
    web::scope("/cart")
        .service(api::get_shoppingcart)
        .service(api::init_shoppingcart)
        .service(api::add_product_to_cart)
        .service(api::delete_product_from_cart)
        .service(api::update_product_in_cart)
}
