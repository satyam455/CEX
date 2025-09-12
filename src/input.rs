use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]

pub struct CreateOrderInput {
    pub price: u32,
    pub quantity: u32,
    pub user_id: String,
    pub side: Side,
}

#[derive(Deserialize, Debug)]

pub enum Side {
    Buy,
    Sell,
}

#[derive(Deserialize, Debug)]|

pub struct DeleteOrderInput {
    pub order_id: String,
}

#[derive(Deserialize, Serialize)]

pub struct Depth {
    pub bid: Vec<[u32; 2]>,
    pub ask: Vec<[u32; 2]>,
    pub lastUpdatedId: String
}

