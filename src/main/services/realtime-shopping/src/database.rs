pub mod constant {
    pub const DATABASE_NAME: &str = "realtime-shopping";
    pub const MONGODB_CONNECTION: &str = "mongodb://127.0.0.1:27017/";
}

use mongodb::{Client};

pub struct Database;

impl Database {
    pub async fn get_client() -> Option<Client> {
        Some(Client::with_uri_str(constant::MONGODB_CONNECTION).await.ok()?)
    }
}
