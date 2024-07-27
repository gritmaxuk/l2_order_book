# L2 Order Book

## Overview 

This project implements a Level 2 (L2) order book that streams real-time market data from a specified exchange using WebSocket connections. The order book maintains the best bid and offer prices and enforces a configurable depth limit. The project includes command-line interface (CLI) support for configuration, logging for activity tracking, and comprehensive unit and integration tests to ensure correctness.

The features tests for: 
- `Deribit` provider : `BTC-PERPETUAL` instrument. For `Bitstamp` not available for public. 
- `Bitstamp` provider : `BTC-USD` instrument. For `Deribit` not available for public. 

### Plan 

* Libraries
  * Use a websocket library like tungstenite to establish the connection.
  * Consider using a serde implementation for easily deserializing JSON messages.
  * Explore data structures like HashMap or custom structs to store order book data efficiently.
* Functionality
  * Connect to the websocket API endpoint of your choice
  * Allow configuration of the instrument symbol (e.g., "BTC-PERPETUAL") for which to subscribe to L2 order book updates (e.g., through command-line arguments or a configuration file or environment variables).
  * Subscribe to the appropriate channel for L2 order book updates based on the chosen instrument.
  * Process incoming messages related to L2 order book updates
  * Parse and update the in-memory order book structure accordingly (add, remove, update orders).
  * Provide access to the current state of the in-memory order book.
  * Gracefully handle disconnections and errors.
* Bonus points:
  * Implement functionality to track best bid and offer prices from the order book.
  * Add options for specifying the maximum order book depth to store.
* Testing
  * Write unit tests for the order book update logic and data structures.
  * Implement a simple command-line tool to demonstrate the adapter's usage, showcasing configuration, subscription, order book updates, and access to the order book state.

### Completed 

* Work with Deribit and Bitstamp 
* Deribit implementation uses external library to setup communication 
* Bitstamp implementation uses own implementation to connect with the provider 

## Notes 

### Used materials

`Direbit` documentation.
https://docs.deribit.com/#book-instrument_name-group-depth-interval

Channels config from Bitstamp. There a list of publically supported instruments.
https://www.bitstamp.net/websocket/v2/ --> Chanells 


https://assets.bitstamp.net/static/webapp/examples/order_book_v2.3610acefe104f01a8dcd4fda1cb88403f368526b.html

## Features

- Real-time market data streaming using WebSockets.
- Configurable depth limit for the order book.
- Configurable provider.
- Command-line interface for instrument configuration.
- Logging for activity and error tracking.
- Unit tests.
- Fancy UI console output with Ratatui lib

## Configuration

### Available values 

- `depth level` may have values `[1, 10, 20]` . That's beacuse Direbit support these values only.
- `provider` may be `[Direbit, Bitstamp]`

### config.toml

Ensure you have a `config.toml` file in the root directory with the following structure:
The file will be read first when initiate data from the configuration.

```toml
[exchange]
depth_limit = 20
instrument = "BTC-USD"

[provider]
name = "Bitstamp"
```

### Env variables 

The `Env` overrides the parameters from file.

```
EXCHANGE_DEPTH_LIMIT=10
EXCHANGE_INSTRUMENT=BTC-USD
PROVIDER_NAME=Bitstamp
```

### Command-Line Interface

CLI args override all other parameters.

```rust
    Command::new("L2 Order Book CLI")
        .version("1.0")
        .about("Command line interface for the L2 Order Book project")
        .arg(
            Arg::new("depth_limit")
                .short('d')
                .long("depth_limit")
                .value_name("DEPTH_LIMIT")
                .help("Specifies the depth limit for the order book")
                .required(false),
        )
        .arg(
            Arg::new("instrument")
                .short('i')
                .long("instrument")
                .value_name("INSTRUMENT")
                .help("Specifies the trading instrument")
                .required(false),
        )
        .arg(
            Arg::new("provider")
                .short('p')
                .long("provider")
                .value_name("PROVIDER")
                .help("Specifies the provider name")
                .required(false),
        )
```

#### Usage

The CLI allows you to specify the trading instrument. Here’s how you can use the CLI:

##### Examples to show client UI 

Run default. It takes parameters from `config.toml`.

```sh
cargo run
```

Override the instrument.

```sh
cargo run -- --instrument <INSTRUMENT>
```

Override provider.

```sh
cargo run -- --instrument BTC-USD --provider=Bitstamp
```

##### Debug Mode 

When setup `RUST_LOG` no UI expected but stream of log data.

```sh
RUST_LOG=info cargo run -- --instrument BTC-USD
```


## Running the Process

### Prerequisites

- Rust and Cargo installed on your system.

### Steps

1. Clone the repository:
   ```sh
   git clone https://github.com/your-username/l2_order_book.git
   cd l2_order_book
   ```

2. Ensure the `config.toml` file is in the root directory with the correct configuration.

3. Build the project:
   ```sh
   cargo build
   ```

4. Run the project with the specified instrument:
   ```sh
   cargo run -- --instrument BTC-USD
   ```

### Logging

Logs will be output to the console. You can adjust the logging level by setting the `RUST_LOG` environment variable:

```sh
RUST_LOG=info cargo run -- --instrument BTC-PERPETUAL
```

## Testing

### Unit Tests

To run the unit tests, use the following command:

```sh
cargo test
```

## Implementation Details

### Order Book

- The `OrderBook` struct maintains the state of the order book, including bids, asks, best bid, and best ask prices.
- The order book enforces a depth limit to maintain only the top N bids and asks.
- The `SharedOrderBook` struct provides a thread-safe wrapper around the `OrderBook` using `RwLock`.

### WebSocket Connection

- The WebSocket connection is managed using the `tokio-tungstenite` crate.
- Incoming messages are parsed and processed to update the order book in real-time.

### Logging

- The application uses the `log` and `env_logger` crates for logging.
- Logs provide detailed information about the application's activity and errors.

### Testing

- Unit tests ensure the correctness of individual components like `OrderBook` and `SharedOrderBook`.