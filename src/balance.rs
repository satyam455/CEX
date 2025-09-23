use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct UserBal {
    pub balances: HashMap<String, Decimal>, //token symbol -> balance
}

impl userBalance {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }

    pub fn get_balance(&self, token_symbol: &str) -> Decimal {
        self.balances.get(token).copied().unwrap_or(Decimal::ZERO)
    }

    pub fn has_sufficient_balance(&mut self, token_symbol: &str, amount: Decimal) -> bool {
        self.get_balance(token_symbol) >= amount
    }

    pub fn subtract_balance(&mut self, token_symbol: &str, amount: Decimal)
    -> Result<(), String> {
        let current = self.get_balance(token_symbol);
        if current < amount {
            return Err(format!("Insufficient balance has {}, needs {}", token_symbol, amount));
        }

        self.balances.insert(token_symbol.to_string(), current - amount);
        Ok(())
    }

    pub struct BalanceManager {
        user_balances: HashMap<String, UserBal>,
    }

    impl BalanceManager {
        pub fn new() -> Self {
            Self {
                user_balances: HashMap::new(),
            }
        }

        pub fn get_user_balance(&self, user_id: &str) -> Option<&UserBal> {
            self.user_balances.get(user_id)
        }


        pub fn initialize_market_maker(&mut self, maker_id: &str) {
            let mut balance = UserBal::new();
            
            balance.balances.insert("TAN".to_string(), Decimal::new(100_000, 0));
            balance.balances.insert("KAN".to_string(), Decimal::new(500_000, 0));
            balance.balances.insert("ADI".to_string(), Decimal::new(100_000, 0));
            balance.balances.insert("PRA".to_string(), Decimal::new(100_000, 0));
            balance.balances.insert("SAT".to_string(), Decimal::new(100_000, 0));
            balance.balances.insert("RAC".to_string(), Decimal::new(100_000, 0));

            self.user_balances.insert(maker_id.to_string(), balance);
            println!("Initialized market maker balance for {}", maker_id);


        }
    }

    
}

