mod matching_engine;
use matching_engine::engine::{MatchingEngine, TradingPair};
use matching_engine::orderbook::{BidOrAsk, Order, OrderBook};
fn main() {
    let order = Order::new(BidOrAsk::Bid, 2.45);
    let order2 = Order::new(BidOrAsk::Bid, 5.5);
    let mut order_book = OrderBook::new();
    order_book.add_limit_order(4.4, order2);
    order_book.add_limit_order(4.4, order);
    println!("{:?}", order_book);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());
    let order3 = Order::new(BidOrAsk::Bid, 2.45);
    let eth_pair = TradingPair::new("ETH".to_string(), "USD".to_string());
    // if let Ok(value) = engine.place_limit_order(eth_pair, 1000.0, order3) {}
    engine.place_limit_order(eth_pair, 1000.0, order3).unwrap()
}
