use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use chrono;

#[derive(Serialize, Deserialize)]
pub enum CartState {
    Open,
    InProgress,
    Closed,
}

#[derive(Serialize, Deserialize)]
pub struct ShoppingCart {
    #[serde(rename = "_id")]
    cart_id: ObjectId,
    user_id: String,
    products: Vec<ProductCart>,
    state: CartState,
    timestamp: i64,
}

impl ShoppingCart {
    pub fn new(user_id: &str) -> Self {
        ShoppingCart {
            cart_id: ObjectId::new(),
            user_id: user_id.to_string(),
            products: Vec::new(),
            state: CartState::Open,
            timestamp: chrono::offset::Utc::now().timestamp(),
         }
    }

    pub fn get_id(&self) -> String {
        self.cart_id.to_hex()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProductCart {
    product_id: String,
    price: f64,
    quantity: u32,
}

impl ProductCart {
    pub fn product_id(&self) -> &str {
        &self.product_id
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }
}
