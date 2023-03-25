pub mod database;
pub mod enviroment;
pub mod products;
pub mod service;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(products::get_products_apis()))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
