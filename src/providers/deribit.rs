use anyhow::Error;
use deribit::models::{PublicSubscribeRequest, SubscriptionData, SubscriptionMessage, SubscriptionParams, WithChannel};
use futures::StreamExt;
use crate::core::SharedOrderBook;
use crate::network::messages::OrderBookUpdate;
use log::{info, error};

pub async fn subscribe_to_order_book(order_book: SharedOrderBook, instrument: &str) -> Result<(), Error> {
    let drb = deribit::DeribitBuilder::default().build().expect("Cannot create deribit client");

    let (mut client, mut subscription) = drb.connect().await?;

    let subscription_channel = format!("book.{}.5.20.100ms", instrument);
    let req = PublicSubscribeRequest::new(&[subscription_channel.into()]);
    client.call(req).await?;

    while let Some(message) = subscription.next().await {
        match message {
            Ok(SubscriptionMessage { params: SubscriptionParams::Subscription(SubscriptionData::GroupedBook(WithChannel { data, .. },)), .. }) => {
                let asks: Vec<OrderBookUpdate> = data.asks.into_iter().map(|(price, quantity)| OrderBookUpdate {
                    price,
                    quantity,
                    side: "sell".to_string(),
                }).collect();
                let bids: Vec<OrderBookUpdate> = data.bids.into_iter().map(|(price, quantity)| OrderBookUpdate {
                    price,
                    quantity,
                    side: "buy".to_string(),
                }).collect();

                order_book.process_snapshot(bids, asks).await;
                info!("Order book updated for instrument: {}", instrument);
            },
            Err(e) => {
                error!("Error in subscription message: {:?}", e);
            },
            _ => {}
        }
    }

    Ok(())
}