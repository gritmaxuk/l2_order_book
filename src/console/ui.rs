use std::time::Duration;

use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Borders, Cell, Row, Table};
use ratatui::{Frame, Terminal};

use tokio::time;

use crate::core::SharedOrderBook;

pub struct Ui {
    order_book: SharedOrderBook,
}

impl Ui {
    pub fn new(order_book: SharedOrderBook) -> Self {
        Self { order_book}
    }

    pub async fn run<B: Backend>(
        &self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let best_bid = self.order_book.get_best_bid().await.unwrap_or_default();
            let best_ask = self.order_book.get_best_ask().await.unwrap_or_default();
            let bids = self.order_book.get_bids().await.unwrap_or_default();
            let asks = self.order_book.get_asks().await.unwrap_or_default();

            terminal.draw(|f| {
                self.draw_ui(f, best_bid, best_ask, bids, asks);
            })?;

            time::sleep(Duration::from_secs(1)).await; // update UI every second
        }
    }

    fn draw_ui(&self, f: &mut Frame, best_bid: f64, best_ask: f64, bids: Vec<f64>, asks: Vec<f64>) {
        // Layout for the UI
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(1)].as_ref())
            .split(f.size());
    
        // Best prices row
        let best_prices_row = Row::new(vec![
            Cell::from(format!("{}", best_ask)).style(Style::default().fg(Color::Green)),
            Cell::from(format!("{}", best_bid)).style(Style::default().fg(Color::Red)),
        ]);
    
        // Best prices table
        let best_prices_table = Table::new(
            vec![best_prices_row],
            vec![Constraint::Percentage(20), Constraint::Percentage(20)],
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Best Prices"),
        );
    
        // Order book rows
        let mut rows = vec![];
        let len = bids.len().max(asks.len());
        for i in 0..len {
            let bid = bids.get(i).cloned().unwrap_or(0.0);
            let ask = asks.get(i).cloned().unwrap_or(0.0);
            rows.push(Row::new(vec![
                Cell::from(format!("{}", ask)).style(Style::default().fg(Color::Blue)),
                Cell::from(format!("{}", bid)).style(Style::default().fg(Color::Blue)),
            ]));
        }
    
        // Order book table
        let order_book_table = Table::new(
            rows,
            vec![Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .header(Row::new(vec![
            Cell::from("Ask Price"),
            Cell::from("Bid Price"),
        ]))
        .block(Block::default().borders(Borders::ALL).title("All Order Book Prices"))
        .widths([Constraint::Percentage(20), Constraint::Percentage(20)]);
    
        // Exit info row
        let exit_info_row = Row::new(vec![Cell::from("Press Esc or q to exit").style(Style::default().italic())]);
    
        // Exit info table
        let exit_info_table = Table::new(vec![exit_info_row], vec![Constraint::Percentage(100)])
            .block(Block::default().borders(Borders::NONE));
    
        // Render the tables
        f.render_widget(best_prices_table, chunks[0]);
        f.render_widget(order_book_table, chunks[1]);
        f.render_widget(exit_info_table, chunks[2]);
    }    
}
