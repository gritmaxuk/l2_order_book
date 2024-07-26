use tokio::runtime::Runtime;
use l2_order_book::utils::config::Config;
use l2_order_book::core::SharedOrderBook;
use log::{info, error};
use std::env;
use l2_order_book::providers::deribit::subscribe_to_order_book;

fn main() {
    // config
    let config = Config::read_config();
    config.validate(); 

    env_logger::init();

    // extract params 
    let instrument = config.exchange.instrument.clone().unwrap();
    let depth_limit = config.exchange.depth_limit.unwrap();
    let provider_type = config.provider.name.clone().unwrap();

    info!("Instrument specified: {}", instrument);
    info!("Provider type: {:?}", provider_type);

    let rt = Runtime::new().unwrap();
    let order_book = SharedOrderBook::new(depth_limit);

    rt.block_on(async {
        match subscribe_to_order_book(order_book.clone(), &instrument).await {
            Ok(_) => {
                info!("Subscription successful");
                if let Some(best_bid) = order_book.get_best_bid().await {
                    info!("Best Bid: {}", best_bid);
                }
                if let Some(best_ask) = order_book.get_best_ask().await {
                    info!("Best Ask: {}", best_ask);
                }
            },
            Err(e) => error!("Subscription error: {:?}", e),
        }
    });
}