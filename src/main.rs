mod matching_engine;
use matching_engine::orderbook::{OrderBook,BidOrAsk,Order};
use matching_engine::engine::{MatchingEngine, TradingPair};
use rust_decimal_macros::dec;

fn main() {
    // let mut limit=Limit::new(65.6);
    let buyorder_fromalice=Order::new(BidOrAsk::Bid,3.5);
    let buyorder_fromBob=Order::new(BidOrAsk::Bid,2.45);    // limit.add_order(buyorder);
    // limit.add_order(sellorder);
    let mut orderbook=OrderBook::new();
    orderbook.add_order(dec!(4.4), buyorder_fromalice);
    orderbook.add_order(dec!(2.2), buyorder_fromBob);

    let sell_order=Order::new(BidOrAsk::Ask,2.1);
    orderbook.add_order(dec!(2.6), sell_order);
    let mut engine=MatchingEngine::new();
    let pair=TradingPair::new("BTC".to_string(),"Sol".to_string());
    engine.add_new_market(pair.clone());

    let buy_order=Order::new(BidOrAsk::Bid,6.5);
    engine.place_limit_order(pair, dec!(10.00), buy_order).unwrap();





    //println!("{:?}",orderbook);
}
