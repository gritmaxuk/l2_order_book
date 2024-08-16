use l2_order_book::console::{listen_user_input, setup_console_output};
use l2_order_book::core::SharedOrderBook;
use l2_order_book::providers::subscribe_to_provider;
use l2_order_book::utils::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // config
    let config = Config::read_config();
    config.validate();

    // logger
    env_logger::init();
    println!("Configuration: {:?}", config);

    // create shared order book
    let order_book = SharedOrderBook::initialise(config.exchange.depth_limit.unwrap());

    // setup console and subscribe to provider events
    let ui_cancellation_tx = setup_console_output(order_book.clone());
    let subscribe_canclellation_tx = subscribe_to_provider(config, order_book);
    let mut user_key_pressed_tx = listen_user_input();

    // listen cancellation
    user_key_pressed_tx.recv().await.unwrap();

    // clean up and cancel all tasks
    if let Some(ui_cancellation_tx) = ui_cancellation_tx {
        ui_cancellation_tx.send(()).await?;
    }
    if let Some(subscribe_canclellation_tx) = subscribe_canclellation_tx {
        subscribe_canclellation_tx.send(()).await?;
    }

    Ok(())
}
