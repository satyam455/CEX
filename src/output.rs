use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]

pub struct CreateOrderResponse {
    pub order_id: String,
}

#[derive(Deserialize, Serialize, Debug)]

pub struct DeleteOrderResponse {
    pub price: u32,
    pub quantity: u32,
}
