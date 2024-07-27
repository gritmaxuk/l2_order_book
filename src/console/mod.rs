pub mod ui;

use std::{env, io};

use crossterm::{event::{EnableMouseCapture, Event, EventStream, KeyCode}, execute, terminal::{self, EnterAlternateScreen}};
use futures::StreamExt;
use ratatui::{prelude::CrosstermBackend, Terminal};
use tokio::{sync::mpsc::{self, Receiver, Sender}, task};
use ui::Ui;

use crate::core::SharedOrderBook;

pub fn setup_console_output(order_book: SharedOrderBook) -> Option<Sender<()>> {
    // check if setup debug mode
    if env::var("RUST_LOG").is_ok() {
        return None;
    }

    // init fancy UI
    let (stop_tx, stop_rx) = mpsc::channel(1);
    
    let status = init_terminal(order_book, stop_rx);
    if let Err(err) = status {
        eprintln!("Error initializing console: {:?}", err);
    }

    Some(stop_tx)
}

fn init_terminal(order_book: SharedOrderBook, mut stop_rx: Receiver<()>) -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?; 
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // spawn UI task
    task::spawn(async move {
        let ui = Ui::new(order_book);
        ui.run(&mut terminal).await.unwrap();
    });

    // spawn task to listen for stop signal
    task::spawn(async move {
        stop_rx.recv().await;
        dispose_terminal().unwrap();
    });

    Ok(())
}

fn dispose_terminal() -> Result<(), Box<dyn std::error::Error>> { 
    terminal::disable_raw_mode()?;
    execute!(std::io::stdout(), terminal::LeaveAlternateScreen)?;

    Ok(())
}

pub fn listen_user_input() -> mpsc::Receiver<()> { 
    let (shutdown_tx, shutdown_rx) = mpsc::channel(1);

    tokio::task::spawn(async move {
        let mut event_stream = EventStream::new();
        loop {
            match event_stream.next().await {
                Some(Ok(Event::Key(key))) => {
                    if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                        shutdown_tx.send(()).await.unwrap();
                    }
                }
                Some(Err(e)) => {
                    // Handle error
                    eprintln!("Error reading event: {}", e);
                }
                _ => {} // skip other events
            }
        }
    });

    shutdown_rx
}