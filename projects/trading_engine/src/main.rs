use std::collections::HashMap;

#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask,
}
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
        let scalar = 100_000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            scalar,
            integral,
            fractional,
        }
    }
}
#[derive(Debug)]
struct Limit {
    price: Price,
    order: Vec<Order>,
}

impl Limit {
    fn new(price: Price) -> Limit {
        Limit {
            price,
            order: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        self.order.push(order);
    }
}
#[derive(Debug)]
struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { size, bid_or_ask }
    }
}

#[derive(Debug)]
struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl OrderBook {
    fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    fn add_limit_order(&mut self, price: f64, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::Ask => {}
            BidOrAsk::Bid => {
                let price = Price::new(price);
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

fn main() {
    let order = Order::new(BidOrAsk::Bid, 2.45);
    let order2 = Order::new(BidOrAsk::Bid, 5.5);
    let mut order_book = OrderBook::new();
    order_book.add_limit_order(4.4, order2);
    order_book.add_limit_order(4.4, order);
    println!("{:?}", order_book);
}
