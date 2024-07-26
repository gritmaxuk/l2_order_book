pub mod order_book;
pub mod messages;

use messages::OrderBookUpdate;
use order_book::OrderBook;
use tokio::sync::RwLock;
use std::sync::Arc;

/// A shared for thread-safty read-write lock for the order book.
#[derive(Clone)]
pub struct SharedOrderBook {
    inner: Arc<RwLock<OrderBook>>,
}

impl SharedOrderBook {
    pub fn new(depth_limit: usize) -> Self {
        SharedOrderBook {
            inner: Arc::new(RwLock::new(OrderBook::new(depth_limit))),
        }
    }

    pub async fn process_snapshot(&self, bids: Vec<OrderBookUpdate>, asks: Vec<OrderBookUpdate>) {
        let mut order_book = self.inner.write().await;
        order_book.process_snapshot(bids, asks);
    }


    pub async fn get_best_bid(&self) -> Option<f64> {
        let order_book = self.inner.read().await;
        order_book.get_best_bid()
    }

    pub async fn get_best_ask(&self) -> Option<f64> {
        let order_book = self.inner.read().await;
        order_book.get_best_ask()
    }

    pub async fn get_used_depth_limit(&self) -> usize { 
        let order_book = self.inner.read().await;
        order_book.asks.len()
    }

}