mod bitstamp;
mod deribit;

use crate::{
    core::SharedOrderBook,
    utils::config::{Config, Provider},
};
use tokio::sync::mpsc::{self, Sender};

pub fn subscribe_to_provider(config: Config, order_book: SharedOrderBook) -> Option<Sender<()>> {
    let provider = config.provider.name.unwrap_or(Provider::None);
    let (stop_tx, stop_rx) = mpsc::channel(1);

    tokio::task::spawn(async move {
        match provider {
            Provider::Deribit => {
                if let Err(e) =
                    deribit::subscribe_to_order_book(order_book, &config.exchange, stop_rx).await
                {
                    eprintln!("Error subscribing to Deribit: {:?}", e);
                }
            }
            Provider::Bitstamp => {
                if let Err(e) =
                    bitstamp::subscribe_to_order_book(order_book.clone(), &config.exchange, stop_rx)
                        .await
                {
                    eprintln!("Error subscribing to Bitstamp: {:?}", e);
                }
            }
            _ => {
                panic!(
                    "Unsupported provider. Only Deribit and Bitstamp are supported. Provided: {:?}",
                    provider
                )
            }
        }
    });

    Some(stop_tx.clone())
}
