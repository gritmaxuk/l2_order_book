# L2 Order Book

// todo: update

https://docs.deribit.com/#book-instrument_name-group-depth-interval
https://www.bitstamp.net/websocket/v2/ --> Chanells 
https://assets.bitstamp.net/static/webapp/examples/order_book_v2.3610acefe104f01a8dcd4fda1cb88403f368526b.html

deribit -> use package connnect and disconnect gracefully 
bitstamp -> custom WS implementation example 

//

## Project Description

This project implements a Level 2 (L2) order book that streams real-time market data from a specified exchange using WebSocket connections. The order book maintains the best bid and offer prices and enforces a configurable depth limit. The project includes command-line interface (CLI) support for configuration, logging for activity tracking, and comprehensive unit and integration tests to ensure correctness.

## Features

- Real-time market data streaming using WebSockets.
- Configurable depth limit for the order book.
- Command-line interface for instrument configuration.
- Logging for activity and error tracking.
- Unit and integration tests.

## Configuration

### config.toml

Ensure you have a `config.toml` file in the root directory with the following structure:

```toml
[exchange]
url = "wss://your-selected-exchange.com/websocket"
depth_limit = 10
```

## Command-Line Interface

### Usage

The CLI allows you to specify the trading instrument. Here’s how you can use the CLI:

```sh
cargo run -- --instrument <INSTRUMENT>
```

### Example

```sh
cargo run -- --instrument BTC-USD
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
RUST_LOG=info cargo run -- --instrument BTC-USD
```

## Testing

### Unit Tests

To run the unit tests, use the following command:

```sh
cargo test
```

### Integration Tests

Integration tests are located in the `tests` directory. To run the integration tests, use the following command:

```sh
cargo test --test integration_tests
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
- Integration tests verify that the different parts of the application work together correctly.