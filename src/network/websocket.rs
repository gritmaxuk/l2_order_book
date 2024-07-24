use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
}

pub async fn connect(url: &str) -> Result<(), WebSocketError> {
    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    // Example: Sending a subscription message
    let subscription_msg = r#"{"type": "subscribe", "channel": "ticker"}"#;
    write.send(Message::Text(subscription_msg.into())).await?;

    // Reading messages
    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => {
                if let Ok(text) = msg.to_text() {
                    let json_msg: Value = serde_json::from_str(text)?;
                    println!("Received: {:?}", json_msg);
                }
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    Ok(())
}