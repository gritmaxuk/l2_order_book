pub mod order_book;
pub mod messages;

use messages::OrderBookUpdate;
use order_book::OrderBook;
use tokio::sync::RwLock;
use std::sync::Arc;

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

    pub async fn process_update(&self, update: OrderBookUpdate) {
        let mut order_book = self.inner.write().await;
        order_book.process_update(update);
    }

    pub async fn get_best_bid(&self) -> Option<f64> {
        let order_book = self.inner.read().await;
        order_book.get_best_bid()
    }

    pub async fn get_best_ask(&self) -> Option<f64> {
        let order_book = self.inner.read().await;
        order_book.get_best_ask()
    }

    pub async fn read(&self) -> tokio::sync::RwLockReadGuard<'_, OrderBook> {
        self.inner.read().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_shared_order_book() {
        let shared_order_book = SharedOrderBook::new(10);

        let update = OrderBookUpdate {
            price: 100.0,
            quantity: 1.0,
            side: "buy".to_string(),
        };

        shared_order_book.process_update(update.clone()).await;
        let best_bid = shared_order_book.get_best_bid().await;
        assert_eq!(best_bid, Some(100.0));

        let update_remove = OrderBookUpdate {
            price: 100.0,
            quantity: 0.0,
            side: "buy".to_string(),
        };

        shared_order_book.process_update(update_remove).await;
        let best_bid = shared_order_book.get_best_bid().await;
        assert_eq!(best_bid, None);
    }
}