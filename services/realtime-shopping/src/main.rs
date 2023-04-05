pub mod database;
pub mod enviroment;
pub mod products;
pub mod service;
pub mod shoppingcart;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(products::get_products_apis())
            .service(shoppingcart::get_shoppingcart_api())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
