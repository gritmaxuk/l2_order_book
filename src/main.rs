use l2_order_book::core::SharedOrderBook;
use l2_order_book::{providers, console};
use l2_order_book::utils::config::{Config, Provider};
use tokio::sync::mpsc;
use tokio::task;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

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

    // Create a shutdown channel
    let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);

    // console UI
    let ui_order_book = order_book.clone();
    let ui_handler = console::init_terminal(ui_order_book, shutdown_tx.clone())?;

    // logic
    let logic_handler = task::spawn(async move { 
        match provider {
            Provider::Deribit => {
                if let Err(e) = providers::deribit::subscribe_to_order_book(
                    order_book.clone(),
                    &config.exchange,
                )
                .await
                {
                    eprintln!("Error subscribing to Deribit: {:?}", e);
                }
            }
            Provider::Bitstamp => {
                if let Err(e) =
                    providers::bitstamp::subscribe_to_order_book(order_book.clone(), &config.exchange).await
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

    // Wait for the UI to shut down
    shutdown_rx.recv().await;

    // Cancel all tasks
    ui_handler.abort();
    logic_handler.abort();

    // Wait for all tasks to complete
    let all_tasks = vec![ui_handler, logic_handler];
    for task in all_tasks {
        if let Err(e) = task.await {
            eprintln!("Error in task: {:?}", e);
        }
    }

    // Clean up terminal
    console::dispose_terminal()
}