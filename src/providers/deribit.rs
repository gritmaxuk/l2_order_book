use anyhow::Error;
use deribit::{models::{PublicSubscribeRequest, SubscriptionData, SubscriptionMessage, SubscriptionParams, WithChannel}, DeribitSubscriptionClient, DeribitAPIClient};
use futures::StreamExt;
use crate::{core::{messages::OrderBookUpdate, SharedOrderBook}, utils::config::ExchangeConfig};
use log::{debug, error, info};

pub async fn subscribe_to_order_book(order_book: SharedOrderBook, config: &ExchangeConfig) -> Result<(), Error> {
    let (mut client, mut subscription) = create_client().await?;
    
    subscribe_to_instrument(config, &mut client).await?;
    while let Some(message) = subscription.next().await {
        match message {
            Ok(subscription_meassage) => {

                match parse(subscription_meassage) {
                    Some((bids, asks)) => { 
                        order_book.process_snapshot(bids, asks).await; // snapshot update, data equal to depth limit
                        debug!("Order book updated for instrument: {}", config.instrument.clone().unwrap());
                    },
                    None => {}
                }
            },
            Err(e) => {
                error!("Error in subscription message: {:?}", e);
            },
            _ => {} // Ignore other types of messages
        }
    }

    Ok(())
}

async fn create_client() -> Result<(DeribitAPIClient, DeribitSubscriptionClient), Error> {
    let drb = deribit::DeribitBuilder::default().build().expect("Cannot create deribit client");
    Ok(drb.connect().await?)
}

async fn subscribe_to_instrument(config: &ExchangeConfig, client: &mut DeribitAPIClient) -> Result<(), Error> {
    let instrument = config.instrument.clone().unwrap();
    let depth_limit = config.depth_limit.unwrap();

    let subscription_channel = format!("book.{}.5.{}.100ms", instrument, depth_limit);
    let req = PublicSubscribeRequest::new(&[subscription_channel.into()]);
    client.call(req).await?;
    
    Ok(())
}

fn parse(msg: SubscriptionMessage) -> Option<(Vec<OrderBookUpdate>, Vec<OrderBookUpdate>)> {
    match msg {
        SubscriptionMessage { params: SubscriptionParams::Subscription(SubscriptionData::GroupedBook(WithChannel { data, .. },)), .. } => {
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

            Some((bids, asks))

        },
        _ => None // Ignore other types of messages
    }
}