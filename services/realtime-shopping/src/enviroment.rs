pub mod keys {
    pub const MONGODB_CONFIG_USERNAME: &str = "MONGODB_CONFIG_USERNAME";
    pub const MONGODB_CONFIG_PASSWORD: &str = "MONGODB_CONFIG_PASSWORD";
    pub const MONGODB_CONFIG_CONNECTION: &str = "MONGODB_CONFIG_CONNECTION";
}

use std::env;

pub struct Env;

impl Env {
    pub fn get_mongodb_username() -> String {
        env::var(keys::MONGODB_CONFIG_USERNAME).unwrap()
    }

    pub fn get_mongodb_password() -> String {
        env::var(keys::MONGODB_CONFIG_PASSWORD).unwrap()
    }

    pub fn get_mongodb_connection_uri() -> String {
        env::var(keys::MONGODB_CONFIG_CONNECTION).unwrap()
    }
}
