use serde::{Deserialize, Serialize};

pub type Price = usize;
pub type Quantity = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitOrderSubmitRequest {
    side: Side,
    price: Price,
    quantity: Quantity,
}