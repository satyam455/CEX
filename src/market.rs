use crate::orderbook::Orderbook;
use crate::token::TradingPair;
use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Market {
    pub pair: TradingPair,
    pub orderbook: Orderbook,
    pub base_liquidity: Decimal,  //available base token
    pub quote_liquidity: Decimal, //available quote token
    pub price: Decimal,           //current market price of the pair
    pub is_active: bool,
}

impl Market {
    pub fn new(pair: TradingPair, initial_price: Decimal) -> Self {
        Self {
            pair,
            orderbook: Orderbook::new(),
            base_liquidity: Decimal::ZERO,
            quote_liquidity: Decimal::ZERO,
            price: initial_price,
            is_active: true,
        }
    }

    pub fn add_liquidity(&mut self, base_amt: Decimal, quote_amt: Decimal) {
        self.base_liquidity += base_amt;
        self.quote_liquidity += quote_amt;

        if self.base_liquidity > Decimal::ZERO {
            self.price = self.quote_liquidity / self.base_liquidity;
        }

        self.is_active = true;
        println!(
            "Added liquidity to {}: {} {} and {} {}",
            self.pair.pair_symbol, base_amt, self.pair.base_tkn, quote_amt, self.pair.quote_tkn
        );
    }
}

pub struct MarketManager {
    markets: HashMap<String, Market>,
}

impl MarketManager {
    pub fn new() -> Self {
        Self {
            markets: HashMap::new(),
        }
    }

    pub fn get_market_mut(&mut self, key: &str) -> Option<&mut Market> {
        self.markets.get_mut(key)
    }

    pub fn create_market(
        &mut self,
        pair: TradingPair,
        initial_price: Decimal,
    ) -> Result<(), String> {
        if self.markets.contains_key(&pair.pair_symbol) {
            return Err(format!("Market for {} already exists", pair.pair_symbol));
        }

        let market = Market::new(pair.clone(), initial_price);
        self.markets.insert(pair.pair_symbol.clone(), market);

        println!(
            "Created market for {} with initial price {}",
            pair.pair_symbol, initial_price
        );
        Ok(())
    }
}

impl Default for MarketManager {
    fn default() -> Self {
        let mut manager = Self::new();

        let create_pairs = vec![
            ("TAN", "KAN", Decimal::new(5, 0)), // 1 TAN = 5 KAN
            ("ADI", "PRA", Decimal::new(3, 0)), // 1 ADI = 3 PRA
            ("RAC", "SAT", Decimal::new(2, 0)), // 1 RAC = 2 SAT
        ];

        for (base, quote, price) in create_pairs {
            let pair = TradingPair::new(base, quote);
            manager
                .create_market(pair, price)
                .expect("failed to create default market");
        }
        manager
    }
}
