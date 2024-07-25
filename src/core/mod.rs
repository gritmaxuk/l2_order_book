pub mod order_book;

use crate::network::messages::OrderBookUpdate;
use order_book::OrderBook; 
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Clone)]
pub struct SharedOrderBook {
    inner: Arc<RwLock<OrderBook>>,
}

impl SharedOrderBook {
    pub fn new() -> Self {
        SharedOrderBook {
            inner: Arc::new(RwLock::new(OrderBook::new())),
        }
    }

    pub async fn process_snapshot(&self, bids: Vec<OrderBookUpdate>, asks: Vec<OrderBookUpdate>) {
        let mut order_book = self.inner.write().await;
        order_book.process_snapshot(bids, asks);
    }

    pub async fn process_update(&self, update: OrderBookUpdate) {
        let mut order_book = self.inner.write().await;
        order_book.process_update(update);
    }

    pub async fn read(&self) -> tokio::sync::RwLockReadGuard<'_, OrderBook> {
        self.inner.read().await
    }
}