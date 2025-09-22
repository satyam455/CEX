use crate::input::Side;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]

pub struct CreateOrderResponse {
    pub status: String,
    pub order_id: String,
}

#[derive(Deserialize, Serialize, Debug)]

pub struct DeleteOrderResponse {
    pub price: u32,
    pub quantity: u32,
}

// // --- API Request & Response Types ---

#[derive(Deserialize, Debug)]
pub struct CreateOrderRequest {
    pub user_id: String,
    pub side: Side,
    pub price: String, // Accept strings to avoid float precision issues from JSON
    pub quantity: String,
}

#[derive(Serialize, Debug)]
pub struct OrderResponse {
    pub order_id: String,
    pub user_id: String,
    pub side: Side,
    pub price: String,
    pub quantity: String,
    pub filled_quantity: String,
    pub timestamp: i64,
}

#[derive(Serialize, Debug)]
pub struct DepthResponse {
    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>,
}
