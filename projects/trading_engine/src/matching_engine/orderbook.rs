#![allow(dead_code)]
use rust_decimal::prelude::*;
use std::collections::HashMap;
#[derive(Debug)]
pub enum BidOrAsk {
    Bid,
    Ask,
}
// Prev Price
// #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
// pub struct Decimal {
//     integral: u64,
//     fractional: u64,
//     scalar: u64,
// }

// impl Decimal {
//     pub fn new(price: f64) -> Decimal {
//         let scalar = 100_000;
//         let integral = price as u64;
//         let fractional = ((price % 1.0) * scalar as f64) as u64;
//         Decimal {
//             scalar,
//             integral,
//             fractional,
//         }
//     }
// }
#[derive(Debug)]
pub struct Limit {
    price: Decimal,
    order: Vec<Order>,
}

impl Limit {
    pub fn new(price: Decimal) -> Limit {
        Limit {
            price,
            order: Vec::new(),
        }
    }

    pub fn total_volume(&self) -> f64 {
        self.order
            .iter()
            .map(|order| order.size)
            .reduce(|a, b| a + b)
            .unwrap()
    }

    pub fn add_order(&mut self, order: Order) {
        self.order.push(order);
    }

    pub fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.order.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0
                }
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0
                }
            }

            if market_order.is_filled() {
                break;
            }
        }
    }
}
#[derive(Debug)]
pub struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { size, bid_or_ask }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Decimal, Limit>,
    bids: HashMap<Decimal, Limit>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        let limits = match market_order.bid_or_ask {
            BidOrAsk::Bid => self.ask_limit(),
            BidOrAsk::Ask => self.bid_limit(),
        };
        for limit_order in limits {
            limit_order.fill_order(market_order);

            if market_order.is_filled() {
                break;
            }
        }
    }

    pub fn ask_limit(&mut self) -> Vec<&mut Limit> {
        let mut limit = self.asks.values_mut().collect::<Vec<&mut Limit>>();
        limit.sort_by(|a, b| a.price.cmp(&b.price));
        limit
    }

    pub fn bid_limit(&mut self) -> Vec<&mut Limit> {
        let mut limit = self.bids.values_mut().collect::<Vec<&mut Limit>>();
        limit.sort_by(|a, b| b.price.cmp(&a.price));
        limit
    }

    pub fn add_limit_order(&mut self, price: Decimal, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::Ask => {
                let limit = self.asks.get_mut(&price);
                match limit {
                    Some(limit) => {
                        println!("Already got limit!");
                        limit.add_order(order);
                    }
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        println!("Create new limit!");
                        self.asks.insert(price, limit);
                    }
                }
            }
            BidOrAsk::Bid => {
                let limit = self.bids.get_mut(&price);
                match limit {
                    Some(limit) => {
                        println!("Already got limit!");
                        limit.add_order(order);
                    }
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        println!("Create new limit!");
                        self.bids.insert(price, limit);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn orderbook_fill_market_order_ask() {
        let mut orderbook = OrderBook::new();
        orderbook.add_limit_order(dec!(500), Order::new(BidOrAsk::Ask, 10.0));
        orderbook.add_limit_order(dec!(200), Order::new(BidOrAsk::Ask, 10.0));
        orderbook.add_limit_order(dec!(300), Order::new(BidOrAsk::Ask, 10.0));
        orderbook.add_limit_order(dec!(400), Order::new(BidOrAsk::Ask, 10.0));
        println!("{:?}", orderbook.ask_limit());

        let mut market_order = Order::new(BidOrAsk::Bid, 10.0);
        orderbook.fill_market_order(&mut market_order);
        let ask_limits = orderbook.ask_limit();
        let matched_limit = ask_limits.get(0).unwrap(); // .order.get(0).unwrap();

        assert_eq!(matched_limit.price, dec!(200));
        assert_eq!(market_order.is_filled(), true);

        let matched_order = matched_limit.order.get(0).unwrap();
        assert_eq!(matched_order.is_filled(), true);
    }

    #[test]
    fn limit_total_volume() {
        let price = dec!(10000.0);
        let mut limit = Limit::new(price);

        let buy_limit_order_a = Order::new(BidOrAsk::Bid, 100.0);
        let buy_limit_order_b = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        assert_eq!(limit.total_volume(), 200.0);
    }

    #[test]
    fn limit_order_single_fill() {
        let price = dec!(10000.0);
        let mut limit = Limit::new(price);

        let buy_limit_order = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order);
        let mut market_sell_order = Order::new(BidOrAsk::Ask, 99.0);
        limit.fill_order(&mut market_sell_order);

        println!("{:?}", limit);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.order.get(0).unwrap().size, 1.0);
    }

    #[test]
    fn limit_order_multiple_fill() {
        let price = dec!(10000.0);
        let mut limit = Limit::new(price);

        let buy_limit_order_a = Order::new(BidOrAsk::Bid, 100.0);
        let buy_limit_order_b = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);
        let mut market_sell_order = Order::new(BidOrAsk::Ask, 199.0);
        limit.fill_order(&mut market_sell_order);

        println!("{:?}", limit);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.order.get(0).unwrap().is_filled(), true);
        assert_eq!(limit.order.get(1).unwrap().is_filled(), false);
        assert_eq!(limit.order.get(1).unwrap().size, 1.0);
    }
}
