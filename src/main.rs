use tokio::runtime::Runtime;
use l2_order_book::cli::get_matches;
use l2_order_book::utils::config::Config;
use l2_order_book::network::websocket::connect;
use l2_order_book::core::SharedOrderBook;

fn main() {
    let matches = get_matches();

    if let Some(instrument) = matches.get_one::<String>("instrument") {
        println!("Instrument specified: {}", instrument);
        
        let config = Config::from_file("config.toml");
        let url = config.exchange.url;

        let rt = Runtime::new().unwrap();
        let order_book = SharedOrderBook::new();

        rt.block_on(async {
            match connect(&url, order_book).await {
                Ok(_) => println!("Connection successful"),
                Err(e) => eprintln!("Connection error: {:?}", e),
            }
        });
    } else {
        eprintln!("Instrument not specified!");
    }
}