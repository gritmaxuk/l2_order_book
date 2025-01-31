use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrderBookUpdate {
    pub price: f64,
    pub quantity: f64,
    pub side: Side,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrderBookSnapshot {
    pub bids: Vec<OrderBookUpdate>,
    pub asks: Vec<OrderBookUpdate>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Message {
    Update(OrderBookUpdate),
    Snapshot(OrderBookSnapshot),
}
