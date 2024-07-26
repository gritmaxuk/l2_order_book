use l2_order_book::providers;
use l2_order_book::utils::config::{Config, Provider};
use l2_order_book::core::SharedOrderBook;
use log::{info, error};

#[tokio::main]
async fn main() {
    // config
    let config = Config::read_config();
    config.validate(); 

    // logger
    env_logger::init();
    println!("Configuration: {:?}", config);

    // check provider
    let provider = config.provider.name.unwrap_or(Provider::None);
    if provider != Provider::Deribit {
        panic!("Unsupported provider. Only Deribit is supported. Provided: {:?}", provider)
    }

    let instrument = config.exchange.instrument.clone().unwrap();

    // subscire to provider 
    let order_book = SharedOrderBook::new(config.exchange.depth_limit.unwrap());

    match providers::deribit::subscribe_to_order_book(order_book.clone(), &instrument).await {
        Ok(_) => {
            info!("Subscription successful");
            if let Some(best_bid) = order_book.get_best_bid().await {
                println!("Best Bid: {}", best_bid);
            }
            if let Some(best_ask) = order_book.get_best_ask().await {
                println!("Best Ask: {}", best_ask);
            }
        },
        Err(e) => error!("Subscription error: {:?}", e),
    }
}