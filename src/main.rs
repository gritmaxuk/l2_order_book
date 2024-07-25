use tokio::runtime::Runtime;
use l2_order_book::cli::get_matches;
use l2_order_book::utils::config::Config;
use l2_order_book::core::SharedOrderBook;
use log::{info, error};
use std::env;
use l2_order_book::providers::deribit::subscribe_to_order_book;

fn main() {
    env_logger::init();

    let matches = get_matches();

    let instrument = matches.get_one::<String>("instrument").cloned()
        .or_else(|| env::var("INSTRUMENT").ok())
        .or_else(|| Config::from_file("config.toml").exchange.instrument)
        .expect("Instrument not specified!");

    info!("Instrument specified: {}", instrument);
    
    let config = Config::from_file("config.toml");
    let depth_limit = config.exchange.depth_limit;

    let rt = Runtime::new().unwrap();
    let order_book = SharedOrderBook::new(depth_limit);

    rt.block_on(async {
        match subscribe_to_order_book(order_book.clone(), &instrument).await { //todo: rename 
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