use crate::{
    core::{messages::OrderBookUpdate, SharedOrderBook},
    utils::config::ExchangeConfig,
};
use anyhow::Error;
use deribit::{
    models::{
        PublicSubscribeRequest, PublicUnsubscribeRequest, SubscriptionData, SubscriptionMessage, SubscriptionParams,
        WithChannel,
    },
    DeribitAPIClient, DeribitSubscriptionClient,
};
use futures::StreamExt;
use log::{debug, error, info};
use tokio::sync::mpsc::Receiver;

pub async fn subscribe_to_order_book(
    order_book: SharedOrderBook,
    config: &ExchangeConfig,
    mut stop_rx: Receiver<()>,
) -> Result<(), Error> {
    let (mut client, mut subscription) = create_client().await?;

    subscribe_to_instrument(config, &mut client).await?;

    loop {
        tokio::select! { 
            _ = stop_rx.recv() => {
                unsubscribe_from_instrument(config, &mut client).await?;
                break;
            },
            Some(message) = subscription.next() => {
                match message {
                    Ok(subscription_meassage) => {
                        if let Some((bids, asks)) = parse(subscription_meassage) {
                            order_book.process_snapshot(bids, asks).await; // snapshot update, data equal to depth limit

                            debug!(
                                "Order book updated for instrument: {}",
                                config.instrument.clone().unwrap()
                            );
                        }
                    }
                    Err(e) => {
                        error!("Error in subscription message: {:?}", e);
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

async fn create_client() -> Result<(DeribitAPIClient, DeribitSubscriptionClient), Error> {
    let drb = deribit::DeribitBuilder::default()
        .build()
        .expect("Cannot create deribit client");

    drb.connect().await
}

async fn subscribe_to_instrument(
    config: &ExchangeConfig,
    client: &mut DeribitAPIClient,
) -> Result<(), Error> {
    let instrument = config.instrument.clone().unwrap();
    let depth_limit = config.depth_limit.unwrap();

    let subscription_channel = format!("book.{}.5.{}.100ms", instrument, depth_limit);
    let req = PublicSubscribeRequest::new(&[subscription_channel]);
    client.call(req).await?;

    Ok(())
}

async fn unsubscribe_from_instrument(
    config: &ExchangeConfig,
    client: &mut DeribitAPIClient,
) -> Result<(), Error> {
    info!("Stopping WebSocket stream");
    println!("Requesting to close WebSocket stream for Deribit");
    
    let instrument = config.instrument.clone().unwrap();
    let depth_limit = config.depth_limit.unwrap();

    let subscription_channel = format!("book.{}.5.{}.100ms", instrument, depth_limit);
    let req = PublicUnsubscribeRequest::new(&[subscription_channel.clone()]);
    client.call(req).await?;

    info!("Unsubscribed from {}", subscription_channel);
    Ok(())
}

fn parse(msg: SubscriptionMessage) -> Option<(Vec<OrderBookUpdate>, Vec<OrderBookUpdate>)> {
    match msg {
        SubscriptionMessage {
            params:
                SubscriptionParams::Subscription(SubscriptionData::GroupedBook(WithChannel {
                    data,
                    ..
                })),
            ..
        } => {
            let asks: Vec<OrderBookUpdate> = data
                .asks
                .into_iter()
                .map(|(price, quantity)| OrderBookUpdate {
                    price,
                    quantity,
                    side: "sell".to_string(),
                })
                .collect();
            let bids: Vec<OrderBookUpdate> = data
                .bids
                .into_iter()
                .map(|(price, quantity)| OrderBookUpdate {
                    price,
                    quantity,
                    side: "buy".to_string(),
                })
                .collect();

            Some((bids, asks))
        }
        _ => None, // Ignore other types of messages
    }
}
