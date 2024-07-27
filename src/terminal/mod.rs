pub mod ui;

use std::io;

use crossterm::{event::EnableMouseCapture, execute, terminal::{self, EnterAlternateScreen}};
use ratatui::{prelude::CrosstermBackend, Terminal};
use tokio::task;
use ui::Ui;

use crate::core::SharedOrderBook;

pub fn init_terminal(order_book: SharedOrderBook) -> std::result::Result<tokio::task::JoinHandle<()>, std::boxed::Box<(dyn std::error::Error + 'static)>> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // spawn UI task
    let ui_order_book = order_book.clone();
    let handler = task::spawn(async move {
        let ui = Ui::new(ui_order_book);
        ui.run(&mut terminal).await.unwrap();
    });

    Ok(handler)
}