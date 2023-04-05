mod api;
pub mod models;

use actix_web::{web, Scope};

pub fn get_products_apis() -> Scope {
    web::scope("/products")
        .service(api::get_product)
        .service(api::update_product)
        .service(api::add_product)
        .service(api::delete_product)
}
