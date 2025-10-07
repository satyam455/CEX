use crate::balance::BalanceManager;
use crate::input::{Fill, Order, Side};
use crate::market::MarketManager;
use crate::orderbook::Orderbook;
use crate::token::{TokenRegistry, TradinPair};
use actix::{Actor, Context, Handler, Message};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Message, Debug)]
#[rtype(result = "Result<Uuid, String>")]

pub struct CreateMarketOrder {
    pub user_id: String,
    pub market: String,
    pub side: Side,
    pub price: Decimal,
    pub quantity: Decimal,
}

#[derive(Message)]
#[rtype(result = "Result<crate::output::DepthResponse, String>")]
pub struct GetMarketDepth {
    pub market_pair: String,
}

#[derive(Message)]
#[rtype(result = "Result<Order, String>")]
pub struct GetOrder {
    pub order_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "Result<Uuid, String>")]
pub struct CancelOrder {
    pub order_id: Uuid,
    pub user_id: String,
}

#[derive(Message)]
#[rtype(result = "Result<crate::output::DepthResponse, String>")]
pub struct GetDepth;

///// implement more order message like cancel order, cancel all orders, get open order, get open orders, get depth, cancel all orders

pub struct MatchingEngine {
    pub token_registry: TokenRegistry,
    pub market_manager: MarketManager,
    pub balance_manager: BalanceManager,
    pub orderbook: Orderbook,
    orders: std::collections::HashMap<Uuid, Order>,
}

impl MatchingEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            token_registry: TokenRegistry::new(),
            market_manager: MarketManager::new(),
            balance_manager: BalanceManager::new(),
            orderbook: Orderbook::new(),
            orders: std::collections::HashMap::new(),
        };

        // Initialize market maker with liquidity
        engine
            .balance_manager
            .initialize_market_maker("market_maker_1");
        engine.provide_initial_liquidity();
        engine
    }

    fn provide_initial_liquidity(&mut self) {
        let liquidity_provisions = vec![
            ("TAN_KAN", Decimal::new(10_000, 0), Decimal::new(50_000, 0)), // 10k TAN, 50
            ("ADI_TAN", Decimal::new(5_000, 0), Decimal::new(10_000, 0)),  // 5k ADI, 10k
            ("PRA_KAN", Decimal::new(3_000, 0), Decimal::new(9_000, 0)),   // 3k PRA, 9k
            ("RAC_SAT", Decimal::new(2_000, 0), Decimal::new(8_000, 0)),   // 2k RAC, 8k
        ];
        for (market, base_amount, quote_amount) in liquidity_provisions {
            if let Some(market_ref) = self.market_manager.get_market_mut(market) {
                market_ref.add_liquidity(base_amount, quote_amount);
            }
        }
        println!("Provided initial liquidity to all markets");
    }
}

impl Actor for MatchingEngine {
    type Context = Context<Self>;
}

// actor handling the create order message
impl Handler<CreateMarketOrder> for MatchingEngine {
    type Result = Result<Uuid, String>;

    fn handle(&mut self, msg: CreateMarketOrder, _ctx: &mut Self::Context) -> Self::Result {
        let order_id = Uuid::new_v4();
        let mut taker_order = Order {
            order_id,
            user_id: msg.user_id,
            side: msg.side.clone(),
            price: msg.price,
            quantity: msg.quantity,
            filled_quantity: Decimal::ZERO,
            timestamp: chrono::Utc::now().timestamp_millis(),
        };

        println!("Engine processing order: {:?}", taker_order.order_id);
        let fills = self.match_order(&mut taker_order);

        if !fills.is_empty() {
            println!("Matched {} fills.", fills.len());
            // TODO: Persist fills to DB and publish to Redis
        }

        if taker_order.quantity > taker_order.filled_quantity {
            self.add_order_to_book(taker_order.clone());
        }

        self.orders.insert(order_id, taker_order);
        Ok(order_id)
    }
}

impl Handler<CreateMarketOrder> for MatchingEngine {
    type Result = Result<Uuid, String>;

    fn handle(&mut self, msg: CreateMarketOrder, _ctx: &mut Self::Context) -> Self::Result {
        // Validate market exists
        let market = self
            .market_manager
            .get_market_mut(&msg.market_pair)
            .ok_or_else(|| format!("Market {} not found", msg.market_pair))?;

        if !market.is_active {
            return Err(format!("Market {} is not active", msg.market_pair));
        }

        let order_id = Uuid::new_v4();
        println!(
            "Processing order {} for market {}",
            order_id, msg.market_pair
        );

        // Here you would implement the full order matching logic
        // For now, just acknowledge the order

        Ok(order_id)
    }
}

impl Handler<GetOrder> for MatchingEngine {
    type Result = Result<Order, std::string::String>;
    fn handle(&mut self, msg: GetOrder, _ctx: &mut Self::Context) -> Self::Result {
        self.orders
            .get(&msg.order_id)
            .cloned()
            .ok_or_else(|| "Order not found".to_string())
    }
}

impl Handler<CancelOrder> for MatchingEngine {
    type Result = Result<Uuid, String>;
    fn handle(&mut self, msg: CancelOrder, _ctx: &mut Self::Context) -> Self::Result {
        // Take ownership to avoid overlapping mutable borrows of `self`.
        let order = self
            .orders
            .remove(&msg.order_id)
            .ok_or_else(|| "Order not found".to_string())?;

        // Basic validation: only the user who created the order can cancel it.
        if order.user_id != msg.user_id {
            return Err("User not authorized to cancel this order".to_string());
        }

        self.remove_order_from_book(order.order_id, order.side, order.price);

        Ok(msg.order_id)
    }
}

