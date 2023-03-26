use std::collections::HashSet;

type ProductId = String;
type Quantity = u64;

pub enum CartState {
    Open,
    InProgress,
    Closed,
}

pub struct ShoppingCart {
    cart_id: String, 
    user_id: String,
    product_list: HashSet<ProductId, Quantity>,
    state: CartState,
}
