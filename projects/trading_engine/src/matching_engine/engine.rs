use rust_decimal::Decimal;

use super::orderbook::{Order, OrderBook};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String,  // BTC
    quote: String, // USD (Sell to buy base)
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair { base, quote }
    }

    pub fn to_string(self) -> String {
        format!("Base: {} : Quote: {}", self.base, self.quote)
    }
}
pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, OrderBook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair, OrderBook::new());
    }

    pub fn place_limit_order(
        &mut self,
        pair: TradingPair,
        price: Decimal,
        order: Order,
    ) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_limit_order(price, order);
                Ok(())
            }
            None => Err(format!(
                "The orderbook for the given trading pair {} doesnt exist",
                pair.to_string()
            )),
        }
    }
}
