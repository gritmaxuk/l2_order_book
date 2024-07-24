use std::collections::BTreeMap;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub id: u64,
    pub price: f64,
    pub quantity: f64,
}

#[derive(Debug, Clone)]
pub struct PriceLevel {
    pub price: f64,
    pub orders: Vec<Order>,
}

#[derive(Debug, Clone)]
pub struct OrderBook {
    pub bids: BTreeMap<OrderedFloat<f64>, PriceLevel>,
    pub asks: BTreeMap<OrderedFloat<f64>, PriceLevel>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        let price = OrderedFloat(order.price);
        let price_level = if order.quantity > 0.0 {
            self.bids.entry(price).or_insert_with(|| PriceLevel {
                price: order.price,
                orders: Vec::new(),
            })
        } else {
            self.asks.entry(price).or_insert_with(|| PriceLevel {
                price: order.price,
                orders: Vec::new(),
            })
        };
        price_level.orders.push(order);
    }

    pub fn update_order(&mut self, order: Order) {
        self.remove_order(order.id);
        self.add_order(order);
    }

    pub fn remove_order(&mut self, order_id: u64) {
        let mut remove_price_level = None;

        for price_level in self.bids.values_mut() {
            if let Some(pos) = price_level.orders.iter().position(|x| x.id == order_id) {
                price_level.orders.remove(pos);
                if price_level.orders.is_empty() {
                    remove_price_level = Some(price_level.price);
                }
                break;
            }
        }

        if let Some(price) = remove_price_level {
            self.bids.remove(&OrderedFloat(price));
            return;
        }

        for price_level in self.asks.values_mut() {
            if let Some(pos) = price_level.orders.iter().position(|x| x.id == order_id) {
                price_level.orders.remove(pos);
                if price_level.orders.is_empty() {
                    remove_price_level = Some(price_level.price);
                }
                break;
            }
        }

        if let Some(price) = remove_price_level {
            self.asks.remove(&OrderedFloat(price));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_order() {
        let mut order_book = OrderBook::new();
        let order = Order {
            id: 1,
            price: 100.0,
            quantity: 1.0,
        };
        order_book.add_order(order.clone());

        assert_eq!(order_book.bids.len(), 1);
        assert_eq!(order_book.bids[&OrderedFloat(100.0)].orders.len(), 1);
        assert_eq!(order_book.bids[&OrderedFloat(100.0)].orders[0], order);
    }

    #[test]
    fn test_update_order() {
        let mut order_book = OrderBook::new();
        let order1 = Order {
            id: 1,
            price: 100.0,
            quantity: 1.0,
        };
        let order2 = Order {
            id: 1,
            price: 100.0,
            quantity: 2.0,
        };
        order_book.add_order(order1.clone());
        order_book.update_order(order2.clone());

        assert_eq!(order_book.bids.len(), 1);
        assert_eq!(order_book.bids[&OrderedFloat(100.0)].orders.len(), 1);
        assert_eq!(order_book.bids[&OrderedFloat(100.0)].orders[0], order2);
    }

    #[test]
    fn test_remove_order() {
        let mut order_book = OrderBook::new();
        let order = Order {
            id: 1,
            price: 100.0,
            quantity: 1.0,
        };
        order_book.add_order(order.clone());
        order_book.remove_order(order.id);

        assert_eq!(order_book.bids.len(), 0);
    }
}