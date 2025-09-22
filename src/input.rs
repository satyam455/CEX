use actix_web::web::Json;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]

pub struct CreateOrderInput {
    pub price: u32,
    pub quantity: u32,
    pub user_id: u32,
    pub side: Side,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub order_id: Uuid,
    pub user_id: String,
    pub side: Side,
    pub price: Decimal,
    pub quantity: Decimal,
    pub filled_quantity: Decimal,
    pub timestamp: i64,
}

#[derive(Debug, Clone)]
pub struct Fill {
    pub trade_id: Uuid,
    pub price: Decimal,
    pub quantity: Decimal,
    pub maker_order_id: Uuid,
    pub taker_order_id: Uuid,
    pub timestamp: i64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Deserialize, Debug)]

pub struct DeleteOrderInput {
    pub order_id: String,
}

#[derive(Deserialize, Serialize)]

pub struct Depth {
    pub bid: Vec<[u32; 2]>,
    pub ask: Vec<[u32; 2]>,
    pub lastUpdatedId: String,
}
