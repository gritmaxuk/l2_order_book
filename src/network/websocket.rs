use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WsMessage};
use crate::network::messages::{Message, OrderBookUpdate, OrderBookSnapshot};
use crate::core::SharedOrderBook;
use log::{info, error};

const WS_URL_TESTNET: &'static str = "wss://test.deribit.com/ws/api/v2";

pub async fn connect(instrument: String, order_book: SharedOrderBook) -> tokio_tungstenite::tungstenite::Result<()> {
    let (ws_stream, _) = connect_async(WS_URL_TESTNET).await?;
    let (mut write, mut read) = ws_stream.split();

    info!("WebSocket connection established");

    // subscibe
    let subscription_msg = format!(r#"{{"type": "subscribe", "channel": "orderbook", "instrument": "{}"}}"#, instrument);
    write.send(WsMessage::Text(subscription_msg.into())).await?;
    info!("Subscription message sent for instrument: {}", instrument);

    // reading
    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => {
                if let Ok(text) = msg.to_text() {
                    match serde_json::from_str::<Message>(text) {
                        Ok(parsed_msg) => {
                            handle_message(parsed_msg, &order_book).await;
                        }
                        Err(e) => error!("Failed to parse message: {:?}", e),
                    }
                }
            }
            Err(e) => error!("Error: {:?}", e),
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
    info!("Processing update: {:?}", update);
    order_book.process_update(update).await;
}

async fn handle_snapshot(snapshot: OrderBookSnapshot, order_book: &SharedOrderBook) {
    info!("Processing snapshot: {:?}", snapshot);
    order_book.process_snapshot(snapshot.bids, snapshot.asks).await;
}