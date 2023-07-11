use super::orderbook::OrderBook;
use std::collections::HashMap;

//quote is what we sell in the return of base
//base bitcoin
//quote solana
#[derive(Debug,PartialEq, Eq,Hash,Clone)]
pub struct TradingPair{
    base:String,
    quote:String
}

impl TradingPair{
    pub fn new(base:String,quote:String)->TradingPair{
        TradingPair{base,quote}
    }
}



pub struct MatchingEngine{
    orderbooks:HashMap<TradingPair,OrderBook>,
}

impl MatchingEngine{
    pub fn new()->MatchingEngine{
        MatchingEngine{
            orderbooks:HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self,pair:TradingPair){
        self.orderbooks.insert(pair.clone(),OrderBook::new());
        println!("Creating new orderbook for market {:?}",pair);
    }
}