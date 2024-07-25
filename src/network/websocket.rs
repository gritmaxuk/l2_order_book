use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WsMessage};
use futures_util::{StreamExt, SinkExt};
use serde_json::Value;
use crate::network::messages::{Message, OrderBookUpdate, OrderBookSnapshot};

pub async fn connect(url: &str) -> tokio_tungstenite::tungstenite::Result<()> {
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
                        Ok(parsed_msg) => handle_message(parsed_msg),
                        Err(e) => eprintln!("Failed to parse message: {:?}", e),
                    }
                }
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    Ok(())
}

fn handle_message(msg: Message) {
    match msg {
        Message::Update(update) => handle_update(update),
        Message::Snapshot(snapshot) => handle_snapshot(snapshot),
    }
}

fn handle_update(update: OrderBookUpdate) {
    println!("Update received: {:?}", update);
    // Process the update message
}

fn handle_snapshot(snapshot: OrderBookSnapshot) {
    println!("Snapshot received: {:?}", snapshot);
    // Process the snapshot message
}