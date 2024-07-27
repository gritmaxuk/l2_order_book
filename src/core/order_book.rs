use log::{debug, info, warn};
use ordered_float::OrderedFloat;
use serde::Deserialize;
use std::collections::BTreeMap;

use super::messages::OrderBookUpdate;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Order {
    pub price: f64,
    pub quantity: f64,
}

/*
    BTreeMap to maintain bids and asks in ascending order of price.
    it is a sort of min-heap.
*/
#[derive(Debug, Clone)]
pub struct OrderBook {
    pub bids: BTreeMap<OrderedFloat<f64>, Order>,
    pub asks: BTreeMap<OrderedFloat<f64>, Order>,
    pub best_bid: Option<f64>,
    pub best_ask: Option<f64>,
    pub depth_limit: usize,
}

impl OrderBook {
    pub fn new(depth_limit: usize) -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            best_bid: None,
            best_ask: None,
            depth_limit,
        }
    }

    pub fn add_order(&mut self, order: Order, side: &str) {
        let price = OrderedFloat(order.price);
        if side == "buy" {
            self.bids.insert(price, order);
            self.update_best_bid();
            self.enforce_depth_limit("buy");
            debug!("Added order to bids: {:?}", self.bids.get(&price));
        } else if side == "sell" {
            self.asks.insert(price, order);
            self.update_best_ask();
            self.enforce_depth_limit("sell");
            debug!("Added order to asks: {:?}", self.asks.get(&price));
        }
    }

    pub fn remove_order(&mut self, price: f64, side: &str) {
        let price = OrderedFloat(price);
        if side == "buy" {
            self.bids.remove(&price);
            self.update_best_bid();
            warn!("Removed order from bids at price: {:?}", price);
        } else if side == "sell" {
            self.asks.remove(&price);
            self.update_best_ask();
            warn!("Removed order from asks at price: {:?}", price);
        }
    }

    pub fn process_snapshot(&mut self, bids: Vec<OrderBookUpdate>, asks: Vec<OrderBookUpdate>) {
        self.bids.clear();
        self.asks.clear();

        for bid in bids {
            let order = Order {
                price: bid.price,
                quantity: bid.quantity,
            };
            self.add_order(order, "buy");
        }

        for ask in asks {
            let order = Order {
                price: ask.price,
                quantity: ask.quantity,
            };
            self.add_order(order, "sell");
        }

        self.update_best_bid();
        self.update_best_ask();

        info!(
            "Best Bid: {:?}, Best Ask: {:?}",
            self.best_bid, self.best_ask
        );
        debug!("Order book depth limit: {}", self.bids.len());
    }

    fn update_best_bid(&mut self) {
        self.best_bid = self.bids.keys().rev().next().map(|p| p.into_inner()); // get the price of the highest bid
    }

    fn update_best_ask(&mut self) {
        self.best_ask = self.asks.keys().next().map(|p| p.into_inner()); // get the price of the lowest ask
    }

    fn enforce_depth_limit(&mut self, side: &str) {
        if side == "buy" {
            while self.bids.len() > self.depth_limit {
                let lowest_bid = self.bids.keys().next().cloned().unwrap(); // remove less competetive bid from the top
                self.bids.remove(&lowest_bid);
                warn!(
                    "Enforced depth limit on bids, removed order at price: {:?}",
                    lowest_bid
                );
            }
        } else if side == "sell" {
            while self.asks.len() > self.depth_limit {
                let highest_ask = self.asks.keys().rev().next().cloned().unwrap(); // remove more competetive ask from the bottom
                self.asks.remove(&highest_ask);
                warn!(
                    "Enforced depth limit on asks, removed order at price: {:?}",
                    highest_ask
                );
            }
        }
    }

    pub fn get_best_bid(&self) -> Option<f64> {
        self.best_bid
    }

    pub fn get_best_ask(&self) -> Option<f64> {
        self.best_ask
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_order() {
        let mut order_book = OrderBook::new(10);
        let order = Order {
            price: 100.0,
            quantity: 1.0,
        };
        order_book.add_order(order.clone(), "buy");

        assert_eq!(order_book.bids.len(), 1);
        assert_eq!(order_book.bids[&OrderedFloat(100.0)], order);
    }

    #[test]
    fn test_remove_order() {
        let mut order_book = OrderBook::new(10);
        let order = Order {
            price: 100.0,
            quantity: 1.0,
        };
        order_book.add_order(order.clone(), "buy");
        order_book.remove_order(100.0, "buy");

        assert_eq!(order_book.bids.len(), 0);
    }

    #[test]
    fn test_process_snapshot() {
        let mut order_book = OrderBook::new(10);
        let bids = vec![
            OrderBookUpdate {
                price: 100.0,
                quantity: 1.0,
                side: "buy".to_string(),
            },
            OrderBookUpdate {
                price: 101.0,
                quantity: 2.0,
                side: "buy".to_string(),
            },
        ];
        let asks = vec![
            OrderBookUpdate {
                price: 102.0,
                quantity: 1.0,
                side: "sell".to_string(),
            },
            OrderBookUpdate {
                price: 103.0,
                quantity: 2.0,
                side: "sell".to_string(),
            },
        ];

        order_book.process_snapshot(bids, asks);

        assert_eq!(order_book.bids.len(), 2);
        assert_eq!(order_book.asks.len(), 2);
        assert_eq!(order_book.best_bid, Some(101.0));
        assert_eq!(order_book.best_ask, Some(102.0));
    }

    #[test]
    fn test_enforce_depth_limit() {
        let mut order_book = OrderBook::new(2);
        let order1 = Order {
            price: 100.0,
            quantity: 1.0,
        };
        let order2 = Order {
            price: 101.0,
            quantity: 1.0,
        };
        let order3 = Order {
            price: 102.0,
            quantity: 1.0,
        };
        order_book.add_order(order1, "buy");
        order_book.add_order(order2, "buy");
        order_book.add_order(order3, "buy");

        assert_eq!(order_book.bids.len(), 2);
        assert!(order_book.bids.contains_key(&OrderedFloat(101.0)));
        assert!(order_book.bids.contains_key(&OrderedFloat(102.0)));
        assert!(!order_book.bids.contains_key(&OrderedFloat(100.0)));
    }
}
