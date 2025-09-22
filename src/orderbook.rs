use crate::input::Order;
use rust_decimal::Decimal;

use std::collections::{BTreeMap, VecDeque};

// pub struct Orderbook {
//     pub bids: HashMap<u32, Vec<Order>>,
//     pub asks: HashMap<u32, Vec<Order>>,
// }

pub struct Orderbook {
    ////@note why we created this struct?
    pub bids: BTreeMap<std::cmp::Reverse<Decimal>, VecDeque<Order>>,
    pub asks: BTreeMap<Decimal, VecDeque<Order>>,
}

impl Orderbook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }
}

// Removed an outdated `create_order` helper that did not match current types
