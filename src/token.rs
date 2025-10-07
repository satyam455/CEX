use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Token {
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub total_supply: Decimal,
    pub mint_authority: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingPair {
    pub base_tkn: String,    //eg TAN
    pub quote_tkn: String,   // eg KAN
    pub pair_symbol: String, // eg "TAN_KAN"
}

impl TradingPair {
    pub fn new(base: &str, quote: &str) -> Self {
        Self {
            base_tkn: base.to_string(),
            quote_tkn: quote.to_string(),
            pair_symbol: format!("{}_{}", base, quote),
        }
    }



pub struct TokenRegistry {
    tokens: HashMap<String, Token>,
}

impl TokenRegistry {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }



pub fn create_token(
    &mut self, symbol: String, name: String,
     decimals: u8, initial_supply: Decimal,
    ) -> Result<Token, String> {
        if self.tokens.contains_key(&symbol) {
            return Err(format!("Token {} already exists", symbol));
        }

        let token = Token {
            symbol: symbol.clone(),
            name,
            decimals,
            total_supply: initial_supply,
            mint_authority: None,
        };

        self.tokens.insert(symbol.clone(), token.clone());
        Ok(token)
    }

    pub fn get_token(&self, symbol: &str) -> Option<&Token> {
        self.tokens.get(symbol)
    }
}
}

impl Default for TokenRegistry {
    fn default() -> Self {
        let mut registry = Self::new();

        let tokens_to_create = vec![
            ("TAN", "Tan Token", 18, Decimal::new(1_000_000, 0)), // 1M TAN
            ("ADI", "Adi Token", 18, Decimal::new(1_000_000, 0)), // 1M ADI
            ("PRA", "Pra Token", 18, Decimal::new(1_000_000, 0)), // 1M PRA
            ("KAN", "Kan Token", 18, Decimal::new(1_000_000, 0)), // 1M KAN
            ("SAT", "Sat Token", 18, Decimal::new(1_000_000, 0)), // 1M SAT
            ("RAC", "Rac Token", 18, Decimal::new(1_000_000, 0)), // 1M RAC
        ];


        for (symbol, name, decimals, initial_supply) in tokens_to_create {
            registry.create_token(symbol.to_string(), name.to_string(), decimals, initial_supply)
            .expect("Failed to create token");
        }

        registry
    }
}