impl Handler<GetDepth> for MatchingEngine {
    type Result = Result<crate::output::DepthResponse, String>;

    fn handle(&mut self, _msg: GetDepth, _ctx: &mut Self::Context) -> Self::Result {
        let bids = self
            .orderbook
            .bids
            .iter()
            .map(|(price, orders)| {
                let total_quantity: Decimal =
                    orders.iter().map(|o| o.quantity - o.filled_quantity).sum();
                (price.0.to_string(), total_quantity.to_string())
            })
            .collect();

        let asks = self
            .orderbook
            .asks
            .iter()
            .map(|(price, orders)| {
                let total_quantity: Decimal =
                    orders.iter().map(|o| o.quantity - o.filled_quantity).sum();
                (price.to_string(), total_quantity.to_string())
            })
            .collect();

        Ok(crate::output::DepthResponse { bids, asks })
    }
}
impl Handler<GetMarketDepth> for MatchingEngine {
    type Result = Result<crate::output::DepthResponse, String>;

    fn handle(&mut self, msg: GetMarketDepth, _ctx: &mut Self::Context) -> Self::Result {
        let market = self
            .market_manager
            .get_market_mut(&msg.market_pair)
            .ok_or_else(|| format!("Market {} not found", msg.market_pair))?;

        // Return market-specific depth
        let bids = market
            .orderbook
            .bids
            .iter()
            .map(|(price, orders)| {
                let total_quantity: Decimal =
                    orders.iter().map(|o| o.quantity - o.filled_quantity).sum();
                (price.0.to_string(), total_quantity.to_string())
            })
            .collect();

        let asks = market
            .orderbook
            .asks
            .iter()
            .map(|(price, orders)| {
                let total_quantity: Decimal =
                    orders.iter().map(|o| o.quantity - o.filled_quantity).sum();
                (price.to_string(), total_quantity.to_string())
            })
            .collect();

        Ok(crate::output::DepthResponse { bids, asks })
    }
}

// --- Core Matching Logic ---

impl MatchingEngine {
    fn add_order_to_book(&mut self, order: Order) {
        let book_side = match order.side {
            Side::Buy => self
                .orderbook
                .bids
                .entry(std::cmp::Reverse(order.price))
                .or_default(),
            Side::Sell => self.orderbook.asks.entry(order.price).or_default(),
        };
        book_side.push_back(order);
    }

    fn remove_order_from_book(&mut self, order_id: Uuid, side: Side, price: Decimal) {
        let book_side = match side {
            Side::Buy => self.orderbook.bids.get_mut(&std::cmp::Reverse(price)),
            Side::Sell => self.orderbook.asks.get_mut(&price),
        };

        if let Some(orders_at_price) = book_side {
            orders_at_price.retain(|o| o.order_id != order_id);
            if orders_at_price.is_empty() {
                match side {
                    Side::Buy => self.orderbook.bids.remove(&std::cmp::Reverse(price)),
                    Side::Sell => self.orderbook.asks.remove(&price),
                };
            }
        }
    }

    fn match_order(&mut self, taker_order: &mut Order) -> Vec<Fill> {
        let mut fills = Vec::new();
        let mut orders_to_remove = Vec::new();

        match taker_order.side {
            Side::Sell => {
                for (price, orders_at_price) in self.orderbook.bids.iter_mut() {
                    if taker_order.price > price.0 {
                        break;
                    } // Taker wants to sell for more than buyers are offering

                    for maker_order in orders_at_price.iter_mut() {
                        if taker_order.filled_quantity >= taker_order.quantity {
                            break;
                        }

                        let trade_qty = std::cmp::min(
                            taker_order.quantity - taker_order.filled_quantity,
                            maker_order.quantity - maker_order.filled_quantity,
                        );

                        taker_order.filled_quantity += trade_qty;
                        maker_order.filled_quantity += trade_qty;

                        fills.push(Fill {
                            trade_id: Uuid::new_v4(),
                            price: maker_order.price,
                            quantity: trade_qty,
                            maker_order_id: maker_order.order_id,
                            taker_order_id: taker_order.order_id,
                            timestamp: chrono::Utc::now().timestamp_millis(),
                        });

                        if maker_order.quantity == maker_order.filled_quantity {
                            orders_to_remove.push(maker_order.order_id);
                        }
                    }
                }
            }
            Side::Buy => {
                for (price, orders_at_price) in self.orderbook.asks.iter_mut() {
                    if taker_order.price < *price {
                        break;
                    } // Taker wants to buy for less than sellers are asking

                    for maker_order in orders_at_price.iter_mut() {
                        if taker_order.filled_quantity >= taker_order.quantity {
                            break;
                        }

                        let trade_qty = std::cmp::min(
                            taker_order.quantity - taker_order.filled_quantity,
                            maker_order.quantity - maker_order.filled_quantity,
                        );

                        taker_order.filled_quantity += trade_qty;
                        maker_order.filled_quantity += trade_qty;

                        fills.push(Fill {
                            trade_id: Uuid::new_v4(),
                            price: maker_order.price,
                            quantity: trade_qty,
                            maker_order_id: maker_order.order_id,
                            taker_order_id: taker_order.order_id,
                            timestamp: chrono::Utc::now().timestamp_millis(),
                        });

                        if maker_order.quantity == maker_order.filled_quantity {
                            orders_to_remove.push(maker_order.order_id);
                        }
                    }
                }
            }
        }

        // Clean up fully filled orders without overlapping borrows
        for id in orders_to_remove {
            if let Some(order) = self.orders.remove(&id) {
                self.remove_order_from_book(id, order.side, order.price);
            }
        }

        fills
    }
}
