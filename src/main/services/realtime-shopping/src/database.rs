pub mod constant {
    pub const DATABASE_NAME: &str = "realtime-shopping";
    pub const PRODUCTS_COLLECTION: &str = "products";
}

use crate::enviroment::Env;
use mongodb::options::{ClientOptions, Credential};
use mongodb::Client;

pub struct Database;

impl Database {
    pub async fn get_client() -> Option<Client> {
        let mut client_options = ClientOptions::parse(Env::get_mongodb_connection_uri())
            .await
            .unwrap();
        let credentials = Credential::builder()
            .username(Env::get_mongodb_username())
            .password(Env::get_mongodb_password())
            .build();
        client_options.credential = Some(credentials);
        Some(Client::with_options(client_options).unwrap())
    }
}
