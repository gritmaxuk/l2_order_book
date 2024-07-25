use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WsMessage};
use futures_util::{StreamExt, SinkExt};
use serde_json::Value;
use crate::network::messages::{Message, OrderBookUpdate, OrderBookSnapshot};
use crate::core::SharedOrderBook;

pub async fn connect(url: &str, order_book: SharedOrderBook) -> tokio_tungstenite::tungstenite::Result<()> {
    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    // Example: Sending a subscription message
    let subscription_msg = r#"{"type": "subscribe", "channel": "orderbook"}"#;
    write.send(WsMessage::Text(subscription_msg.into())).await?;

    // Reading messages
    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => {
                if let Ok(text) = msg.to_text() {
                    match serde_json::from_str::<Message>(text) {
                        Ok(parsed_msg) => handle_message(parsed_msg, &order_book).await,
                        Err(e) => eprintln!("Failed to parse message: {:?}", e),
                    }
                }
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    Ok(())
}

async fn handle_message(msg: Message, order_book: &SharedOrderBook) {
    match msg {
        Message::Update(update) => handle_update(update, order_book).await,
        Message::Snapshot(snapshot) => handle_snapshot(snapshot, order_book).await,
    }
}

async fn handle_update(update: OrderBookUpdate, order_book: &SharedOrderBook) {
    order_book.process_update(update).await;
}

async fn handle_snapshot(snapshot: OrderBookSnapshot, order_book: &SharedOrderBook) {
    order_book.process_snapshot(snapshot.bids, snapshot.asks).await;
}