use std::collections::{BTreeMap, VecDeque};
use crate::order::{Order, Side};


#[derive(Debug)]
pub struct MatchingEngine {
    pub bids: BTreeMap<u64, VecDeque<Order>>,
    pub asks: BTreeMap<u64, VecDeque<Order>>,
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn process_order(&mut self, mut order: Order) {
        match order.Side {
            Side::Bid => self.ma
        }
    }

    fn add_limit_order(&mut self, order: Order) {
        let book = match order.side {
            Side::Bid => &mut self.bids,
            Side::Ask => &mut self.asks,
        };

        book.entry(order.price).or_insert_with(VecDeque::new).push_back(order);
    }
}