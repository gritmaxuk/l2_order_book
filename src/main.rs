use l2_order_book::core::SharedOrderBook;
use l2_order_book::providers;
use l2_order_book::utils::config::{Config, Provider};

#[tokio::main]
async fn main() {
    // config
    let config = Config::read_config();
    config.validate();

    // logger
    env_logger::init();
    println!("Configuration: {:?}", config);

    // choose provider
    let provider = config.provider.name.unwrap_or(Provider::None);

    // create shared order book
    let order_book = SharedOrderBook::new(config.exchange.depth_limit.unwrap());

    match provider {
        Provider::Deribit => {
            if let Err(e) = providers::deribit::subscribe_to_order_book(order_book.clone(), &config.exchange).await {
                eprintln!("Error subscribing to Deribit: {:?}", e);
            }
        },
        Provider::Bitstamp => {
            if let Err(e) = providers::bitstamp::subscribe_to_order_book(order_book.clone(), &config.exchange).await {
                eprintln!("Error subscribing to Bitstamp: {:?}", e);
            }
        },
        _ => {
            panic!(
                "Unsupported provider. Only Deribit and Bitstamp are supported. Provided: {:?}",
                provider
            )
        }
    }
}