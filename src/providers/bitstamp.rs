use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::{core::SharedOrderBook, utils::config::ExchangeConfig};
use crate::core::messages::OrderBookUpdate;
use log::{debug, error};

#[derive(Serialize)]
struct SubscribeMessage {
    event: String,
    data: SubscribeData,
}

#[derive(Serialize)]
struct SubscribeData {
    channel: String,
}

#[derive(Deserialize, Debug)]
struct RawOrderBookData {
    bids: Vec<[String; 2]>,
    asks: Vec<[String; 2]>,
}

#[derive(Deserialize, Debug)]
struct RawOrderBook {
    data: RawOrderBookData,
}

#[derive(Debug)]
struct OrderBook {
    bids: Vec<OrderBookUpdate>,
    asks: Vec<OrderBookUpdate>,
}

impl From<RawOrderBookData> for OrderBook {
    fn from(raw: RawOrderBookData) -> Self {
        let bids = raw.bids.into_iter().map(|b| {
            OrderBookUpdate {
                price: b[0].parse().unwrap_or(0.0),
                quantity: b[1].parse().unwrap_or(0.0),
                side: "buy".to_string(),
            }
        }).collect();

        let asks = raw.asks.into_iter().map(|a| {
            OrderBookUpdate {
                price: a[0].parse().unwrap_or(0.0),
                quantity: a[1].parse().unwrap_or(0.0),
                side: "sell".to_string(),
            }
        }).collect();

        OrderBook { bids, asks }
    }
}

/// Subscribe to the order book channel.
pub async fn subscribe_to_order_book(order_book: SharedOrderBook, config: &ExchangeConfig) -> Result<(), Box<dyn Error>> {
    // default provider (does not work without)
    rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();
    
    // setuo ws stream
    let url = "wss://ws.bitstamp.net";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let (mut write, mut read) = ws_stream.split();

    // sbiscribe to channel
    let instrument = config.normalized_instrument().expect("Cannot normalize instrument");
    let subscribe_message = SubscribeMessage {
        event: "bts:subscribe".to_string(),
        data: SubscribeData {
            channel: format!("order_book_{}", instrument),
        },
    };

    let subscribe_message = serde_json::to_string(&subscribe_message)?;
    write.send(Message::Text(subscribe_message)).await?;

    // process messages 
    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => {
                if let Ok(raw_order_book) = serde_json::from_str::<RawOrderBook>(&text) {
                    let order_book_data: RawOrderBookData = raw_order_book.data;

                    // raw order book to OrderBook struct
                    let order_book_update: OrderBook = order_book_data.into();
                    
                    // procced order book snapshot
                    order_book.process_snapshot(order_book_update.bids.clone(), order_book_update.asks.clone()).await;

                    debug!("Order Book: {:?}", order_book_update);
                } else {
                    //error!("Failed to parse Order: {}", text);
                    // SKIP other OK messages 
                }
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    Ok(())
}