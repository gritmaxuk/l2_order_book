use l2_order_book::providers;
use l2_order_book::utils::config::{Config, Provider};
use l2_order_book::core::SharedOrderBook;

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

    // subscire to provider 
    let order_book = SharedOrderBook::new(config.exchange.depth_limit.unwrap());

    _ = providers::deribit::subscribe_to_order_book(order_book.clone(), &config.exchange).await;
}