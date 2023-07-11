use super::orderbook::{OrderBook,Order};
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
    pub fn to_string(self)->String{
        format!("{}_{}",self.base,self.quote)

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
        println!("Creating new orderbook for market {:?}",pair.to_string());
    }

    pub fn place_limit_order(&mut self,
    pair:TradingPair,price:f64,order:Order)->Result<(),String>{
        match self.orderbooks.get_mut(&pair){
            Some(orderbook)=>{
                orderbook.add_order(price, order);

                println!("the limit order is set at the price level{:?}",price);
                Ok(())
            }
            None=>{
                Err(format!(
                    "The order book for given ({}) pair doesn't exist",pair.to_string()
                ))
            }
        }
    }
}