pub mod messages;
pub mod order_book;

use messages::OrderBookUpdate;
use order_book::OrderBook;
use std::sync::Arc;
use tokio::sync::RwLock;

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

    pub async fn get_bids(&self) -> Option<Vec<f64>> {
        let order_book = self.inner.read().await;
        Some(
            order_book
                .bids
                .keys()
                .map(|k| k.into_inner())
                .collect::<Vec<_>>(),
        )
    }

    pub async fn get_asks(&self) -> Option<Vec<f64>> {
        let order_book = self.inner.read().await;
        Some(
            order_book
                .asks
                .keys()
                .map(|k| k.into_inner())
                .collect::<Vec<_>>(),
        )
    }
}
