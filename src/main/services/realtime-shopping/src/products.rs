mod api;
mod models;

use actix_web::{Scope, web};

pub fn get_products_apis() -> Scope {
    web::scope("/products")
        .service(api::get_product)
        .service(api::update_product)
        .service(api::add_product)
        .service(api::delete_product)
} 
