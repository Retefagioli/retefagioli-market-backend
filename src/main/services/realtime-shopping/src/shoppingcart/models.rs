use std::collections::HashMap;
use serde::{Deserialize, Serialize};

type ProductId = String;
type Quantity = u64;

#[derive(Serialize, Deserialize)]
pub enum CartState {
    Open,
    InProgress,
    Closed,
}

#[derive(Serialize, Deserialize)]
pub struct ShoppingCart {
    cart_id: String, 
    user_id: String,
    product_list: HashMap<ProductId, Quantity>,
    state: CartState,
}

impl ShoppingCart {
    pub fn new(user_id: &str) -> Self {
        ShoppingCart { 
            cart_id: "JUIS".to_string(), 
            user_id: user_id.to_string(), 
            product_list: HashMap::new(), 
            state: CartState::Open 
        }
    }
}
