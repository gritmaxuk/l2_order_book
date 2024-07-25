use std::collections::BTreeMap;
use ordered_float::OrderedFloat;
use serde::Deserialize;
use tokio::sync::RwLock;
use std::sync::Arc;

use crate::network::messages::OrderBookUpdate;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Order {
    pub price: f64,
    pub quantity: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PriceLevel {
    pub price: f64,
    pub orders: Vec<Order>,
}

#[derive(Debug, Clone)]
pub struct OrderBook {
    pub bids: BTreeMap<OrderedFloat<f64>, Order>,
    pub asks: BTreeMap<OrderedFloat<f64>, Order>,
    pub best_bid: Option<f64>,
    pub best_ask: Option<f64>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            best_bid: None,
            best_ask: None,
        }
    }

    pub fn add_order(&mut self, order: Order, side: &str) {
        let price = OrderedFloat(order.price);
        if side == "buy" {
            self.bids.insert(price, order);
            self.update_best_bid();
        } else if side == "sell" {
            self.asks.insert(price, order);
            self.update_best_ask();
        }
    }

    pub fn remove_order(&mut self, price: f64, side: &str) {
        let price = OrderedFloat(price);
        if side == "buy" {
            self.bids.remove(&price);
            self.update_best_bid();
        } else if side == "sell" {
            self.asks.remove(&price);
            self.update_best_ask();
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
    }

    pub fn process_update(&mut self, update: OrderBookUpdate) {
        if update.quantity == 0.0 {
            self.remove_order(update.price, &update.side);
        } else {
            let order = Order {
                price: update.price,
                quantity: update.quantity,
            };
            self.add_order(order, &update.side);
        }
    }

    fn update_best_bid(&mut self) {
        self.best_bid = self.bids.keys().rev().next().map(|p| p.into_inner());
    }

    fn update_best_ask(&mut self) {
        self.best_ask = self.asks.keys().next().map(|p| p.into_inner());
    }

    pub fn get_best_bid(&self) -> Option<f64> {
        self.best_bid
    }

    pub fn get_best_ask(&self) -> Option<f64> {
        self.best_ask
    }
}